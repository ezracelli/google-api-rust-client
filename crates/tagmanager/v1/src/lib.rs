#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Account."]
pub struct Account {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Account ID uniquely identifies the GTM Account."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account display name. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shareData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the account shares data anonymously with Google and others. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update"]
    pub share_data: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the Google Tag Manager Account access permissions."]
pub struct AccountAccess {
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Account permissions. Valid account permissions are read and manage. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub permission: ::std::option::Option<::std::vec::Vec<AccountAccessPermissionEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AccountAccessPermissionEnum {
    #[serde(rename = "read")]
    #[doc = ""]
    Read,
    #[serde(rename = "edit")]
    #[doc = ""]
    Edit,
    #[serde(rename = "publish")]
    #[doc = ""]
    Publish,
    #[serde(rename = "delete")]
    #[doc = ""]
    Delete,
    #[serde(rename = "manage")]
    #[doc = ""]
    Manage,
    #[serde(rename = "editWorkspace")]
    #[doc = ""]
    EditWorkspace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a predicate."]
pub struct Condition {
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of named parameters (key/value), depending on the condition's type. Notes: - For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. - At this time, the left operand (arg0) must be a reference to a variable. - For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. - To negate an operator, include a boolean parameter named negate boolean parameter that is set to true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of operator for this condition. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub _type: ::std::option::Option<ConditionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of operator for this condition. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
pub enum ConditionTypeEnum {
    #[serde(rename = "equals")]
    #[doc = ""]
    Equals,
    #[serde(rename = "contains")]
    #[doc = ""]
    Contains,
    #[serde(rename = "startsWith")]
    #[doc = ""]
    StartsWith,
    #[serde(rename = "endsWith")]
    #[doc = ""]
    EndsWith,
    #[serde(rename = "matchRegex")]
    #[doc = ""]
    MatchRegex,
    #[serde(rename = "greater")]
    #[doc = ""]
    Greater,
    #[serde(rename = "greaterOrEquals")]
    #[doc = ""]
    GreaterOrEquals,
    #[serde(rename = "less")]
    #[doc = ""]
    Less,
    #[serde(rename = "lessOrEquals")]
    #[doc = ""]
    LessOrEquals,
    #[serde(rename = "cssSelector")]
    #[doc = ""]
    CssSelector,
    #[serde(rename = "urlMatches")]
    #[doc = ""]
    UrlMatches,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Container."]
pub struct Container {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Container ID uniquely identifies the GTM Container."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional list of domain names associated with the Container. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub domain_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enabledBuiltInVariable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub enabled_built_in_variable:
        ::std::option::Option<::std::vec::Vec<ContainerEnabledBuiltInVariableEnum>>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container display name. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container Notes. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container Public ID."]
    pub public_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZoneCountryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container Country ID. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub time_zone_country_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZoneId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container Time Zone ID. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub time_zone_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Usage Contexts for the Container. Valid values include: web, android, ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
    pub usage_context: ::std::option::Option<::std::vec::Vec<ContainerUsageContextEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ContainerEnabledBuiltInVariableEnum {
    #[serde(rename = "pageUrl")]
    #[doc = ""]
    PageUrl,
    #[serde(rename = "pageHostname")]
    #[doc = ""]
    PageHostname,
    #[serde(rename = "pagePath")]
    #[doc = ""]
    PagePath,
    #[serde(rename = "referrer")]
    #[doc = ""]
    Referrer,
    #[serde(rename = "event")]
    #[doc = "For web or mobile."]
    Event,
    #[serde(rename = "clickElement")]
    #[doc = ""]
    ClickElement,
    #[serde(rename = "clickClasses")]
    #[doc = ""]
    ClickClasses,
    #[serde(rename = "clickId")]
    #[doc = ""]
    ClickId,
    #[serde(rename = "clickTarget")]
    #[doc = ""]
    ClickTarget,
    #[serde(rename = "clickUrl")]
    #[doc = ""]
    ClickUrl,
    #[serde(rename = "clickText")]
    #[doc = ""]
    ClickText,
    #[serde(rename = "firstPartyServingUrl")]
    #[doc = ""]
    FirstPartyServingUrl,
    #[serde(rename = "formElement")]
    #[doc = ""]
    FormElement,
    #[serde(rename = "formClasses")]
    #[doc = ""]
    FormClasses,
    #[serde(rename = "formId")]
    #[doc = ""]
    FormId,
    #[serde(rename = "formTarget")]
    #[doc = ""]
    FormTarget,
    #[serde(rename = "formUrl")]
    #[doc = ""]
    FormUrl,
    #[serde(rename = "formText")]
    #[doc = ""]
    FormText,
    #[serde(rename = "environmentName")]
    #[doc = ""]
    EnvironmentName,
    #[serde(rename = "errorMessage")]
    #[doc = ""]
    ErrorMessage,
    #[serde(rename = "errorUrl")]
    #[doc = ""]
    ErrorUrl,
    #[serde(rename = "errorLine")]
    #[doc = ""]
    ErrorLine,
    #[serde(rename = "newHistoryUrl")]
    #[doc = ""]
    NewHistoryUrl,
    #[serde(rename = "oldHistoryUrl")]
    #[doc = ""]
    OldHistoryUrl,
    #[serde(rename = "newHistoryFragment")]
    #[doc = ""]
    NewHistoryFragment,
    #[serde(rename = "oldHistoryFragment")]
    #[doc = ""]
    OldHistoryFragment,
    #[serde(rename = "newHistoryState")]
    #[doc = ""]
    NewHistoryState,
    #[serde(rename = "oldHistoryState")]
    #[doc = ""]
    OldHistoryState,
    #[serde(rename = "historySource")]
    #[doc = ""]
    HistorySource,
    #[serde(rename = "containerVersion")]
    #[doc = "For web or mobile."]
    ContainerVersion,
    #[serde(rename = "debugMode")]
    #[doc = ""]
    DebugMode,
    #[serde(rename = "randomNumber")]
    #[doc = "For web or mobile."]
    RandomNumber,
    #[serde(rename = "containerId")]
    #[doc = "For web or mobile."]
    ContainerId,
    #[serde(rename = "appId")]
    #[doc = ""]
    AppId,
    #[serde(rename = "appName")]
    #[doc = ""]
    AppName,
    #[serde(rename = "appVersionCode")]
    #[doc = ""]
    AppVersionCode,
    #[serde(rename = "appVersionName")]
    #[doc = ""]
    AppVersionName,
    #[serde(rename = "language")]
    #[doc = ""]
    Language,
    #[serde(rename = "osVersion")]
    #[doc = ""]
    OsVersion,
    #[serde(rename = "platform")]
    #[doc = ""]
    Platform,
    #[serde(rename = "sdkVersion")]
    #[doc = ""]
    SdkVersion,
    #[serde(rename = "deviceName")]
    #[doc = ""]
    DeviceName,
    #[serde(rename = "resolution")]
    #[doc = ""]
    Resolution,
    #[serde(rename = "advertiserId")]
    #[doc = ""]
    AdvertiserId,
    #[serde(rename = "advertisingTrackingEnabled")]
    #[doc = ""]
    AdvertisingTrackingEnabled,
    #[serde(rename = "htmlId")]
    #[doc = ""]
    HtmlId,
    #[serde(rename = "ampBrowserLanguage")]
    #[doc = ""]
    AmpBrowserLanguage,
    #[serde(rename = "ampCanonicalPath")]
    #[doc = ""]
    AmpCanonicalPath,
    #[serde(rename = "ampCanonicalUrl")]
    #[doc = ""]
    AmpCanonicalUrl,
    #[serde(rename = "ampCanonicalHost")]
    #[doc = ""]
    AmpCanonicalHost,
    #[serde(rename = "ampReferrer")]
    #[doc = ""]
    AmpReferrer,
    #[serde(rename = "ampTitle")]
    #[doc = ""]
    AmpTitle,
    #[serde(rename = "ampClientId")]
    #[doc = ""]
    AmpClientId,
    #[serde(rename = "ampClientTimezone")]
    #[doc = ""]
    AmpClientTimezone,
    #[serde(rename = "ampClientTimestamp")]
    #[doc = ""]
    AmpClientTimestamp,
    #[serde(rename = "ampClientScreenWidth")]
    #[doc = ""]
    AmpClientScreenWidth,
    #[serde(rename = "ampClientScreenHeight")]
    #[doc = ""]
    AmpClientScreenHeight,
    #[serde(rename = "ampClientScrollX")]
    #[doc = ""]
    AmpClientScrollX,
    #[serde(rename = "ampClientScrollY")]
    #[doc = ""]
    AmpClientScrollY,
    #[serde(rename = "ampClientMaxScrollX")]
    #[doc = ""]
    AmpClientMaxScrollX,
    #[serde(rename = "ampClientMaxScrollY")]
    #[doc = ""]
    AmpClientMaxScrollY,
    #[serde(rename = "ampTotalEngagedTime")]
    #[doc = ""]
    AmpTotalEngagedTime,
    #[serde(rename = "ampPageViewId")]
    #[doc = ""]
    AmpPageViewId,
    #[serde(rename = "ampPageLoadTime")]
    #[doc = ""]
    AmpPageLoadTime,
    #[serde(rename = "ampPageDownloadTime")]
    #[doc = ""]
    AmpPageDownloadTime,
    #[serde(rename = "ampGtmEvent")]
    #[doc = ""]
    AmpGtmEvent,
    #[serde(rename = "eventName")]
    #[doc = ""]
    EventName,
    #[serde(rename = "firebaseEventParameterCampaign")]
    #[doc = ""]
    FirebaseEventParameterCampaign,
    #[serde(rename = "firebaseEventParameterCampaignAclid")]
    #[doc = ""]
    FirebaseEventParameterCampaignAclid,
    #[serde(rename = "firebaseEventParameterCampaignAnid")]
    #[doc = ""]
    FirebaseEventParameterCampaignAnid,
    #[serde(rename = "firebaseEventParameterCampaignClickTimestamp")]
    #[doc = ""]
    FirebaseEventParameterCampaignClickTimestamp,
    #[serde(rename = "firebaseEventParameterCampaignContent")]
    #[doc = ""]
    FirebaseEventParameterCampaignContent,
    #[serde(rename = "firebaseEventParameterCampaignCp1")]
    #[doc = ""]
    FirebaseEventParameterCampaignCp1,
    #[serde(rename = "firebaseEventParameterCampaignGclid")]
    #[doc = ""]
    FirebaseEventParameterCampaignGclid,
    #[serde(rename = "firebaseEventParameterCampaignSource")]
    #[doc = ""]
    FirebaseEventParameterCampaignSource,
    #[serde(rename = "firebaseEventParameterCampaignTerm")]
    #[doc = ""]
    FirebaseEventParameterCampaignTerm,
    #[serde(rename = "firebaseEventParameterCurrency")]
    #[doc = ""]
    FirebaseEventParameterCurrency,
    #[serde(rename = "firebaseEventParameterDynamicLinkAcceptTime")]
    #[doc = ""]
    FirebaseEventParameterDynamicLinkAcceptTime,
    #[serde(rename = "firebaseEventParameterDynamicLinkLinkid")]
    #[doc = ""]
    FirebaseEventParameterDynamicLinkLinkid,
    #[serde(rename = "firebaseEventParameterNotificationMessageDeviceTime")]
    #[doc = ""]
    FirebaseEventParameterNotificationMessageDeviceTime,
    #[serde(rename = "firebaseEventParameterNotificationMessageId")]
    #[doc = ""]
    FirebaseEventParameterNotificationMessageId,
    #[serde(rename = "firebaseEventParameterNotificationMessageName")]
    #[doc = ""]
    FirebaseEventParameterNotificationMessageName,
    #[serde(rename = "firebaseEventParameterNotificationMessageTime")]
    #[doc = ""]
    FirebaseEventParameterNotificationMessageTime,
    #[serde(rename = "firebaseEventParameterNotificationTopic")]
    #[doc = ""]
    FirebaseEventParameterNotificationTopic,
    #[serde(rename = "firebaseEventParameterPreviousAppVersion")]
    #[doc = ""]
    FirebaseEventParameterPreviousAppVersion,
    #[serde(rename = "firebaseEventParameterPreviousOsVersion")]
    #[doc = ""]
    FirebaseEventParameterPreviousOsVersion,
    #[serde(rename = "firebaseEventParameterPrice")]
    #[doc = ""]
    FirebaseEventParameterPrice,
    #[serde(rename = "firebaseEventParameterProductId")]
    #[doc = ""]
    FirebaseEventParameterProductId,
    #[serde(rename = "firebaseEventParameterQuantity")]
    #[doc = ""]
    FirebaseEventParameterQuantity,
    #[serde(rename = "firebaseEventParameterValue")]
    #[doc = ""]
    FirebaseEventParameterValue,
    #[serde(rename = "videoProvider")]
    #[doc = ""]
    VideoProvider,
    #[serde(rename = "videoUrl")]
    #[doc = ""]
    VideoUrl,
    #[serde(rename = "videoTitle")]
    #[doc = ""]
    VideoTitle,
    #[serde(rename = "videoDuration")]
    #[doc = ""]
    VideoDuration,
    #[serde(rename = "videoPercent")]
    #[doc = ""]
    VideoPercent,
    #[serde(rename = "videoVisible")]
    #[doc = ""]
    VideoVisible,
    #[serde(rename = "videoStatus")]
    #[doc = ""]
    VideoStatus,
    #[serde(rename = "videoCurrentTime")]
    #[doc = ""]
    VideoCurrentTime,
    #[serde(rename = "scrollDepthThreshold")]
    #[doc = ""]
    ScrollDepthThreshold,
    #[serde(rename = "scrollDepthUnits")]
    #[doc = ""]
    ScrollDepthUnits,
    #[serde(rename = "scrollDepthDirection")]
    #[doc = ""]
    ScrollDepthDirection,
    #[serde(rename = "elementVisibilityRatio")]
    #[doc = ""]
    ElementVisibilityRatio,
    #[serde(rename = "elementVisibilityTime")]
    #[doc = ""]
    ElementVisibilityTime,
    #[serde(rename = "elementVisibilityFirstTime")]
    #[doc = ""]
    ElementVisibilityFirstTime,
    #[serde(rename = "elementVisibilityRecentTime")]
    #[doc = ""]
    ElementVisibilityRecentTime,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ContainerUsageContextEnum {
    #[serde(rename = "web")]
    #[doc = ""]
    Web,
    #[serde(rename = "android")]
    #[doc = ""]
    Android,
    #[serde(rename = "ios")]
    #[doc = ""]
    Ios,
    #[serde(rename = "androidSdk5")]
    #[doc = ""]
    AndroidSdk5,
    #[serde(rename = "iosSdk5")]
    #[doc = ""]
    IosSdk5,
    #[serde(rename = "amp")]
    #[doc = ""]
    Amp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the Google Tag Manager Container access permissions."]
pub struct ContainerAccess {
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Container permissions. Valid container permissions are: read, edit, delete, publish. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub permission: ::std::option::Option<::std::vec::Vec<ContainerAccessPermissionEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ContainerAccessPermissionEnum {
    #[serde(rename = "read")]
    #[doc = ""]
    Read,
    #[serde(rename = "edit")]
    #[doc = ""]
    Edit,
    #[serde(rename = "publish")]
    #[doc = ""]
    Publish,
    #[serde(rename = "delete")]
    #[doc = ""]
    Delete,
    #[serde(rename = "manage")]
    #[doc = ""]
    Manage,
    #[serde(rename = "editWorkspace")]
    #[doc = ""]
    EditWorkspace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Container Version."]
pub struct ContainerVersion {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container that this version was taken from."]
    pub container: ::std::option::Option<::std::boxed::Box<Container>>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
    pub container_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A value of true indicates this container version has been deleted."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The folders in the container that this version was taken from."]
    pub folder: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Folder>>>,
    #[serde(rename = "macro")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The macros in the container that this version was taken from."]
    pub _macro: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Macro>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container version display name. @mutable tagmanager.accounts.containers.versions.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes on how to apply this container version in the container. @mutable tagmanager.accounts.containers.versions.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rules in the container that this version was taken from."]
    pub rule: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Rule>>>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tags in the container that this version was taken from."]
    pub tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
    #[serde(rename = "trigger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The triggers in the container that this version was taken from."]
    pub trigger: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The variables in the container that this version was taken from."]
    pub variable: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Container Version Header."]
pub struct ContainerVersionHeader {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
    pub container_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A value of true indicates this container version has been deleted."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container version display name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numMacros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of macros in the container version."]
    pub num_macros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of rules in the container version."]
    pub num_rules: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of tags in the container version."]
    pub num_tags: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numTriggers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of triggers in the container version."]
    pub num_triggers: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of variables in the container version."]
    pub num_variables: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for new container versions."]
pub struct CreateContainerVersionRequestVersionOptions {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the container version to be created."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notes of the container version to be created."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quickPreview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation of this version may be for quick preview and shouldn't be saved."]
    pub quick_preview: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Create container versions response."]
pub struct CreateContainerVersionResponse {
    #[serde(rename = "compilerError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compiler errors or not."]
    pub compiler_error: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "containerVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container version created."]
    pub container_version: ::std::option::Option<::std::boxed::Box<ContainerVersion>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Environment. Note that a user can create, delete and update environments of type USER, but can only update the enable_debug and url fields of environments of other types."]
pub struct Environment {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authorizationCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The environment authorization code."]
    pub authorization_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authorizationTimestampMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last update time-stamp for the authorization code."]
    pub authorization_timestamp_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub container_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The environment description. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableDebug")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to enable debug by default on for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
    pub enable_debug: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Environment ID uniquely identifies the GTM Environment."]
    pub environment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The environment display name. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this environment."]
    pub _type: ::std::option::Option<EnvironmentTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default preview page url for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of this environment."]
pub enum EnvironmentTypeEnum {
    #[serde(rename = "user")]
    #[doc = "Used for user defined environments."]
    User,
    #[serde(rename = "live")]
    #[doc = "Used for Live environment, which points to the live published container version."]
    Live,
    #[serde(rename = "latest")]
    #[doc = "Used for Latest environment, which points to the latest created container version."]
    Latest,
    #[serde(rename = "draft")]
    #[doc = "Used for Draft environment, which points to the single draft in the container."]
    Draft,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Folder."]
pub struct Folder {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Folder ID uniquely identifies the GTM Folder."]
    pub folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Folder display name. @mutable tagmanager.accounts.containers.folders.create @mutable tagmanager.accounts.containers.folders.update"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Folder's contents."]
pub struct FolderEntities {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of tags inside the folder."]
    pub tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
    #[serde(rename = "trigger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of triggers inside the folder."]
    pub trigger: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of variables inside the folder."]
    pub variable: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List AccountUsers Response."]
pub struct ListAccountUsersResponse {
    #[serde(rename = "userAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All GTM AccountUsers of a GTM Account."]
    pub user_access: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserAccess>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Accounts Response."]
pub struct ListAccountsResponse {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of GTM Accounts that a user has access to."]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List container versions response."]
pub struct ListContainerVersionsResponse {
    #[serde(rename = "containerVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All versions of a GTM Container."]
    pub container_version:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerVersion>>>,
    #[serde(rename = "containerVersionHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All container version headers of a GTM Container."]
    pub container_version_header:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerVersionHeader>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Containers Response."]
pub struct ListContainersResponse {
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All Containers of a GTM Account."]
    pub containers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Container>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Environments Response."]
pub struct ListEnvironmentsResponse {
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All Environments of a GTM Container."]
    pub environments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Environment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Folders Response."]
pub struct ListFoldersResponse {
    #[serde(rename = "folders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All GTM Folders of a GTM Container."]
    pub folders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Folder>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Tags Response."]
pub struct ListTagsResponse {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All GTM Tags of a GTM Container."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List triggers response."]
pub struct ListTriggersResponse {
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All GTM Triggers of a GTM Container."]
    pub triggers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List Variables Response."]
pub struct ListVariablesResponse {
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All GTM Variables of a GTM Container."]
    pub variables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Macro."]
pub struct Macro {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disablingRuleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mobile containers only: A list of rule IDs for disabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub disabling_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enablingRuleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mobile containers only: A list of rule IDs for enabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub enabling_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Macro as computed at storage time. This value is recomputed whenever the macro is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "macroId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Macro ID uniquely identifies the GTM Macro."]
    pub macro_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Macro display name. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes on how to apply this macro in the container. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The macro's parameters. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parent folder id."]
    pub parent_folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleEndMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end timestamp in milliseconds to schedule a macro. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub schedule_end_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleStartMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start timestamp in milliseconds to schedule a macro. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub schedule_start_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Macro Type. @mutable tagmanager.accounts.containers.macros.create @mutable tagmanager.accounts.containers.macros.update"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Parameter."]
pub struct Parameter {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This list parameter's parameters (keys will be ignored). @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub list: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "map")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This map parameter's parameters (must have keys; keys must be unique). @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub map: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub _type: ::std::option::Option<ParameterTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A parameter's value (may contain variable references such as \"{{myVariable}}\") as appropriate to the specified type. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
pub enum ParameterTypeEnum {
    #[serde(rename = "template")]
    #[doc = "May include variable references (such as \"{{myVariable}}\")."]
    Template,
    #[serde(rename = "integer")]
    #[doc = ""]
    Integer,
    #[serde(rename = "boolean")]
    #[doc = ""]
    Boolean,
    #[serde(rename = "list")]
    #[doc = ""]
    List,
    #[serde(rename = "map")]
    #[doc = ""]
    Map,
    #[serde(rename = "triggerReference")]
    #[doc = ""]
    TriggerReference,
    #[serde(rename = "tagReference")]
    #[doc = ""]
    TagReference,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publish container version response."]
pub struct PublishContainerVersionResponse {
    #[serde(rename = "compilerError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compiler errors or not."]
    pub compiler_error: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "containerVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container version created."]
    pub container_version: ::std::option::Option<::std::boxed::Box<ContainerVersion>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Rule."]
pub struct Rule {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of conditions that make up this rule (implicit AND between them). @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update"]
    pub condition: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Rule as computed at storage time. This value is recomputed whenever the rule is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rule display name. @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes on how to apply this rule in the container. @mutable tagmanager.accounts.containers.rules.create @mutable tagmanager.accounts.containers.rules.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Rule ID uniquely identifies the GTM Rule."]
    pub rule_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTag {
    #[serde(rename = "stopOnSetupFailure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, fire the main tag if and only if the setup tag fires successfully. If false, fire the main tag regardless of setup tag firing status."]
    pub stop_on_setup_failure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tagName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the setup tag."]
    pub tag_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Tag."]
pub struct Tag {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blockingRuleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub blocking_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "blockingTriggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub blocking_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firingRuleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub firing_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "firingTriggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub firing_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "liveOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub live_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag display name. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tag's parameters. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parent folder id."]
    pub parent_folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paused")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the tag is paused. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub paused: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub priority: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "scheduleEndMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub schedule_end_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleStartMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub schedule_start_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "setupTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of setup tags. Currently we only allow one."]
    pub setup_tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SetupTag>>>,
    #[serde(rename = "tagFiringOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option to fire this tag."]
    pub tag_firing_option: ::std::option::Option<TagTagFiringOptionEnum>,
    #[serde(rename = "tagId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Tag ID uniquely identifies the GTM Tag."]
    pub tag_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teardownTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of teardown tags. Currently we only allow one."]
    pub teardown_tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeardownTag>>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Tag Type. @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Option to fire this tag."]
pub enum TagTagFiringOptionEnum {
    #[serde(rename = "unlimited")]
    #[doc = "Tag can be fired multiple times per event."]
    Unlimited,
    #[serde(rename = "oncePerEvent")]
    #[doc = "Tag can only be fired per event but can be fired multiple times per load (e.g., app load or page load)."]
    OncePerEvent,
    #[serde(rename = "oncePerLoad")]
    #[doc = "Tag can only be fired per load (e.g., app load or page load)."]
    OncePerLoad,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeardownTag {
    #[serde(rename = "stopTeardownOnFailure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, fire the teardown tag if and only if the main tag fires successfully. If false, fire the teardown tag regardless of main tag firing status."]
    pub stop_teardown_on_failure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tagName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the teardown tag."]
    pub tag_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Trigger"]
pub struct Trigger {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "autoEventFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used in the case of auto event tracking. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub auto_event_filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
    #[serde(rename = "checkValidation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub check_validation: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "continuousTimeMinMilliseconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub continuous_time_min_milliseconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "customEventFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used in the case of custom event, which is fired iff all Conditions are true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub custom_event_filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
    #[serde(rename = "eventName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the GTM event that is fired. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub event_name: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trigger will only fire iff all Conditions are true. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "horizontalScrollPercentageList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub horizontal_scroll_percentage_list: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub interval: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "intervalSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub interval_seconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub limit: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "maxTimerLengthSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub max_timer_length_seconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trigger display name. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional parameters. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parent folder id."]
    pub parent_folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A click trigger CSS selector (i.e. \"a\", \"button\" etc.). Only valid for AMP Click trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub selector: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "totalTimeMinMilliseconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub total_time_min_milliseconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Trigger ID uniquely identifies the GTM Trigger."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub _type: ::std::option::Option<TriggerTypeEnum>,
    #[serde(rename = "uniqueTriggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub unique_trigger_id: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "verticalScrollPercentageList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub vertical_scroll_percentage_list: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "visibilitySelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A visibility trigger CSS selector (i.e. \"#id\"). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub visibility_selector: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "visiblePercentageMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub visible_percentage_max: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "visiblePercentageMin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub visible_percentage_min: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "waitForTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub wait_for_tags: ::std::option::Option<::std::boxed::Box<Parameter>>,
    #[serde(rename = "waitForTagsTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
    pub wait_for_tags_timeout: ::std::option::Option<::std::boxed::Box<Parameter>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update"]
pub enum TriggerTypeEnum {
    #[serde(rename = "pageview")]
    #[doc = ""]
    Pageview,
    #[serde(rename = "domReady")]
    #[doc = ""]
    DomReady,
    #[serde(rename = "windowLoaded")]
    #[doc = ""]
    WindowLoaded,
    #[serde(rename = "customEvent")]
    #[doc = ""]
    CustomEvent,
    #[serde(rename = "triggerGroup")]
    #[doc = ""]
    TriggerGroup,
    #[serde(rename = "always")]
    #[doc = ""]
    Always,
    #[serde(rename = "formSubmission")]
    #[doc = ""]
    FormSubmission,
    #[serde(rename = "click")]
    #[doc = ""]
    Click,
    #[serde(rename = "linkClick")]
    #[doc = ""]
    LinkClick,
    #[serde(rename = "jsError")]
    #[doc = ""]
    JsError,
    #[serde(rename = "historyChange")]
    #[doc = ""]
    HistoryChange,
    #[serde(rename = "timer")]
    #[doc = ""]
    Timer,
    #[serde(rename = "ampClick")]
    #[doc = ""]
    AmpClick,
    #[serde(rename = "ampTimer")]
    #[doc = ""]
    AmpTimer,
    #[serde(rename = "ampScroll")]
    #[doc = ""]
    AmpScroll,
    #[serde(rename = "ampVisibility")]
    #[doc = ""]
    AmpVisibility,
    #[serde(rename = "youTubeVideo")]
    #[doc = ""]
    YouTubeVideo,
    #[serde(rename = "scrollDepth")]
    #[doc = ""]
    ScrollDepth,
    #[serde(rename = "elementVisibility")]
    #[doc = ""]
    ElementVisibility,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a user's permissions to an account and its container."]
pub struct UserAccess {
    #[serde(rename = "accountAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub account_access: ::std::option::Option<::std::boxed::Box<AccountAccess>>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub container_access:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerAccess>>>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User's email address. @mutable tagmanager.accounts.permissions.create"]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account Permission ID."]
    pub permission_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Google Tag Manager Variable."]
pub struct Variable {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Container ID."]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disablingTriggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub disabling_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enablingTriggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub enabling_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Variable display name. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes on how to apply this variable in the container. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The variable's parameters. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parent folder id."]
    pub parent_folder_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleEndMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub schedule_end_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleStartMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub schedule_start_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GTM Variable Type. @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Variable ID uniquely identifies the GTM Variable."]
    pub variable_id: ::std::option::Option<::std::string::String>,
}
