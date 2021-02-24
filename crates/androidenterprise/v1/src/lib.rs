#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This represents an enterprise admin who can manage the enterprise in the managed Google Play store."]
pub struct Administrator {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The admin's email address."]
    pub email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A token authorizing an admin to access an iframe."]
pub struct AdministratorWebToken {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque token to be passed to the Play front-end to generate an iframe."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specification for a token used to generate iframes. The token specifies what data the admin is allowed to modify and the URI the iframe is allowed to communiate with."]
pub struct AdministratorWebTokenSpec {
    #[serde(rename = "managedConfigurations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for displaying the Managed Configuration page."]
    pub managed_configurations:
        ::std::option::Option<::std::boxed::Box<AdministratorWebTokenSpecManagedConfigurations>>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the parent frame hosting the iframe. To prevent XSS, the iframe may not be hosted at other URIs. This URI must be https. Use whitespaces to separate multiple parent URIs."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Use PlaySearch.approveApps."]
    pub permission: ::std::option::Option<::std::vec::Vec<AdministratorWebTokenSpecPermissionEnum>>,
    #[serde(rename = "playSearch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for displaying the managed Play Search apps page."]
    pub play_search: ::std::option::Option<::std::boxed::Box<AdministratorWebTokenSpecPlaySearch>>,
    #[serde(rename = "privateApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for displaying the Private Apps page."]
    pub private_apps:
        ::std::option::Option<::std::boxed::Box<AdministratorWebTokenSpecPrivateApps>>,
    #[serde(rename = "storeBuilder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for displaying the Organize apps page."]
    pub store_builder:
        ::std::option::Option<::std::boxed::Box<AdministratorWebTokenSpecStoreBuilder>>,
    #[serde(rename = "webApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for displaying the Web Apps page."]
    pub web_apps: ::std::option::Option<::std::boxed::Box<AdministratorWebTokenSpecWebApps>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AdministratorWebTokenSpecPermissionEnum {
    #[serde(rename = "unknown")]
    #[doc = "Unknown permission."]
    Unknown,
    #[serde(rename = "approveApps")]
    #[doc = "Permission to approve and unapprove apps."]
    ApproveApps,
    #[serde(rename = "manageMcm")]
    #[doc = "Permission to manage app restrictions."]
    ManageMcm,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdministratorWebTokenSpecManagedConfigurations {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Managed Configuration page is displayed. Default is true."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdministratorWebTokenSpecPlaySearch {
    #[serde(rename = "approveApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allow access to the iframe in approve mode. Default is false."]
    pub approve_apps: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the managed Play Search apps page is displayed. Default is true."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdministratorWebTokenSpecPrivateApps {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Private Apps page is displayed. Default is true."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdministratorWebTokenSpecStoreBuilder {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Organize apps page is displayed. Default is true."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdministratorWebTokenSpecWebApps {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Web Apps page is displayed. Default is true."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the list of app restrictions available to be pre-configured for the product."]
pub struct AppRestrictionsSchema {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of restrictions that make up this schema."]
    pub restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppRestrictionsSchemaRestriction>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a new app version is uploaded to Google Play and its app restrictions schema changed. To fetch the app restrictions schema for an app, use Products.getAppRestrictionsSchema on the EMM API."]
pub struct AppRestrictionsSchemaChangeEvent {
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") for which the app restriction schema changed. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A restriction in the App Restriction Schema represents a piece of configuration that may be pre-applied."]
pub struct AppRestrictionsSchemaRestriction {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value of the restriction. bundle and bundleArray restrictions never have a default value."]
    pub default_value:
        ::std::option::Option<::std::boxed::Box<AppRestrictionsSchemaRestrictionRestrictionValue>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A longer description of the restriction, giving more detail of what it affects."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For choice or multiselect restrictions, the list of possible entries' human-readable names."]
    pub entry: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "entryValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For choice or multiselect restrictions, the list of possible entries' machine-readable values. These values should be used in the configuration, either as a single string value for a choice restriction or in a stringArray for a multiselect restriction."]
    pub entry_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique key that the product uses to identify the restriction, e.g. \"com.google.android.gm.fieldname\"."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nestedRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For bundle or bundleArray restrictions, the list of nested restrictions. A bundle restriction is always nested within a bundleArray restriction, and a bundleArray restriction is at most two levels deep."]
    pub nested_restriction:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppRestrictionsSchemaRestriction>>>,
    #[serde(rename = "restrictionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the restriction."]
    pub restriction_type:
        ::std::option::Option<AppRestrictionsSchemaRestrictionRestrictionTypeEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the restriction."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the restriction."]
pub enum AppRestrictionsSchemaRestrictionRestrictionTypeEnum {
    #[serde(rename = "bool")]
    #[doc = "A restriction of boolean type."]
    Bool,
    #[serde(rename = "string")]
    #[doc = "A restriction of string type."]
    String,
    #[serde(rename = "integer")]
    #[doc = "A restriction of integer type."]
    Integer,
    #[serde(rename = "choice")]
    #[doc = "A choice of one item from a set."]
    Choice,
    #[serde(rename = "multiselect")]
    #[doc = "A choice of multiple items from a set."]
    Multiselect,
    #[serde(rename = "hidden")]
    #[doc = "A hidden restriction of string type (the default value can be used to pass along information that cannot be modified, such as a version code)."]
    Hidden,
    #[serde(rename = "bundle")]
    #[doc = "[M+ devices only] A bundle of restrictions"]
    Bundle,
    #[serde(rename = "bundleArray")]
    #[doc = "[M+ devices only] An array of restriction bundles"]
    BundleArray,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A typed value for the restriction."]
pub struct AppRestrictionsSchemaRestrictionRestrictionValue {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the value being provided."]
    pub _type: ::std::option::Option<AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum>,
    #[serde(rename = "valueBool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The boolean value - this will only be present if type is bool."]
    pub value_bool: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "valueInteger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The integer value - this will only be present if type is integer."]
    pub value_integer: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "valueMultiselect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of string values - this will only be present if type is multiselect."]
    pub value_multiselect: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "valueString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value - this will be present for types string, choice and hidden."]
    pub value_string: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the value being provided."]
pub enum AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum {
    #[serde(rename = "bool")]
    #[doc = "A restriction of boolean type."]
    Bool,
    #[serde(rename = "string")]
    #[doc = "A restriction of string type."]
    String,
    #[serde(rename = "integer")]
    #[doc = "A restriction of integer type."]
    Integer,
    #[serde(rename = "choice")]
    #[doc = "A choice of one item from a set."]
    Choice,
    #[serde(rename = "multiselect")]
    #[doc = "A choice of multiple items from a set."]
    Multiselect,
    #[serde(rename = "hidden")]
    #[doc = "A hidden restriction of string type (the default value can be used to pass along information that cannot be modified, such as a version code)."]
    Hidden,
    #[serde(rename = "bundle")]
    #[doc = "[M+ devices only] A bundle of restrictions"]
    Bundle,
    #[serde(rename = "bundleArray")]
    #[doc = "[M+ devices only] An array of restriction bundles"]
    BundleArray,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of states set by the app."]
pub struct AppState {
    #[serde(rename = "keyedAppState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of keyed app states. This field will always be present."]
    pub keyed_app_state: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyedAppState>>>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The package name of the app. This field will always be present."]
    pub package_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a new version of an app is uploaded to Google Play. Notifications are sent for new public versions only: alpha, beta, or canary versions do not generate this event. To fetch up-to-date version history for an app, use Products.Get on the EMM API."]
pub struct AppUpdateEvent {
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") that was updated. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This represents a single version of the app."]
pub struct AppVersion {
    #[serde(rename = "isProduction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if this version is a production APK."]
    pub is_production: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "track")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated, use trackId instead."]
    pub track: ::std::option::Option<AppVersionTrackEnum>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Track ids that the app version is published in. Replaces the track field (deprecated), but doesn't include the production track (see isProduction instead)."]
    pub track_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique increasing identifier for the app version."]
    pub version_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "versionString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string used in the Play store by the app developer to identify the version. The string is not necessarily unique or localized (for example, the string could be \"1.4\")."]
    pub version_string: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Deprecated, use trackId instead."]
pub enum AppVersionTrackEnum {
    #[serde(rename = "appTrackUnspecified")]
    #[doc = ""]
    AppTrackUnspecified,
    #[serde(rename = "production")]
    #[doc = ""]
    Production,
    #[serde(rename = "beta")]
    #[doc = ""]
    Beta,
    #[serde(rename = "alpha")]
    #[doc = ""]
    Alpha,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information on an approval URL."]
pub struct ApprovalUrlInfo {
    #[serde(rename = "approvalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL that displays a product's permissions and that can also be used to approve the product with the Products.approve call."]
    pub approval_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An AuthenticationToken is used by the EMM's device policy client on a device to provision the given EMM-managed user on that device."]
pub struct AuthenticationToken {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The authentication token to be passed to the device policy client on the device where it can be used to provision the account for which this token was generated."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The auto-install constraint. Defines a set of restrictions for installation. At least one of the fields must be set."]
pub struct AutoInstallConstraint {
    #[serde(rename = "chargingStateConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Charging state constraint."]
    pub charging_state_constraint:
        ::std::option::Option<AutoInstallConstraintChargingStateConstraintEnum>,
    #[serde(rename = "deviceIdleStateConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device idle state constraint."]
    pub device_idle_state_constraint:
        ::std::option::Option<AutoInstallConstraintDeviceIdleStateConstraintEnum>,
    #[serde(rename = "networkTypeConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Network type constraint."]
    pub network_type_constraint:
        ::std::option::Option<AutoInstallConstraintNetworkTypeConstraintEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Charging state constraint."]
pub enum AutoInstallConstraintChargingStateConstraintEnum {
    #[serde(rename = "chargingStateConstraintUnspecified")]
    #[doc = ""]
    ChargingStateConstraintUnspecified,
    #[serde(rename = "chargingNotRequired")]
    #[doc = "Device doesn't have to be charging."]
    ChargingNotRequired,
    #[serde(rename = "chargingRequired")]
    #[doc = "Device has to be charging."]
    ChargingRequired,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Device idle state constraint."]
pub enum AutoInstallConstraintDeviceIdleStateConstraintEnum {
    #[serde(rename = "deviceIdleStateConstraintUnspecified")]
    #[doc = ""]
    DeviceIdleStateConstraintUnspecified,
    #[serde(rename = "deviceIdleNotRequired")]
    #[doc = "Device doesn't have to be idle, app can be installed while the user is interacting with the device."]
    DeviceIdleNotRequired,
    #[serde(rename = "deviceIdleRequired")]
    #[doc = "Device has to be idle."]
    DeviceIdleRequired,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Network type constraint."]
pub enum AutoInstallConstraintNetworkTypeConstraintEnum {
    #[serde(rename = "networkTypeConstraintUnspecified")]
    #[doc = ""]
    NetworkTypeConstraintUnspecified,
    #[serde(rename = "anyNetwork")]
    #[doc = "Any active networks (Wi-Fi, cellular, etc.)."]
    AnyNetwork,
    #[serde(rename = "unmeteredNetwork")]
    #[doc = "Any unmetered network (e.g. Wi-FI)."]
    UnmeteredNetwork,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutoInstallPolicy {
    #[serde(rename = "autoInstallConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The constraints for auto-installing the app. You can specify a maximum of one constraint."]
    pub auto_install_constraint:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AutoInstallConstraint>>>,
    #[serde(rename = "autoInstallMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-install mode. If unset defaults to \"doNotAutoInstall\"."]
    pub auto_install_mode: ::std::option::Option<AutoInstallPolicyAutoInstallModeEnum>,
    #[serde(rename = "autoInstallPriority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The priority of the install, as an unsigned integer. A lower number means higher priority."]
    pub auto_install_priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minimumVersionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum version of the app. If a lower version of the app is installed, then the app will be auto-updated according to the auto-install constraints, instead of waiting for the regular auto-update. You can set a minimum version code for at most 20 apps per device."]
    pub minimum_version_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The auto-install mode. If unset defaults to \"doNotAutoInstall\"."]
pub enum AutoInstallPolicyAutoInstallModeEnum {
    #[serde(rename = "autoInstallModeUnspecified")]
    #[doc = ""]
    AutoInstallModeUnspecified,
    #[serde(rename = "doNotAutoInstall")]
    #[doc = "The product is not installed automatically, the user needs to install it from the Play Store."]
    DoNotAutoInstall,
    #[serde(rename = "autoInstallOnce")]
    #[doc = "The product is automatically installed once, if the user uninstalls the product it will not be installed again."]
    AutoInstallOnce,
    #[serde(rename = "forceAutoInstall")]
    #[doc = "The product is automatically installed, if the user uninstalls the product it will be installed again. On managed devices the DPC should block uninstall."]
    ForceAutoInstall,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configuration variables resource contains the managed configuration settings ID to be applied to a single user, as well as the variable set that is attributed to the user. The variable set will be used to replace placeholders in the managed configuration settings."]
pub struct ConfigurationVariables {
    #[serde(rename = "mcmId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managed configurations settings."]
    pub mcm_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The variable set that is attributed to the user."]
    pub variable_set: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VariableSet>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Devices resource represents a mobile device managed by the EMM and belonging to a specific enterprise user."]
pub struct Device {
    #[serde(rename = "androidId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Play Services Android ID for the device encoded as a lowercase hex string. For example, \"123456789abcdef0\"."]
    pub android_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - \"managedDevice\", a device that has the EMM's device policy controller (DPC) as the device owner. - \"managedProfile\", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - \"containerApp\", no longer used (deprecated). - \"unmanagedProfile\", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC. "]
    pub management_type: ::std::option::Option<DeviceManagementTypeEnum>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The policy enforced on the device."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "report")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device report updated with the latest app states."]
    pub report: ::std::option::Option<::std::boxed::Box<DeviceReport>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - \"managedDevice\", a device that has the EMM's device policy controller (DPC) as the device owner. - \"managedProfile\", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - \"containerApp\", no longer used (deprecated). - \"unmanagedProfile\", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC. "]
pub enum DeviceManagementTypeEnum {
    #[serde(rename = "managedDevice")]
    #[doc = ""]
    ManagedDevice,
    #[serde(rename = "managedProfile")]
    #[doc = ""]
    ManagedProfile,
    #[serde(rename = "containerApp")]
    #[doc = ""]
    ContainerApp,
    #[serde(rename = "unmanagedProfile")]
    #[doc = ""]
    UnmanagedProfile,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device report updated with the latest app states for managed apps on the device."]
pub struct DeviceReport {
    #[serde(rename = "appState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of app states set by managed apps on the device. App states are defined by the app's developers. This field will always be present."]
    pub app_state: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppState>>>,
    #[serde(rename = "lastUpdatedTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the last report update in milliseconds since epoch. This field will always be present."]
    pub last_updated_timestamp_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when an updated device report is available."]
pub struct DeviceReportUpdateEvent {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Android ID of the device. This field will always be present."]
    pub device_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "report")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The device report updated with the latest app states. This field will always be present."]
    pub report: ::std::option::Option<::std::boxed::Box<DeviceReport>>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the user. This field will always be present."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The state of a user's device, as accessed by the getState and setState methods on device resources."]
pub struct DeviceState {
    #[serde(rename = "accountState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the Google account on the device. \"enabled\" indicates that the Google account on the device can be used to access Google services (including Google Play), while \"disabled\" means that it cannot. A new device is initially in the \"disabled\" state."]
    pub account_state: ::std::option::Option<DeviceStateAccountStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the Google account on the device. \"enabled\" indicates that the Google account on the device can be used to access Google services (including Google Play), while \"disabled\" means that it cannot. A new device is initially in the \"disabled\" state."]
pub enum DeviceStateAccountStateEnum {
    #[serde(rename = "enabled")]
    #[doc = ""]
    Enabled,
    #[serde(rename = "disabled")]
    #[doc = ""]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DevicesListResponse {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A managed device."]
    pub device: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Enterprises resource represents the binding between an EMM and a specific organization. That binding can be instantiated in one of two different ways using this API as follows: - For Google managed domain customers, the process involves using Enterprises.enroll and Enterprises.setAccount (in conjunction with artifacts obtained from the Admin console and the Google API Console) and submitted to the EMM through a more-or-less manual process. - For managed Google Play Accounts customers, the process involves using Enterprises.generateSignupUrl and Enterprises.completeSignup in conjunction with the managed Google Play sign-up UI (Google-provided mechanism) to create the binding without manual steps. As an EMM, you can support either or both approaches in your EMM console. See Create an Enterprise for details."]
pub struct Enterprise {
    #[serde(rename = "administrator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Admins of the enterprise. This is only supported for enterprises created via the EMM-initiated flow."]
    pub administrator: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Administrator>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID for the enterprise."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the enterprise, for example, \"Example, Inc\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "primaryDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The enterprise's primary domain, such as \"example.com\"."]
    pub primary_domain: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A service account that can be used to authenticate as the enterprise to API calls that require such authentication."]
pub struct EnterpriseAccount {
    #[serde(rename = "accountEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the service account."]
    pub account_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnterprisesListResponse {
    #[serde(rename = "enterprise")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An enterprise."]
    pub enterprise: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Enterprise>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnterprisesSendTestPushNotificationResponse {
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message ID of the test push notification that was sent."]
    pub message_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Cloud Pub/Sub topic to which notifications for this enterprise's enrolled account will be sent."]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The presence of an Entitlements resource indicates that a user has the right to use a particular app. Entitlements are user specific, not device specific. This allows a user with an entitlement to an app to install the app on all their devices. It's also possible for a user to hold an entitlement to an app without installing the app on any device. The API can be used to create an entitlement. As an option, you can also use the API to trigger the installation of an app on all a user's managed devices at the same time the entitlement is created. If the app is free, creating the entitlement also creates a group license for that app. For paid apps, creating the entitlement consumes one license, and that license remains consumed until the entitlement is removed. If the enterprise hasn't purchased enough licenses, then no entitlement is created and the installation fails. An entitlement is also not created for an app if the app requires permissions that the enterprise hasn't accepted. If an entitlement is deleted, the app may be uninstalled from a user's device. As a best practice, uninstall the app by calling Installs.delete() before deleting the entitlement. Entitlements for apps that a user pays for on an unmanaged profile have \"userPurchase\" as the entitlement reason. These entitlements cannot be removed via the API."]
pub struct Entitlement {
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product that the entitlement is for. For example, \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the entitlement. For example, \"free\" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses."]
    pub reason: ::std::option::Option<EntitlementReasonEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The reason for the entitlement. For example, \"free\" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses."]
pub enum EntitlementReasonEnum {
    #[serde(rename = "free")]
    #[doc = ""]
    Free,
    #[serde(rename = "groupLicense")]
    #[doc = ""]
    GroupLicense,
    #[serde(rename = "userPurchase")]
    #[doc = ""]
    UserPurchase,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntitlementsListResponse {
    #[serde(rename = "entitlement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entitlement of a user to a product (e.g. an app). For example, a free app that they have installed, or a paid app that they have been allocated a license to."]
    pub entitlement: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entitlement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Group license objects allow you to keep track of licenses (called entitlements) for both free and paid apps. For a free app, a group license is created when an enterprise admin first approves the product in Google Play or when the first entitlement for the product is created for a user via the API. For a paid app, a group license object is only created when an enterprise admin purchases the product in Google Play for the first time. Use the API to query group licenses. A Grouplicenses resource includes the total number of licenses purchased (paid apps only) and the total number of licenses currently in use. In other words, the total number of Entitlements that exist for the product. Only one group license object is created per product and group license objects are never deleted. If a product is unapproved, its group license remains. This allows enterprise admins to keep track of any remaining entitlements for the product."]
pub struct GroupLicense {
    #[serde(rename = "acquisitionKind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How this group license was acquired. \"bulkPurchase\" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is \"free\" (for free products)."]
    pub acquisition_kind: ::std::option::Option<GroupLicenseAcquisitionKindEnum>,
    #[serde(rename = "approval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created."]
    pub approval: ::std::option::Option<GroupLicenseApprovalEnum>,
    #[serde(rename = "numProvisioned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of provisioned licenses for this product. Returned by read operations, but ignored in write operations."]
    pub num_provisioned: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numPurchased")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of purchased licenses (possibly in multiple purchases). If this field is omitted, then there is no limit on the number of licenses that can be provisioned (for example, if the acquisition kind is \"free\")."]
    pub num_purchased: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission approval status of the product. This field is only set if the product is approved. Possible states are: - \"currentApproved\", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - \"needsReapproval\", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - \"allCurrentAndFutureApproved\", the current permissions are approved and any future permission updates will be automatically approved without administrator review. "]
    pub permissions: ::std::option::Option<GroupLicensePermissionsEnum>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product that the license is for. For example, \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How this group license was acquired. \"bulkPurchase\" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is \"free\" (for free products)."]
pub enum GroupLicenseAcquisitionKindEnum {
    #[serde(rename = "free")]
    #[doc = ""]
    Free,
    #[serde(rename = "bulkPurchase")]
    #[doc = ""]
    BulkPurchase,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created."]
pub enum GroupLicenseApprovalEnum {
    #[serde(rename = "approved")]
    #[doc = ""]
    Approved,
    #[serde(rename = "unapproved")]
    #[doc = ""]
    Unapproved,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The permission approval status of the product. This field is only set if the product is approved. Possible states are: - \"currentApproved\", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - \"needsReapproval\", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - \"allCurrentAndFutureApproved\", the current permissions are approved and any future permission updates will be automatically approved without administrator review. "]
pub enum GroupLicensePermissionsEnum {
    #[serde(rename = "currentApproved")]
    #[doc = ""]
    CurrentApproved,
    #[serde(rename = "needsReapproval")]
    #[doc = ""]
    NeedsReapproval,
    #[serde(rename = "allCurrentAndFutureApproved")]
    #[doc = ""]
    AllCurrentAndFutureApproved,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupLicenseUsersListResponse {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A user of an enterprise."]
    pub user: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupLicensesListResponse {
    #[serde(rename = "groupLicense")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A group license for a product approved for use in the enterprise."]
    pub group_license: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GroupLicense>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The existence of an Installs resource indicates that an app is installed on a particular device (or that an install is pending). The API can be used to create an install resource using the update method. This triggers the actual install of the app on the device. If the user does not already have an entitlement for the app, then an attempt is made to create one. If this fails (for example, because the app is not free and there is no available license), then the creation of the install fails. The API can also be used to update an installed app. If the update method is used on an existing install, then the app will be updated to the latest available version. Note that it is not possible to force the installation of a specific version of an app: the version code is read-only. If a user installs an app themselves (as permitted by the enterprise), then again an install resource and possibly an entitlement resource are automatically created. The API can also be used to delete an install resource, which triggers the removal of the app from the device. Note that deleting an install does not automatically remove the corresponding entitlement, even if there are no remaining installs. The install resource will also be deleted if the user uninstalls the app themselves."]
pub struct Install {
    #[serde(rename = "installState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Install state. The state \"installPending\" means that an install request has recently been made and download to the device is in progress. The state \"installed\" means that the app has been installed. This field is read-only."]
    pub install_state: ::std::option::Option<InstallInstallStateEnum>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product that the install is for. For example, \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the installed product. Guaranteed to be set only if the install state is \"installed\"."]
    pub version_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Install state. The state \"installPending\" means that an install request has recently been made and download to the device is in progress. The state \"installed\" means that the app has been installed. This field is read-only."]
pub enum InstallInstallStateEnum {
    #[serde(rename = "installed")]
    #[doc = ""]
    Installed,
    #[serde(rename = "installPending")]
    #[doc = ""]
    InstallPending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when an app installation failed on a device"]
pub struct InstallFailureEvent {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Android ID of the device. This field will always be present."]
    pub device_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional details on the failure if applicable."]
    pub failure_details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the installation failure. This field will always be present."]
    pub failure_reason: ::std::option::Option<InstallFailureEventFailureReasonEnum>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") for which the install failure event occured. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the user. This field will always be present."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The reason for the installation failure. This field will always be present."]
pub enum InstallFailureEventFailureReasonEnum {
    #[serde(rename = "unknown")]
    #[doc = "Used whenever no better reason for failure can be provided."]
    Unknown,
    #[serde(rename = "timeout")]
    #[doc = "Used when the installation timed out. This can cover a number of situations, for example when the device did not have connectivity at any point during the retry period, or if the device is OOM."]
    Timeout,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallsListResponse {
    #[serde(rename = "install")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An installation of an app for a user on a specific device. The existence of an install implies that the user must have an entitlement to the app."]
    pub install: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Install>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a keyed app state containing a key, timestamp, severity level, optional description, and optional data."]
pub struct KeyedAppState {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional field intended for machine-readable data. For example, a number or JSON object. To prevent XSS, we recommend removing any HTML from the data before displaying it."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key indicating what the app is providing a state for. The content of the key is set by the app's developer. To prevent XSS, we recommend removing any HTML from the key before displaying it. This field will always be present."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form, human-readable message describing the app state. For example, an error message. To prevent XSS, we recommend removing any HTML from the message before displaying it."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Severity of the app state. This field will always be present."]
    pub severity: ::std::option::Option<KeyedAppStateSeverityEnum>,
    #[serde(rename = "stateTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of when the app set the state in milliseconds since epoch. This field will always be present."]
    pub state_timestamp_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Severity of the app state. This field will always be present."]
pub enum KeyedAppStateSeverityEnum {
    #[serde(rename = "severityUnknown")]
    #[doc = ""]
    SeverityUnknown,
    #[serde(rename = "severityInfo")]
    #[doc = ""]
    SeverityInfo,
    #[serde(rename = "severityError")]
    #[doc = ""]
    SeverityError,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A localized string with its locale."]
pub struct LocalizedText {
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP47 tag for a locale. (e.g. \"en-US\", \"de\")."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The text localized in the associated locale."]
    pub text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Maintenance window for managed Google Play Accounts. This allows Play store to update the apps on the foreground in the designated window."]
pub struct MaintenanceWindow {
    #[serde(rename = "durationMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the maintenance window, in milliseconds. The duration must be between 30 minutes and 24 hours (inclusive)."]
    pub duration_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeAfterMidnightMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time of the maintenance window, in milliseconds after midnight on the device. Windows can span midnight."]
    pub start_time_after_midnight_ms: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A managed configuration resource contains the set of managed properties defined by the app developer in the app's managed configurations schema, as well as any configuration variables defined for the user."]
pub struct ManagedConfiguration {
    #[serde(rename = "configurationVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user."]
    pub configuration_variables: ::std::option::Option<::std::boxed::Box<ConfigurationVariables>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managedProperty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of managed properties for this configuration."]
    pub managed_property:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedProperty>>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product that the managed configuration is for, e.g. \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedConfigurationsForDeviceListResponse {
    #[serde(rename = "managedConfigurationForDevice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A managed configuration for an app on a specific device."]
    pub managed_configuration_for_device:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedConfiguration>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedConfigurationsForUserListResponse {
    #[serde(rename = "managedConfigurationForUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A managed configuration for an app for a specific user."]
    pub managed_configuration_for_user:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedConfiguration>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A managed configurations settings resource contains the set of managed properties that have been configured for an Android app to be applied to a set of users. The app's developer would have defined configurable properties in the managed configurations schema."]
pub struct ManagedConfigurationsSettings {
    #[serde(rename = "lastUpdatedTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last updated time of the managed configuration settings in milliseconds since 1970-01-01T00:00:00Z."]
    pub last_updated_timestamp_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mcmId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managed configurations settings."]
    pub mcm_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the managed configurations settings."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedConfigurationsSettingsListResponse {
    #[serde(rename = "managedConfigurationsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A managed configurations settings for an app that may be assigned to a group of users in an enterprise."]
    pub managed_configurations_settings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedConfigurationsSettings>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A managed property of a managed configuration. The property must match one of the properties in the app restrictions schema of the product. Exactly one of the value fields must be populated, and it must match the property's type in the app restrictions schema."]
pub struct ManagedProperty {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique key that identifies the property."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueBool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The boolean value - this will only be present if type of the property is bool."]
    pub value_bool: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "valueBundle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bundle of managed properties - this will only be present if type of the property is bundle."]
    pub value_bundle: ::std::option::Option<::std::boxed::Box<ManagedPropertyBundle>>,
    #[serde(rename = "valueBundleArray")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of bundles of properties - this will only be present if type of the property is bundle_array."]
    pub value_bundle_array:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedPropertyBundle>>>,
    #[serde(rename = "valueInteger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The integer value - this will only be present if type of the property is integer."]
    pub value_integer: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "valueString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value - this will only be present if type of the property is string, choice or hidden."]
    pub value_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueStringArray")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of string values - this will only be present if type of the property is multiselect."]
    pub value_string_array: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A bundle of managed properties."]
pub struct ManagedPropertyBundle {
    #[serde(rename = "managedProperty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of managed properties."]
    pub managed_property:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedProperty>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a new device is ready to be managed."]
pub struct NewDeviceEvent {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Android ID of the device. This field will always be present."]
    pub device_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dpcPackageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy app on the device."]
    pub dpc_package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the extent to which the device is controlled by an Android EMM in various deployment configurations. Possible values include: - \"managedDevice\", a device where the DPC is set as device owner, - \"managedProfile\", a device where the DPC is set as profile owner. "]
    pub management_type: ::std::option::Option<NewDeviceEventManagementTypeEnum>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the user. This field will always be present."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Identifies the extent to which the device is controlled by an Android EMM in various deployment configurations. Possible values include: - \"managedDevice\", a device where the DPC is set as device owner, - \"managedProfile\", a device where the DPC is set as profile owner. "]
pub enum NewDeviceEventManagementTypeEnum {
    #[serde(rename = "managedDevice")]
    #[doc = ""]
    ManagedDevice,
    #[serde(rename = "managedProfile")]
    #[doc = ""]
    ManagedProfile,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when new permissions are added to an app."]
pub struct NewPermissionsEvent {
    #[serde(rename = "approvedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions that the enterprise admin has already approved for this application. Use Permissions.Get on the EMM API to retrieve details about these permissions."]
    pub approved_permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") for which new permissions were added. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions that the app is currently requesting. Use Permissions.Get on the EMM API to retrieve details about these permissions."]
    pub requested_permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A notification of one event relating to an enterprise."]
pub struct Notification {
    #[serde(rename = "appRestrictionsSchemaChangeEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about new app restrictions schema changes."]
    pub app_restrictions_schema_change_event:
        ::std::option::Option<::std::boxed::Box<AppRestrictionsSchemaChangeEvent>>,
    #[serde(rename = "appUpdateEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about app updates."]
    pub app_update_event: ::std::option::Option<::std::boxed::Box<AppUpdateEvent>>,
    #[serde(rename = "deviceReportUpdateEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about device report updates."]
    pub device_report_update_event:
        ::std::option::Option<::std::boxed::Box<DeviceReportUpdateEvent>>,
    #[serde(rename = "enterpriseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the enterprise for which the notification is sent. This will always be present."]
    pub enterprise_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installFailureEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about an app installation failure."]
    pub install_failure_event: ::std::option::Option<::std::boxed::Box<InstallFailureEvent>>,
    #[serde(rename = "newDeviceEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about new devices."]
    pub new_device_event: ::std::option::Option<::std::boxed::Box<NewDeviceEvent>>,
    #[serde(rename = "newPermissionsEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about new app permissions."]
    pub new_permissions_event: ::std::option::Option<::std::boxed::Box<NewPermissionsEvent>>,
    #[serde(rename = "notificationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the notification."]
    pub notification_type: ::std::option::Option<NotificationNotificationTypeEnum>,
    #[serde(rename = "productApprovalEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about changes to a product's approval status."]
    pub product_approval_event: ::std::option::Option<::std::boxed::Box<ProductApprovalEvent>>,
    #[serde(rename = "productAvailabilityChangeEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications about product availability changes."]
    pub product_availability_change_event:
        ::std::option::Option<::std::boxed::Box<ProductAvailabilityChangeEvent>>,
    #[serde(rename = "timestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the notification was published in milliseconds since 1970-01-01T00:00:00Z. This will always be present."]
    pub timestamp_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the notification."]
pub enum NotificationNotificationTypeEnum {
    #[serde(rename = "unknown")]
    #[doc = ""]
    Unknown,
    #[serde(rename = "testNotification")]
    #[doc = "A test push notification."]
    TestNotification,
    #[serde(rename = "productApproval")]
    #[doc = "Notification about change to a product's approval status."]
    ProductApproval,
    #[serde(rename = "installFailure")]
    #[doc = "Notification about an app installation failure."]
    InstallFailure,
    #[serde(rename = "appUpdate")]
    #[doc = "Notification about app update."]
    AppUpdate,
    #[serde(rename = "newPermissions")]
    #[doc = "Notification about new app permissions."]
    NewPermissions,
    #[serde(rename = "appRestricionsSchemaChange")]
    #[doc = "Notification about new app restrictions schema change."]
    AppRestricionsSchemaChange,
    #[serde(rename = "productAvailabilityChange")]
    #[doc = "Notification about product availability change."]
    ProductAvailabilityChange,
    #[serde(rename = "newDevice")]
    #[doc = "Notification about a new device."]
    NewDevice,
    #[serde(rename = "deviceReportUpdate")]
    #[doc = "Notification about an updated device report."]
    DeviceReportUpdate,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource returned by the PullNotificationSet API, which contains a collection of notifications for enterprises associated with the service account authenticated for the request."]
pub struct NotificationSet {
    #[serde(rename = "notification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notifications received, or empty if no notifications are present."]
    pub notification: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Notification>>>,
    #[serde(rename = "notificationSetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notification set ID, required to mark the notification as received with the Enterprises.AcknowledgeNotification API. This will be omitted if no notifications are present."]
    pub notification_set_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the current page. List operations that supports paging return only one \"page\" of results. This protocol buffer message describes the page that has been returned."]
pub struct PageInfo {
    #[serde(rename = "resultPerPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of results returned in one page. ! The number of results included in the API response."]
    pub result_per_page: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Index of the first result returned in the current page."]
    pub start_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of results available on the backend ! The total number of results in the result set."]
    pub total_results: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Permissions resource represents some extra capability, to be granted to an Android app, which requires explicit consent. An enterprise admin must consent to these permissions on behalf of their users before an entitlement for the app can be created. The permissions collection is read-only. The information provided for each permission (localized name and description) is intended to be used in the MDM user interface when obtaining consent from the enterprise."]
pub struct Permission {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A longer description of the Permissions resource, giving more details of what it affects."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the permission."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque string uniquely identifying the permission."]
    pub permission_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The device policy for a given managed device."]
pub struct Policy {
    #[serde(rename = "autoUpdatePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-update policy for apps installed on the device. \"choiceToTheUser\" allows the device's user to configure the app update policy. \"always\" enables auto updates. \"never\" disables auto updates. \"wifiOnly\" enables auto updates only when the device is connected to wifi."]
    pub auto_update_policy: ::std::option::Option<PolicyAutoUpdatePolicyEnum>,
    #[serde(rename = "deviceReportPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device reports app states to the EMM. The default value is \"deviceReportDisabled\"."]
    pub device_report_policy: ::std::option::Option<PolicyDeviceReportPolicyEnum>,
    #[serde(rename = "maintenanceWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maintenance window defining when apps running in the foreground should be updated."]
    pub maintenance_window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
    #[serde(rename = "productAvailabilityPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availability granted to the device for the specified products. \"all\" gives the device access to all products, regardless of approval status. \"all\" does not enable automatic visibility of \"alpha\" or \"beta\" tracks. \"whitelist\" grants the device access the products specified in productPolicy[]. Only products that are approved or products that were previously approved (products with revoked approval) by the enterprise can be whitelisted. If no value is provided, the availability set at the user level is applied by default."]
    pub product_availability_policy: ::std::option::Option<PolicyProductAvailabilityPolicyEnum>,
    #[serde(rename = "productPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of product policies. The productAvailabilityPolicy needs to be set to WHITELIST or ALL for the product policies to be applied."]
    pub product_policy: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductPolicy>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The auto-update policy for apps installed on the device. \"choiceToTheUser\" allows the device's user to configure the app update policy. \"always\" enables auto updates. \"never\" disables auto updates. \"wifiOnly\" enables auto updates only when the device is connected to wifi."]
pub enum PolicyAutoUpdatePolicyEnum {
    #[serde(rename = "autoUpdatePolicyUnspecified")]
    #[doc = "The auto update policy is not set."]
    AutoUpdatePolicyUnspecified,
    #[serde(rename = "choiceToTheUser")]
    #[doc = "The user can control auto-updates."]
    ChoiceToTheUser,
    #[serde(rename = "never")]
    #[doc = "Apps are never auto-updated."]
    Never,
    #[serde(rename = "wifiOnly")]
    #[doc = "Apps are auto-updated over WiFi only."]
    WifiOnly,
    #[serde(rename = "always")]
    #[doc = "Apps are auto-updated at any time. Data charges may apply."]
    Always,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the device reports app states to the EMM. The default value is \"deviceReportDisabled\"."]
pub enum PolicyDeviceReportPolicyEnum {
    #[serde(rename = "deviceReportPolicyUnspecified")]
    #[doc = "The device report policy is not set."]
    DeviceReportPolicyUnspecified,
    #[serde(rename = "deviceReportDisabled")]
    #[doc = "Device reports are disabled."]
    DeviceReportDisabled,
    #[serde(rename = "deviceReportEnabled")]
    #[doc = "Device reports are enabled."]
    DeviceReportEnabled,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The availability granted to the device for the specified products. \"all\" gives the device access to all products, regardless of approval status. \"all\" does not enable automatic visibility of \"alpha\" or \"beta\" tracks. \"whitelist\" grants the device access the products specified in productPolicy[]. Only products that are approved or products that were previously approved (products with revoked approval) by the enterprise can be whitelisted. If no value is provided, the availability set at the user level is applied by default."]
pub enum PolicyProductAvailabilityPolicyEnum {
    #[serde(rename = "productAvailabilityPolicyUnspecified")]
    #[doc = "Unspecified, applies the user available product set by default."]
    ProductAvailabilityPolicyUnspecified,
    #[serde(rename = "whitelist")]
    #[doc = "The approved products with product availability set to AVAILABLE in the product policy are available."]
    Whitelist,
    #[serde(rename = "all")]
    #[doc = "All products are available except those explicitly marked as unavailable in the product availability policy."]
    All,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Products resource represents an app in the Google Play store that is available to at least some users in the enterprise. (Some apps are restricted to a single enterprise, and no information about them is made available outside that enterprise.) The information provided for each product (localized name, icon, link to the full Google Play details page) is intended to allow a basic representation of the product within an EMM user interface."]
pub struct Product {
    #[serde(rename = "appTracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracks visible to the enterprise."]
    pub app_tracks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrackInfo>>>,
    #[serde(rename = "appVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App versions currently available for this product."]
    pub app_version: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppVersion>>>,
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the author of the product (for example, the app developer)."]
    pub author_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "availableCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The countries which this app is available in."]
    pub available_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "availableTracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated, use appTracks instead."]
    pub available_tracks: ::std::option::Option<::std::vec::Vec<ProductAvailableTracksEnum>>,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The app category (e.g. RACING, SOCIAL, etc.)"]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content rating for this app."]
    pub content_rating: ::std::option::Option<ProductContentRatingEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized promotional description, if available."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detailsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the (consumer) Google Play details page for the product."]
    pub details_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "distributionChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted."]
    pub distribution_channel: ::std::option::Option<ProductDistributionChannelEnum>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Noteworthy features (if any) of this product."]
    pub features: ::std::option::Option<::std::vec::Vec<ProductFeaturesEnum>>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to an image that can be used as an icon for the product. This image is suitable for use at up to 512px x 512px."]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdatedTimestampMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximate time (within 7 days) the app was last published, expressed in milliseconds since epoch."]
    pub last_updated_timestamp_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minAndroidSdkVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum Android SDK necessary to run the app."]
    pub min_android_sdk_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of permissions required by the app."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductPermission>>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string of the form *app:<package name>*. For example, app:com.google.android.gm represents the Gmail app."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productPricing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it)."]
    pub product_pricing: ::std::option::Option<ProductProductPricingEnum>,
    #[serde(rename = "recentChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the recent changes made to the app."]
    pub recent_changes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requiresContainerApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub requires_container_app: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "screenshotUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of screenshot links representing the app."]
    pub screenshot_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "signingCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The certificate used to sign this product."]
    pub signing_certificate: ::std::option::Option<::std::boxed::Box<ProductSigningCertificate>>,
    #[serde(rename = "smallIconUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to a smaller image that can be used as an icon for the product. This image is suitable for use at up to 128px x 128px."]
    pub small_icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the product."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workDetailsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the managed Google Play details page for the product, for use by an Enterprise admin."]
    pub work_details_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProductAvailableTracksEnum {
    #[serde(rename = "appTrackUnspecified")]
    #[doc = ""]
    AppTrackUnspecified,
    #[serde(rename = "production")]
    #[doc = ""]
    Production,
    #[serde(rename = "beta")]
    #[doc = ""]
    Beta,
    #[serde(rename = "alpha")]
    #[doc = ""]
    Alpha,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The content rating for this app."]
pub enum ProductContentRatingEnum {
    #[serde(rename = "ratingUnknown")]
    #[doc = ""]
    RatingUnknown,
    #[serde(rename = "all")]
    #[doc = ""]
    All,
    #[serde(rename = "preTeen")]
    #[doc = ""]
    PreTeen,
    #[serde(rename = "teen")]
    #[doc = ""]
    Teen,
    #[serde(rename = "mature")]
    #[doc = ""]
    Mature,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted."]
pub enum ProductDistributionChannelEnum {
    #[serde(rename = "publicGoogleHosted")]
    #[doc = ""]
    PublicGoogleHosted,
    #[serde(rename = "privateGoogleHosted")]
    #[doc = ""]
    PrivateGoogleHosted,
    #[serde(rename = "privateSelfHosted")]
    #[doc = ""]
    PrivateSelfHosted,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProductFeaturesEnum {
    #[serde(rename = "featureUnknown")]
    #[doc = ""]
    FeatureUnknown,
    #[serde(rename = "vpnApp")]
    #[doc = "The app is a VPN."]
    VpnApp,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it)."]
pub enum ProductProductPricingEnum {
    #[serde(rename = "unknown")]
    #[doc = "Unknown pricing, used to denote an approved product that is not generally available."]
    Unknown,
    #[serde(rename = "free")]
    #[doc = "The product is free."]
    Free,
    #[serde(rename = "freeWithInAppPurchase")]
    #[doc = "The product is free, but offers in-app purchases."]
    FreeWithInAppPurchase,
    #[serde(rename = "paid")]
    #[doc = "The product is paid."]
    Paid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a product's approval status is changed."]
pub struct ProductApprovalEvent {
    #[serde(rename = "approved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the product was approved or unapproved. This field will always be present."]
    pub approved: ::std::option::Option<ProductApprovalEventApprovedEnum>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") for which the approval status has changed. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the product was approved or unapproved. This field will always be present."]
pub enum ProductApprovalEventApprovedEnum {
    #[serde(rename = "unknown")]
    #[doc = "Conveys no information."]
    Unknown,
    #[serde(rename = "approved")]
    #[doc = "The product was approved."]
    Approved,
    #[serde(rename = "unapproved")]
    #[doc = "The product was unapproved."]
    Unapproved,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated whenever a product's availability changes."]
pub struct ProductAvailabilityChangeEvent {
    #[serde(rename = "availabilityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new state of the product. This field will always be present."]
    pub availability_status:
        ::std::option::Option<ProductAvailabilityChangeEventAvailabilityStatusEnum>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the product (e.g. \"app:com.google.android.gm\") for which the product availability changed. This field will always be present."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The new state of the product. This field will always be present."]
pub enum ProductAvailabilityChangeEventAvailabilityStatusEnum {
    #[serde(rename = "unknown")]
    #[doc = "Conveys no information."]
    Unknown,
    #[serde(rename = "available")]
    #[doc = "The previously unavailable product is again available on Google Play."]
    Available,
    #[serde(rename = "removed")]
    #[doc = "The product was removed from Google Play."]
    Removed,
    #[serde(rename = "unpublished")]
    #[doc = "The product was unpublished by the developer."]
    Unpublished,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A product permissions resource represents the set of permissions required by a specific app and whether or not they have been accepted by an enterprise admin. The API can be used to read the set of permissions, and also to update the set to indicate that permissions have been accepted."]
pub struct ProductPermission {
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque string uniquely identifying the permission."]
    pub permission_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the permission has been accepted or not."]
    pub state: ::std::option::Option<ProductPermissionStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the permission has been accepted or not."]
pub enum ProductPermissionStateEnum {
    #[serde(rename = "required")]
    #[doc = "The permission is required by the app but has not yet been accepted by the enterprise."]
    Required,
    #[serde(rename = "accepted")]
    #[doc = "The permission has been accepted by the enterprise."]
    Accepted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the permissions required by a specific app and whether they have been accepted by the enterprise."]
pub struct ProductPermissions {
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permissions required by the app."]
    pub permission: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductPermission>>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the app that the permissions relate to, e.g. \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The policy for a product."]
pub struct ProductPolicy {
    #[serde(rename = "autoInstallPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-install policy for the product."]
    pub auto_install_policy: ::std::option::Option<::std::boxed::Box<AutoInstallPolicy>>,
    #[serde(rename = "autoUpdateMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-update mode for the product."]
    pub auto_update_mode: ::std::option::Option<ProductPolicyAutoUpdateModeEnum>,
    #[serde(rename = "managedConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The managed configuration for the product."]
    pub managed_configuration: ::std::option::Option<::std::boxed::Box<ManagedConfiguration>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product. For example, \"app:com.google.android.gm\"."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Grants the device visibility to the specified product release track(s), identified by trackIds. The list of release tracks of a product can be obtained by calling Products.Get."]
    pub track_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Use trackIds instead."]
    pub tracks: ::std::option::Option<::std::vec::Vec<ProductPolicyTracksEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The auto-update mode for the product."]
pub enum ProductPolicyAutoUpdateModeEnum {
    #[serde(rename = "autoUpdateModeUnspecified")]
    #[doc = "Unspecified. Defaults to AUTO_UPDATE_DEFAULT."]
    AutoUpdateModeUnspecified,
    #[serde(rename = "autoUpdateDefault")]
    #[doc = "The app is automatically updated with low priority to minimize the impact on the user. The app is updated when the following constraints are met: * The device is not actively used * The device is connected to a Wi-Fi network. * The device is charging * If the system update policy is set to `WINDOWED`: the local time of the device is within the daily maintenance window The device is notified about a new update within 24 hours after it is published by the developer, after which the app is updated the next time the constraints above are met."]
    AutoUpdateDefault,
    #[serde(rename = "autoUpdatePostponed")]
    #[doc = "The app is not automatically updated for a maximum of 90 days after the app becomes out of date. 90 days after the app becomes out of date, the latest available version is installed automatically with low priority (see AUTO_UPDATE_DEFAULT). After the app is updated it is not automatically updated again until 90 days after it becomes out of date again. The user can still manually update the app from the Play Store at any time."]
    AutoUpdatePostponed,
    #[serde(rename = "autoUpdateHighPriority")]
    #[doc = "The app is updated as soon as possible. No constraints are applied. The device is notified immediately about a new app update after it is published by the developer."]
    AutoUpdateHighPriority,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProductPolicyTracksEnum {
    #[serde(rename = "appTrackUnspecified")]
    #[doc = ""]
    AppTrackUnspecified,
    #[serde(rename = "production")]
    #[doc = ""]
    Production,
    #[serde(rename = "beta")]
    #[doc = ""]
    Beta,
    #[serde(rename = "alpha")]
    #[doc = ""]
    Alpha,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of products."]
pub struct ProductSet {
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of product IDs making up the set of products."]
    pub product_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "productSetBehavior")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The interpretation of this product set. \"unknown\" should never be sent and is ignored if received. \"whitelist\" means that the user is entitled to access the product set. \"includeAll\" means that all products are accessible, including products that are approved, products with revoked approval, and products that have never been approved. \"allApproved\" means that the user is entitled to access all products that are approved for the enterprise. If the value is \"allApproved\" or \"includeAll\", the productId field is ignored. If no value is provided, it is interpreted as \"whitelist\" for backwards compatibility. Further \"allApproved\" or \"includeAll\" does not enable automatic visibility of \"alpha\" or \"beta\" tracks for Android app. Use ProductVisibility to enable \"alpha\" or \"beta\" tracks per user."]
    pub product_set_behavior: ::std::option::Option<ProductSetProductSetBehaviorEnum>,
    #[serde(rename = "productVisibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional list of product IDs making up the product set. Unlike the productID array, in this list It's possible to specify which tracks (alpha, beta, production) of a product are visible to the user. See ProductVisibility and its fields for more information. Specifying the same product ID both here and in the productId array is not allowed and it will result in an error."]
    pub product_visibility:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductVisibility>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The interpretation of this product set. \"unknown\" should never be sent and is ignored if received. \"whitelist\" means that the user is entitled to access the product set. \"includeAll\" means that all products are accessible, including products that are approved, products with revoked approval, and products that have never been approved. \"allApproved\" means that the user is entitled to access all products that are approved for the enterprise. If the value is \"allApproved\" or \"includeAll\", the productId field is ignored. If no value is provided, it is interpreted as \"whitelist\" for backwards compatibility. Further \"allApproved\" or \"includeAll\" does not enable automatic visibility of \"alpha\" or \"beta\" tracks for Android app. Use ProductVisibility to enable \"alpha\" or \"beta\" tracks per user."]
pub enum ProductSetProductSetBehaviorEnum {
    #[serde(rename = "unknown")]
    #[doc = "This value should never be sent and ignored if received."]
    Unknown,
    #[serde(rename = "whitelist")]
    #[doc = "This product set constitutes a whitelist."]
    Whitelist,
    #[serde(rename = "includeAll")]
    #[doc = "This product set represents all products. For Android app it represents only \"production\" track. (The value of the productId field is therefore ignored)."]
    IncludeAll,
    #[serde(rename = "allApproved")]
    #[doc = "This product set represents all approved products. For Android app it represents only \"production\" track. (The value of the product_id field is therefore ignored)."]
    AllApproved,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductSigningCertificate {
    #[serde(rename = "certificateHashSha1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base64 urlsafe encoded SHA1 hash of the certificate. (This field is deprecated in favor of SHA2-256. It should not be used and may be removed at any time.)"]
    pub certificate_hash_sha1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "certificateHashSha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base64 urlsafe encoded SHA2-256 hash of the certificate."]
    pub certificate_hash_sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A product to be made visible to a user."]
pub struct ProductVisibility {
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product ID to make visible to the user. Required for each item in the productVisibility list."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Grants the user visibility to the specified product track(s), identified by trackIds."]
    pub track_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Use trackIds instead."]
    pub tracks: ::std::option::Option<::std::vec::Vec<ProductVisibilityTracksEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ProductVisibilityTracksEnum {
    #[serde(rename = "appTrackUnspecified")]
    #[doc = ""]
    AppTrackUnspecified,
    #[serde(rename = "production")]
    #[doc = ""]
    Production,
    #[serde(rename = "beta")]
    #[doc = ""]
    Beta,
    #[serde(rename = "alpha")]
    #[doc = ""]
    Alpha,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsApproveRequest {
    #[serde(rename = "approvalUrlInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approval URL that was shown to the user. Only the permissions shown to the user with that URL will be accepted, which may not be the product's entire set of permissions. For example, the URL may only display new permissions from an update after the product was approved, or not include new permissions if the product was updated since the URL was generated."]
    pub approval_url_info: ::std::option::Option<::std::boxed::Box<ApprovalUrlInfo>>,
    #[serde(rename = "approvedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets how new permission requests for the product are handled. \"allPermissions\" automatically approves all current and future permissions for the product. \"currentPermissionsOnly\" approves the current set of permissions for the product, but any future permissions added through updates will require manual reapproval. If not specified, only the current set of permissions will be approved."]
    pub approved_permissions: ::std::option::Option<ProductsApproveRequestApprovedPermissionsEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Sets how new permission requests for the product are handled. \"allPermissions\" automatically approves all current and future permissions for the product. \"currentPermissionsOnly\" approves the current set of permissions for the product, but any future permissions added through updates will require manual reapproval. If not specified, only the current set of permissions will be approved."]
pub enum ProductsApproveRequestApprovedPermissionsEnum {
    #[serde(rename = "currentPermissionsOnly")]
    #[doc = "Approve only the permissions the product requires at approval time. If an update requires additional permissions, the app will not be updated on devices associated with enterprise users until the additional permissions are approved."]
    CurrentPermissionsOnly,
    #[serde(rename = "allPermissions")]
    #[doc = "All current and future permissions the app requires are automatically approved."]
    AllPermissions,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsGenerateApprovalUrlResponse {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL that can be rendered in an iframe to display the permissions (if any) of a product. This URL can be used to approve the product only once and only within 24 hours of being generated, using the Products.approve call. If the product is currently unapproved and has no permissions, this URL will point to an empty page. If the product is currently approved, a URL will only be generated if that product has added permissions since it was last approved, and the URL will only display those new permissions that have not yet been accepted."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsListResponse {
    #[serde(rename = "pageInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General pagination information."]
    pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about a product (e.g. an app) in the Google Play store, for display to an enterprise admin."]
    pub product: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
    #[serde(rename = "tokenPagination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination information for token pagination."]
    pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A service account identity, including the name and credentials that can be used to authenticate as the service account."]
pub struct ServiceAccount {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Credentials that can be used to authenticate as this ServiceAccount."]
    pub key: ::std::option::Option<::std::boxed::Box<ServiceAccountKey>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account name of the service account, in the form of an email address. Assigned by the server."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Credentials that can be used to authenticate as a service account."]
pub struct ServiceAccountKey {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The body of the private key credentials file, in string format. This is only populated when the ServiceAccountKey is created, and is not stored by Google."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque, unique identifier for this ServiceAccountKey. Assigned by the server."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public key data for the credentials file. This is an X.509 cert. If you are using the googleCredentials key type, this is identical to the cert that can be retrieved by using the X.509 cert url inside of the credentials file."]
    pub public_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file format of the generated key data."]
    pub _type: ::std::option::Option<ServiceAccountKeyTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The file format of the generated key data."]
pub enum ServiceAccountKeyTypeEnum {
    #[serde(rename = "googleCredentials")]
    #[doc = "Google Credentials File format."]
    GoogleCredentials,
    #[serde(rename = "pkcs12")]
    #[doc = "PKCS12 format. The password for the PKCS12 file is 'notasecret'. For more information, see https://tools.ietf.org/html/rfc7292. The data for keys of this type are base64 encoded according to RFC 4648 Section 4. See http://tools.ietf.org/html/rfc4648#section-4."]
    Pkcs12,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceAccountKeysListResponse {
    #[serde(rename = "serviceAccountKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service account credentials."]
    pub service_account_key:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceAccountKey>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource returned by the GenerateSignupUrl API, which contains the Signup URL and Completion Token."]
pub struct SignupInfo {
    #[serde(rename = "completionToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque token that will be required, along with the Enterprise Token, for obtaining the enterprise resource from CompleteSignup."]
    pub completion_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL under which the Admin can sign up for an enterprise. The page pointed to cannot be rendered in an iframe."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a managed Google Play store cluster, a list of products displayed as part of a store page."]
pub struct StoreCluster {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID of this cluster. Assigned by the server. Immutable once assigned."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry."]
    pub name: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedText>>>,
    #[serde(rename = "orderInPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters."]
    pub order_in_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster."]
    pub product_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "General setting for the managed Google Play store layout, currently only specifying the page to display the first time the store is opened."]
pub struct StoreLayout {
    #[serde(rename = "homepageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the store page to be used as the homepage. The homepage is the first page shown in the managed Google Play Store. Not specifying a homepage is equivalent to setting the store layout type to \"basic\"."]
    pub homepage_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeLayoutType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The store layout type. By default, this value is set to \"basic\" if the homepageId field is not set, and to \"custom\" otherwise. If set to \"basic\", the layout will consist of all approved apps that have been whitelisted for the user."]
    pub store_layout_type: ::std::option::Option<StoreLayoutStoreLayoutTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The store layout type. By default, this value is set to \"basic\" if the homepageId field is not set, and to \"custom\" otherwise. If set to \"basic\", the layout will consist of all approved apps that have been whitelisted for the user."]
pub enum StoreLayoutStoreLayoutTypeEnum {
    #[serde(rename = "unknown")]
    #[doc = ""]
    Unknown,
    #[serde(rename = "basic")]
    #[doc = ""]
    Basic,
    #[serde(rename = "custom")]
    #[doc = ""]
    Custom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreLayoutClustersListResponse {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A store cluster of an enterprise."]
    pub cluster: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StoreCluster>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreLayoutPagesListResponse {
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A store page of an enterprise."]
    pub page: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StorePage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a managed Google Play store page, made of a localized name and links to other pages. A page also contains clusters defined as a subcollection."]
pub struct StorePage {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID of this page. Assigned by the server. Immutable once assigned."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable."]
    pub link: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry."]
    pub name: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedText>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pagination information returned by a List operation when token pagination is enabled. List operations that supports paging return only one \"page\" of results. This protocol buffer message describes the page that has been returned. When using token pagination, clients should use the next/previous token to get another page of the result. The presence or absence of next/previous token indicates whether a next/previous page is available and provides a mean of accessing this page. ListRequest.page_token should be set to either next_page_token or previous_page_token to access another page."]
pub struct TokenPagination {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tokens to pass to the standard list field 'page_token'. Whenever available, tokens are preferred over manipulating start_index."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Id to name association of a track."]
pub struct TrackInfo {
    #[serde(rename = "trackAlias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A modifiable name for a track. This is the visible name in the play developer console."]
    pub track_alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unmodifiable, unique track identifier. This identifier is the releaseTrackId in the url of the play developer console page that displays the track information."]
    pub track_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Users resource represents an account associated with an enterprise. The account may be specific to a device or to an individual user (who can then use the account across multiple devices). The account may provide access to managed Google Play only, or to other Google services, depending on the identity model: - The Google managed domain identity model requires synchronization to Google account sources (via primaryEmail). - The managed Google Play Accounts identity model provides a dynamic means for enterprises to create user or device accounts as needed. These accounts provide access to managed Google Play. "]
pub struct User {
    #[serde(rename = "accountIdentifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique identifier you create for this user, such as \"user342\" or \"asset#44418\". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users."]
    pub account_identifier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount."]
    pub account_type: ::std::option::Option<UserAccountTypeEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as \"Example, Inc.\") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID for the user."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge."]
    pub management_type: ::std::option::Option<UserManagementTypeEnum>,
    #[serde(rename = "primaryEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's primary email address, for example, \"jsmith@example.com\". Will always be set for Google managed users and not set for EMM managed users."]
    pub primary_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount."]
pub enum UserAccountTypeEnum {
    #[serde(rename = "deviceAccount")]
    #[doc = ""]
    DeviceAccount,
    #[serde(rename = "userAccount")]
    #[doc = ""]
    UserAccount,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge."]
pub enum UserManagementTypeEnum {
    #[serde(rename = "googleManaged")]
    #[doc = ""]
    GoogleManaged,
    #[serde(rename = "emmManaged")]
    #[doc = ""]
    EmmManaged,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsersListResponse {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A user of an enterprise."]
    pub user: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A variable set is a key-value pair of EMM-provided placeholders and its corresponding value, which is attributed to a user. For example, $FIRSTNAME could be a placeholder, and its value could be Alice. Placeholders should start with a '$' sign and should be alphanumeric only."]
pub struct VariableSet {
    #[serde(rename = "placeholder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The placeholder string; defined by EMM."]
    pub placeholder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the placeholder, specific to the user."]
    pub user_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A WebApps resource represents a web app created for an enterprise. Web apps are published to managed Google Play and can be distributed like other Android apps. On a user's device, a web app opens its specified URL."]
pub struct WebApp {
    #[serde(rename = "displayMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display mode of the web app. Possible values include: - \"minimalUi\", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - \"standalone\", the device's status bar and navigation bar are visible when the app is open. - \"fullScreen\", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area. "]
    pub display_mode: ::std::option::Option<WebAppDisplayModeEnum>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used."]
    pub icons: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebAppIcon>>>,
    #[serde(rename = "isPublished")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A flag whether the app has been published to the Play store yet."]
    pub is_published: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start URL, i.e. the URL that should load when the user opens the application."]
    pub start_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon)."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date."]
    pub version_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webAppId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the application. A string of the form \"app:<package name>\" where the package name always starts with the prefix \"com.google.enterprise.webapp.\" followed by a random id."]
    pub web_app_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The display mode of the web app. Possible values include: - \"minimalUi\", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - \"standalone\", the device's status bar and navigation bar are visible when the app is open. - \"fullScreen\", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area. "]
pub enum WebAppDisplayModeEnum {
    #[serde(rename = "displayModeUnspecified")]
    #[doc = ""]
    DisplayModeUnspecified,
    #[serde(rename = "minimalUi")]
    #[doc = "Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL."]
    MinimalUi,
    #[serde(rename = "standalone")]
    #[doc = "Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible."]
    Standalone,
    #[serde(rename = "fullScreen")]
    #[doc = "Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area."]
    FullScreen,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Icon for a web app."]
pub struct WebAppIcon {
    #[serde(rename = "imageData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actual bytes of the image in a base64url encoded string (c.f. RFC4648, section 5 \"Base 64 Encoding with URL and Filename Safe Alphabet\"). - The image type can be png or jpg. - The image should ideally be square. - The image should ideally have a size of 512x512. "]
    pub image_data: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebAppsListResponse {
    #[serde(rename = "webApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The manifest describing a web app."]
    pub web_app: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebApp>>>,
}
