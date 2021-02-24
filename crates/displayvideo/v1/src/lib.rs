#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for ManualTriggerService.ActivateManualTrigger."]
pub struct ActivateManualTriggerRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for custom Active View video viewability metrics."]
pub struct ActiveViewVideoViewabilityMetricConfig {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the custom metric."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum visible video duration required (in seconds) in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first)."]
    pub minimum_duration:
        ::std::option::Option<ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum>,
    #[serde(rename = "minimumQuartile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum visible video duration required, based on the video quartiles, in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first)."]
    pub minimum_quartile:
        ::std::option::Option<ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum>,
    #[serde(rename = "minimumViewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The minimum percentage of the video ad's pixels visible on the screen in order for an impression to be recorded."]
    pub minimum_viewability:
        ::std::option::Option<ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum>,
    #[serde(rename = "minimumVolume")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The minimum percentage of the video ad's volume required in order for an impression to be recorded."]
    pub minimum_volume:
        ::std::option::Option<ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The minimum visible video duration required (in seconds) in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first)."]
pub enum ActiveViewVideoViewabilityMetricConfigMinimumDurationEnum {
    #[serde(rename = "VIDEO_DURATION_UNSPECIFIED")]
    #[doc = "Value is not specified or is unknown in this version."]
    VideoDurationUnspecified,
    #[serde(rename = "VIDEO_DURATION_SECONDS_NONE")]
    #[doc = "No duration value."]
    VideoDurationSecondsNone,
    #[serde(rename = "VIDEO_DURATION_SECONDS_0")]
    #[doc = "0 seconds."]
    VideoDurationSeconds0,
    #[serde(rename = "VIDEO_DURATION_SECONDS_1")]
    #[doc = "1 second."]
    VideoDurationSeconds1,
    #[serde(rename = "VIDEO_DURATION_SECONDS_2")]
    #[doc = "2 seconds."]
    VideoDurationSeconds2,
    #[serde(rename = "VIDEO_DURATION_SECONDS_3")]
    #[doc = "3 seconds."]
    VideoDurationSeconds3,
    #[serde(rename = "VIDEO_DURATION_SECONDS_4")]
    #[doc = "4 seconds."]
    VideoDurationSeconds4,
    #[serde(rename = "VIDEO_DURATION_SECONDS_5")]
    #[doc = "5 seconds."]
    VideoDurationSeconds5,
    #[serde(rename = "VIDEO_DURATION_SECONDS_6")]
    #[doc = "6 seconds."]
    VideoDurationSeconds6,
    #[serde(rename = "VIDEO_DURATION_SECONDS_7")]
    #[doc = "7 seconds."]
    VideoDurationSeconds7,
    #[serde(rename = "VIDEO_DURATION_SECONDS_8")]
    #[doc = "8 seconds."]
    VideoDurationSeconds8,
    #[serde(rename = "VIDEO_DURATION_SECONDS_9")]
    #[doc = "9 seconds."]
    VideoDurationSeconds9,
    #[serde(rename = "VIDEO_DURATION_SECONDS_10")]
    #[doc = "10 seconds."]
    VideoDurationSeconds10,
    #[serde(rename = "VIDEO_DURATION_SECONDS_11")]
    #[doc = "11 seconds."]
    VideoDurationSeconds11,
    #[serde(rename = "VIDEO_DURATION_SECONDS_12")]
    #[doc = "12 seconds."]
    VideoDurationSeconds12,
    #[serde(rename = "VIDEO_DURATION_SECONDS_13")]
    #[doc = "13 seconds."]
    VideoDurationSeconds13,
    #[serde(rename = "VIDEO_DURATION_SECONDS_14")]
    #[doc = "14 seconds."]
    VideoDurationSeconds14,
    #[serde(rename = "VIDEO_DURATION_SECONDS_15")]
    #[doc = "15 seconds."]
    VideoDurationSeconds15,
    #[serde(rename = "VIDEO_DURATION_SECONDS_30")]
    #[doc = "30 seconds."]
    VideoDurationSeconds30,
    #[serde(rename = "VIDEO_DURATION_SECONDS_45")]
    #[doc = "45 seconds."]
    VideoDurationSeconds45,
    #[serde(rename = "VIDEO_DURATION_SECONDS_60")]
    #[doc = "60 seconds."]
    VideoDurationSeconds60,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The minimum visible video duration required, based on the video quartiles, in order for an impression to be recorded. You must specify minimum_duration, minimum_quartile or both. If both are specified, an impression meets the metric criteria if either requirement is met (whichever happens first)."]
pub enum ActiveViewVideoViewabilityMetricConfigMinimumQuartileEnum {
    #[serde(rename = "VIDEO_DURATION_QUARTILE_UNSPECIFIED")]
    #[doc = "Value is not specified or is unknown in this version."]
    VideoDurationQuartileUnspecified,
    #[serde(rename = "VIDEO_DURATION_QUARTILE_NONE")]
    #[doc = "No quartile value."]
    VideoDurationQuartileNone,
    #[serde(rename = "VIDEO_DURATION_QUARTILE_FIRST")]
    #[doc = "First quartile."]
    VideoDurationQuartileFirst,
    #[serde(rename = "VIDEO_DURATION_QUARTILE_SECOND")]
    #[doc = "Second quartile (midpoint)."]
    VideoDurationQuartileSecond,
    #[serde(rename = "VIDEO_DURATION_QUARTILE_THIRD")]
    #[doc = "Third quartile."]
    VideoDurationQuartileThird,
    #[serde(rename = "VIDEO_DURATION_QUARTILE_FOURTH")]
    #[doc = "Fourth quartile (completion)."]
    VideoDurationQuartileFourth,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The minimum percentage of the video ad's pixels visible on the screen in order for an impression to be recorded."]
pub enum ActiveViewVideoViewabilityMetricConfigMinimumViewabilityEnum {
    #[serde(rename = "VIEWABILITY_PERCENT_UNSPECIFIED")]
    #[doc = "Value is not specified or is unknown in this version."]
    ViewabilityPercentUnspecified,
    #[serde(rename = "VIEWABILITY_PERCENT_0")]
    #[doc = "0% viewable."]
    ViewabilityPercent0,
    #[serde(rename = "VIEWABILITY_PERCENT_25")]
    #[doc = "25% viewable."]
    ViewabilityPercent25,
    #[serde(rename = "VIEWABILITY_PERCENT_50")]
    #[doc = "50% viewable."]
    ViewabilityPercent50,
    #[serde(rename = "VIEWABILITY_PERCENT_75")]
    #[doc = "75% viewable."]
    ViewabilityPercent75,
    #[serde(rename = "VIEWABILITY_PERCENT_100")]
    #[doc = "100% viewable."]
    ViewabilityPercent100,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The minimum percentage of the video ad's volume required in order for an impression to be recorded."]
pub enum ActiveViewVideoViewabilityMetricConfigMinimumVolumeEnum {
    #[serde(rename = "VIDEO_VOLUME_PERCENT_UNSPECIFIED")]
    #[doc = "Value is not specified or is unknown in this version."]
    VideoVolumePercentUnspecified,
    #[serde(rename = "VIDEO_VOLUME_PERCENT_0")]
    #[doc = "0% volume."]
    VideoVolumePercent0,
    #[serde(rename = "VIDEO_VOLUME_PERCENT_10")]
    #[doc = "10% volume."]
    VideoVolumePercent10,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of Adloox settings."]
pub struct Adloox {
    #[serde(rename = "excludedAdlooxCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adloox's brand safety settings."]
    pub excluded_adloox_categories:
        ::std::option::Option<::std::vec::Vec<AdlooxExcludedAdlooxCategoriesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AdlooxExcludedAdlooxCategoriesEnum {
    #[serde(rename = "ADLOOX_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any Adloox option."]
    AdlooxUnspecified,
    #[serde(rename = "ADULT_CONTENT_HARD")]
    #[doc = "Adult content (hard)."]
    AdultContentHard,
    #[serde(rename = "ADULT_CONTENT_SOFT")]
    #[doc = "Adult content (soft)."]
    AdultContentSoft,
    #[serde(rename = "ILLEGAL_CONTENT")]
    #[doc = "Illegal content."]
    IllegalContent,
    #[serde(rename = "BORDERLINE_CONTENT")]
    #[doc = "Borderline content."]
    BorderlineContent,
    #[serde(rename = "DISCRIMINATORY_CONTENT")]
    #[doc = "Discriminatory content."]
    DiscriminatoryContent,
    #[serde(rename = "VIOLENT_CONTENT_WEAPONS")]
    #[doc = "Violent content & weapons."]
    ViolentContentWeapons,
    #[serde(rename = "LOW_VIEWABILITY_DOMAINS")]
    #[doc = "Low viewability domains."]
    LowViewabilityDomains,
    #[serde(rename = "FRAUD")]
    #[doc = "Fraud."]
    Fraud,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single advertiser in Display & Video 360 (DV360)."]
pub struct Advertiser {
    #[serde(rename = "adServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Ad server related settings of the advertiser."]
    pub ad_server_config: ::std::option::Option<::std::boxed::Box<AdvertiserAdServerConfig>>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser. Assigned by the system."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Creative related settings of the advertiser."]
    pub creative_config: ::std::option::Option<::std::boxed::Box<AdvertiserCreativeConfig>>,
    #[serde(rename = "dataAccessConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings that control how advertiser data may be accessed."]
    pub data_access_config: ::std::option::Option<::std::boxed::Box<AdvertiserDataAccessConfig>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the advertiser. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Controls whether or not insertion orders and line items of the advertiser can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_SCHEDULED_FOR_DELETION`. * If set to `ENTITY_STATUS_SCHEDULED_FOR_DELETION`, the advertiser will be deleted 30 days from when it was first scheduled for deletion."]
    pub entity_status: ::std::option::Option<AdvertiserEntityStatusEnum>,
    #[serde(rename = "generalConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. General settings of the advertiser."]
    pub general_config: ::std::option::Option<::std::boxed::Box<AdvertiserGeneralConfig>>,
    #[serde(rename = "integrationDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integration details of the advertiser. Only integrationCode is currently applicable to advertiser. Other fields of IntegrationDetails are not supported and will be ignored if provided."]
    pub integration_details: ::std::option::Option<::std::boxed::Box<IntegrationDetails>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the advertiser."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The unique ID of the partner that the advertiser belongs to."]
    pub partner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targeting settings related to ad serving of the advertiser."]
    pub serving_config: ::std::option::Option<::std::boxed::Box<AdvertiserTargetingConfig>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the advertiser was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Controls whether or not insertion orders and line items of the advertiser can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_SCHEDULED_FOR_DELETION`. * If set to `ENTITY_STATUS_SCHEDULED_FOR_DELETION`, the advertiser will be deleted 30 days from when it was first scheduled for deletion."]
pub enum AdvertiserEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ad server related settings of an advertiser."]
pub struct AdvertiserAdServerConfig {
    #[serde(rename = "cmHybridConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for advertisers that use both Campaign Manager 360 (CM360) and third-party ad servers."]
    pub cm_hybrid_config: ::std::option::Option<::std::boxed::Box<CmHybridConfig>>,
    #[serde(rename = "thirdPartyOnlyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for advertisers that use third-party ad servers only."]
    pub third_party_only_config: ::std::option::Option<::std::boxed::Box<ThirdPartyOnlyConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creatives related settings of an advertiser."]
pub struct AdvertiserCreativeConfig {
    #[serde(rename = "dynamicCreativeEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the advertiser is enabled for dynamic creatives."]
    pub dynamic_creative_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "iasClientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An ID for configuring campaign monitoring provided by Integral Ad Service (IAS). The DV360 system will append an IAS \"Campaign Monitor\" tag containing this ID to the creative tag."]
    pub ias_client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obaComplianceDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to use DV360's Online Behavioral Advertising (OBA) compliance. Warning: Changing OBA settings may cause the audit status of your creatives to be reset by some ad exchanges, making them ineligible to serve until they are re-approved."]
    pub oba_compliance_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "videoCreativeDataSharingAuthorized")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "By setting this field to `true`, you, on behalf of your company, authorize Google to use video creatives associated with this Display & Video 360 advertiser to provide reporting and features related to the advertiser's television campaigns. Applicable only when the advertiser has a CM360 hybrid ad server configuration."]
    pub video_creative_data_sharing_authorized: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how advertiser related data may be accessed."]
pub struct AdvertiserDataAccessConfig {
    #[serde(rename = "sdfConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structured Data Files (SDF) settings for the advertiser. If not specified, the SDF settings of the parent partner are used."]
    pub sdf_config: ::std::option::Option<::std::boxed::Box<AdvertiserSdfConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "General settings of an advertiser."]
pub struct AdvertiserGeneralConfig {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Advertiser's currency in ISO 4217 format. Accepted codes and the currencies they represent are: Currency Code : Currency Name * `ARS` : Argentine Peso * `AUD` : Australian Dollar * `BRL` : Brazilian Real * `CAD` : Canadian Dollar * `CHF` : Swiss Franc * `CLP` : Chilean Peso * `CNY` : Chinese Yuan * `COP` : Colombian Peso * `CZK` : Czech Koruna * `DKK` : Danish Krone * `EGP` : Egyption Pound * `EUR` : Euro * `GBP` : British Pound * `HKD` : Hong Kong Dollar * `HUF` : Hungarian Forint * `IDR` : Indonesian Rupiah * `ILS` : Israeli Shekel * `INR` : Indian Rupee * `JPY` : Japanese Yen * `KRW` : South Korean Won * `MXN` : Mexican Pesos * `MYR` : Malaysian Ringgit * `NGN` : Nigerian Naira * `NOK` : Norwegian Krone * `NZD` : New Zealand Dollar * `PEN` : Peruvian Nuevo Sol * `PLN` : Polish Zloty * `RON` : New Romanian Leu * `RUB` : Russian Ruble * `SEK` : Swedish Krona * `TRY` : Turkish Lira * `TWD` : New Taiwan Dollar * `USD` : US Dollar * `ZAR` : South African Rand"]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "domainUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The domain URL of the advertiser's primary website. The system will send this information to publishers that require website URL to associate a campaign with an advertiser. Provide a URL with no path or query string, beginning with `http:` or `https:`. For example, http://www.example.com"]
    pub domain_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The standard TZ database name of the advertiser's time zone. For example, `America/New_York`. See more at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones For CM360 hybrid advertisers, the time zone is the same as that of the associated CM360 account; for third-party only advertisers, the time zone is the same as that of the parent partner."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structured Data Files (SDF) settings of an advertiser."]
pub struct AdvertiserSdfConfig {
    #[serde(rename = "overridePartnerSdfConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not this advertiser overrides the SDF configuration of its parent partner. By default, an advertiser inherits the SDF configuration from the parent partner. To override the partner configuration, set this field to `true` and provide the new configuration in sdfConfig."]
    pub override_partner_sdf_config: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sdfConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SDF configuration for the advertiser. * Required when overridePartnerSdfConfig is `true`. * Output only when overridePartnerSdfConfig is `false`."]
    pub sdf_config: ::std::option::Option<::std::boxed::Box<SdfConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting settings related to ad serving of an advertiser."]
pub struct AdvertiserTargetingConfig {
    #[serde(rename = "exemptTvFromViewabilityTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not connected TV devices are exempt from viewability targeting for all video line items under the advertiser."]
    pub exempt_tv_from_viewability_targeting: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable age range. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AGE_RANGE`."]
pub struct AgeRangeAssignedTargetingOptionDetails {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The age range of an audience. We only support targeting a continuous age range of an audience. Thus, the age range represented in this field can be 1) targeted solely, or, 2) part of a larger continuous age range. The reach of a continuous age range targeting can be expanded by also targeting an audience of an unknown age."]
    pub age_range: ::std::option::Option<AgeRangeAssignedTargetingOptionDetailsAgeRangeEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_AGE_RANGE`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The age range of an audience. We only support targeting a continuous age range of an audience. Thus, the age range represented in this field can be 1) targeted solely, or, 2) part of a larger continuous age range. The reach of a continuous age range targeting can be expanded by also targeting an audience of an unknown age."]
pub enum AgeRangeAssignedTargetingOptionDetailsAgeRangeEnum {
    #[serde(rename = "AGE_RANGE_UNSPECIFIED")]
    #[doc = "Default value when age range is not specified in this version. This enum is a placeholder for default value and does not represent a real age range option."]
    AgeRangeUnspecified,
    #[serde(rename = "AGE_RANGE_18_24")]
    #[doc = "The age range of the audience is 18 to 24."]
    AgeRange1824,
    #[serde(rename = "AGE_RANGE_25_34")]
    #[doc = "The age range of the audience is 25 to 34."]
    AgeRange2534,
    #[serde(rename = "AGE_RANGE_35_44")]
    #[doc = "The age range of the audience is 35 to 44."]
    AgeRange3544,
    #[serde(rename = "AGE_RANGE_45_54")]
    #[doc = "The age range of the audience is 45 to 54."]
    AgeRange4554,
    #[serde(rename = "AGE_RANGE_55_64")]
    #[doc = "The age range of the audience is 55 to 64."]
    AgeRange5564,
    #[serde(rename = "AGE_RANGE_65_PLUS")]
    #[doc = "The age range of the audience is 65 and up."]
    AgeRange65Plus,
    #[serde(rename = "AGE_RANGE_UNKNOWN")]
    #[doc = "The age range of the audience is unknown."]
    AgeRangeUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable age range. This will be populated in the age_range_details field when targeting_type is `TARGETING_TYPE_AGE_RANGE`."]
pub struct AgeRangeTargetingOptionDetails {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The age range of an audience."]
    pub age_range: ::std::option::Option<AgeRangeTargetingOptionDetailsAgeRangeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The age range of an audience."]
pub enum AgeRangeTargetingOptionDetailsAgeRangeEnum {
    #[serde(rename = "AGE_RANGE_UNSPECIFIED")]
    #[doc = "Default value when age range is not specified in this version. This enum is a placeholder for default value and does not represent a real age range option."]
    AgeRangeUnspecified,
    #[serde(rename = "AGE_RANGE_18_24")]
    #[doc = "The age range of the audience is 18 to 24."]
    AgeRange1824,
    #[serde(rename = "AGE_RANGE_25_34")]
    #[doc = "The age range of the audience is 25 to 34."]
    AgeRange2534,
    #[serde(rename = "AGE_RANGE_35_44")]
    #[doc = "The age range of the audience is 35 to 44."]
    AgeRange3544,
    #[serde(rename = "AGE_RANGE_45_54")]
    #[doc = "The age range of the audience is 45 to 54."]
    AgeRange4554,
    #[serde(rename = "AGE_RANGE_55_64")]
    #[doc = "The age range of the audience is 55 to 64."]
    AgeRange5564,
    #[serde(rename = "AGE_RANGE_65_PLUS")]
    #[doc = "The age range of the audience is 65 and up."]
    AgeRange65Plus,
    #[serde(rename = "AGE_RANGE_UNKNOWN")]
    #[doc = "The age range of the audience is unknown."]
    AgeRangeUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned app targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_APP`."]
pub struct AppAssignedTargetingOptionDetails {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the app. Android's Play store app uses bundle ID, for example `com.google.android.gm`. Apple's App store app ID uses 9 digit string, for example `422689480`."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the app."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned app category targeting option. This will be populated in the app_category_details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_APP_CATEGORY`."]
pub struct AppCategoryAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the app category."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_APP_CATEGORY`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable collection of apps. A collection lets you target dynamic groups of related apps that are maintained by the platform, for example `All Apps/Google Play/Games`. This will be populated in the app_category_details field when targeting_type is `TARGETING_TYPE_APP_CATEGORY`."]
pub struct AppCategoryTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the app collection."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single asset."]
pub struct Asset {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The asset content. For uploaded assets, the content is the serving path."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Media ID of the uploaded asset. This is a unique identifier for the asset. This ID can be passed to other API calls, e.g. CreateCreative to associate the asset with a creative."]
    pub media_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Asset association for the creative."]
pub struct AssetAssociation {
    #[serde(rename = "asset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The associated asset."]
    pub asset: ::std::option::Option<::std::boxed::Box<Asset>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role of this asset for the creative."]
    pub role: ::std::option::Option<AssetAssociationRoleEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The role of this asset for the creative."]
pub enum AssetAssociationRoleEnum {
    #[serde(rename = "ASSET_ROLE_UNSPECIFIED")]
    #[doc = "Asset role is not specified or is unknown in this version."]
    AssetRoleUnspecified,
    #[serde(rename = "ASSET_ROLE_MAIN")]
    #[doc = "The asset is the main asset of the creative."]
    AssetRoleMain,
    #[serde(rename = "ASSET_ROLE_BACKUP")]
    #[doc = "The asset is a backup asset of the creative."]
    AssetRoleBackup,
    #[serde(rename = "ASSET_ROLE_POLITE_LOAD")]
    #[doc = "The asset is a polite load asset of the creative."]
    AssetRolePoliteLoad,
    #[serde(rename = "ASSET_ROLE_HEADLINE")]
    #[doc = "Headline of a native creative. The content must be UTF-8 encoded with a length of no more than 25 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleHeadline,
    #[serde(rename = "ASSET_ROLE_LONG_HEADLINE")]
    #[doc = "Long headline of a native creative. The content must be UTF-8 encoded with a length of no more than 50 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleLongHeadline,
    #[serde(rename = "ASSET_ROLE_BODY")]
    #[doc = "Body text of a native creative. The content must be UTF-8 encoded with a length of no more than 90 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleBody,
    #[serde(rename = "ASSET_ROLE_LONG_BODY")]
    #[doc = "Long body text of a native creative. The content must be UTF-8 encoded with a length of no more than 150 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleLongBody,
    #[serde(rename = "ASSET_ROLE_CAPTION_URL")]
    #[doc = "A short, friendly version of the landing page URL to show in the creative. This URL gives people an idea of where they'll arrive after they click on the creative. The content must be UTF-8 encoded with a length of no more than 30 characters. For example, if the landing page URL is 'http://www.example.com/page', the caption URL can be 'example.com'. The protocol (http://) is optional, but the URL can't contain spaces or special characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleCaptionUrl,
    #[serde(rename = "ASSET_ROLE_CALL_TO_ACTION")]
    #[doc = "The text to use on the call-to-action button of a native creative. The content must be UTF-8 encoded with a length of no more than 15 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleCallToAction,
    #[serde(rename = "ASSET_ROLE_ADVERTISER_NAME")]
    #[doc = "The text that identifies the advertiser or brand name. The content must be UTF-8 encoded with a length of no more than 25 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    AssetRoleAdvertiserName,
    #[serde(rename = "ASSET_ROLE_PRICE")]
    #[doc = "The purchase price of your app in the Google play store or iOS app store (for example, $5.99). Note that this value is not automatically synced with the actual value listed in the store. It will always be the one provided when save the creative. The content must be UTF-8 encoded with a length of no more than 15 characters. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE`"]
    AssetRolePrice,
    #[serde(rename = "ASSET_ROLE_ANDROID_APP_ID")]
    #[doc = "The ID of an Android app in the Google play store. You can find this ID in the App’s Google Play Store URL after ‘id’. For example, in https://play.google.com/store/apps/details?id=com.company.appname the identifier is com.company.appname. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE`"]
    AssetRoleAndroidAppId,
    #[serde(rename = "ASSET_ROLE_IOS_APP_ID")]
    #[doc = "The ID of an iOS app in the Apple app store. This ID number can be found in the Apple App Store URL as the string of numbers directly after \"id\". For example, in https://apps.apple.com/us/app/gmail-email-by-google/id422689480 the ID is 422689480. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE`"]
    AssetRoleIosAppId,
    #[serde(rename = "ASSET_ROLE_RATING")]
    #[doc = "The rating of an app in the Google play store or iOS app store. Note that this value is not automatically synced with the actual rating in the store. It will always be the one provided when save the creative. This role is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE`"]
    AssetRoleRating,
    #[serde(rename = "ASSET_ROLE_ICON")]
    #[doc = "The icon of a creative. This role is only supported and required in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE`"]
    AssetRoleIcon,
    #[serde(rename = "ASSET_ROLE_COVER_IMAGE")]
    #[doc = "The cover image of a native video creative. This role is only supported and required in following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    AssetRoleCoverImage,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An assignment between a targetable inventory source and an inventory source group."]
pub struct AssignedInventorySource {
    #[serde(rename = "assignedInventorySourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the assigned inventory source. The ID is only unique within a given inventory source group. It may be reused in other contexts."]
    pub assigned_inventory_source_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventorySourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the inventory source entity being targeted."]
    pub inventory_source_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the assigned inventory source."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An assignment between a location list and a relevant targeting option. Currently, geo region targeting options are the only supported option for assignment."]
pub struct AssignedLocation {
    #[serde(rename = "assignedLocationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the assigned location. The ID is only unique within a location list. It may be reused in other contexts."]
    pub assigned_location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the assigned location."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the targeting option assigned to the location list. Must be of type TARGETING_TYPE_GEO_REGION."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single assigned targeting option, which defines the state of a targeting option for an entity with targeting settings."]
pub struct AssignedTargetingOption {
    #[serde(rename = "ageRangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Age range details. This field will be populated when the TargetingType is `TARGETING_TYPE_AGE_RANGE`."]
    pub age_range_details:
        ::std::option::Option<::std::boxed::Box<AgeRangeAssignedTargetingOptionDetails>>,
    #[serde(rename = "appCategoryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App category details. This field will be populated when the TargetingType is `TARGETING_TYPE_APP_CATEGORY`."]
    pub app_category_details:
        ::std::option::Option<::std::boxed::Box<AppCategoryAssignedTargetingOptionDetails>>,
    #[serde(rename = "appDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App details. This field will be populated when the TargetingType is `TARGETING_TYPE_APP`."]
    pub app_details: ::std::option::Option<::std::boxed::Box<AppAssignedTargetingOptionDetails>>,
    #[serde(rename = "assignedTargetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the assigned targeting option. The ID is only unique within a given line item and targeting type. It may be reused in other contexts."]
    pub assigned_targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "audienceGroupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience targeting details. This field will be populated when the TargetingType is `TARGETING_TYPE_AUDIENCE_GROUP`. You can only target one audience group option per line item."]
    pub audience_group_details:
        ::std::option::Option<::std::boxed::Box<AudienceGroupAssignedTargetingOptionDetails>>,
    #[serde(rename = "authorizedSellerStatusDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authorized seller status details. This field will be populated when the TargetingType is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`. You can only target one authorized seller status option per line item. If a line item doesn't have an authorized seller status option, all authorized sellers indicated as DIRECT or RESELLER in the ads.txt file are targeted by default."]
    pub authorized_seller_status_details: ::std::option::Option<
        ::std::boxed::Box<AuthorizedSellerStatusAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "browserDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browser details. This field will be populated when the TargetingType is `TARGETING_TYPE_BROWSER`."]
    pub browser_details:
        ::std::option::Option<::std::boxed::Box<BrowserAssignedTargetingOptionDetails>>,
    #[serde(rename = "carrierAndIspDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Carrier and ISP details. This field will be populated when the TargetingType is `TARGETING_TYPE_CARRIER_AND_ISP`."]
    pub carrier_and_isp_details:
        ::std::option::Option<::std::boxed::Box<CarrierAndIspAssignedTargetingOptionDetails>>,
    #[serde(rename = "categoryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Category details. This field will be populated when the TargetingType is `TARGETING_TYPE_CATEGORY`. Targeting a category will also target its subcategories. If a category is excluded from targeting and a subcategory is included, the exclusion will take precedence."]
    pub category_details:
        ::std::option::Option<::std::boxed::Box<CategoryAssignedTargetingOptionDetails>>,
    #[serde(rename = "channelDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Channel details. This field will be populated when the TargetingType is `TARGETING_TYPE_CHANNEL`."]
    pub channel_details:
        ::std::option::Option<::std::boxed::Box<ChannelAssignedTargetingOptionDetails>>,
    #[serde(rename = "contentInstreamPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content instream position details. This field will be populated when the TargetingType is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`."]
    pub content_instream_position_details: ::std::option::Option<
        ::std::boxed::Box<ContentInstreamPositionAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "contentOutstreamPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content outstream position details. This field will be populated when the TargetingType is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`."]
    pub content_outstream_position_details: ::std::option::Option<
        ::std::boxed::Box<ContentOutstreamPositionAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "dayAndTimeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day and time details. This field will be populated when the TargetingType is `TARGETING_TYPE_DAY_AND_TIME`."]
    pub day_and_time_details:
        ::std::option::Option<::std::boxed::Box<DayAndTimeAssignedTargetingOptionDetails>>,
    #[serde(rename = "deviceMakeModelDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device make and model details. This field will be populated when the TargetingType is `TARGETING_TYPE_DEVICE_MAKE_MODEL`."]
    pub device_make_model_details:
        ::std::option::Option<::std::boxed::Box<DeviceMakeModelAssignedTargetingOptionDetails>>,
    #[serde(rename = "deviceTypeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device Type details. This field will be populated when the TargetingType is `TARGETING_TYPE_DEVICE_TYPE`."]
    pub device_type_details:
        ::std::option::Option<::std::boxed::Box<DeviceTypeAssignedTargetingOptionDetails>>,
    #[serde(rename = "digitalContentLabelExclusionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Digital content label details. This field will be populated when the TargetingType is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`. Digital content labels are targeting exclusions. Advertiser level digital content label exclusions, if set, are always applied in serving (even though they aren't visible in line item settings). Line item settings can exclude content labels in addition to advertiser exclusions, but can't override them. A line item won't serve if all the digital content labels are excluded."]
    pub digital_content_label_exclusion_details:
        ::std::option::Option<::std::boxed::Box<DigitalContentLabelAssignedTargetingOptionDetails>>,
    #[serde(rename = "environmentDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment details. This field will be populated when the TargetingType is `TARGETING_TYPE_ENVIRONMENT`."]
    pub environment_details:
        ::std::option::Option<::std::boxed::Box<EnvironmentAssignedTargetingOptionDetails>>,
    #[serde(rename = "exchangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exchange details. This field will be populated when the TargetingType is `TARGETING_TYPE_EXCHANGE`."]
    pub exchange_details:
        ::std::option::Option<::std::boxed::Box<ExchangeAssignedTargetingOptionDetails>>,
    #[serde(rename = "genderDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Gender details. This field will be populated when the TargetingType is `TARGETING_TYPE_GENDER`."]
    pub gender_details:
        ::std::option::Option<::std::boxed::Box<GenderAssignedTargetingOptionDetails>>,
    #[serde(rename = "geoRegionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic region details. This field will be populated when the TargetingType is `TARGETING_TYPE_GEO_REGION`."]
    pub geo_region_details:
        ::std::option::Option<::std::boxed::Box<GeoRegionAssignedTargetingOptionDetails>>,
    #[serde(rename = "householdIncomeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Household income details. This field will be populated when the TargetingType is `TARGETING_TYPE_HOUSEHOLD_INCOME`."]
    pub household_income_details:
        ::std::option::Option<::std::boxed::Box<HouseholdIncomeAssignedTargetingOptionDetails>>,
    #[serde(rename = "inheritance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The inheritance status of the assigned targeting option."]
    pub inheritance: ::std::option::Option<AssignedTargetingOptionInheritanceEnum>,
    #[serde(rename = "inventorySourceDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory source details. This field will be populated when the TargetingType is `TARGETING_TYPE_INVENTORY_SOURCE`."]
    pub inventory_source_details:
        ::std::option::Option<::std::boxed::Box<InventorySourceAssignedTargetingOptionDetails>>,
    #[serde(rename = "inventorySourceGroupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory source group details. This field will be populated when the TargetingType is `TARGETING_TYPE_INVENTORY_SOURCE_GROUP`."]
    pub inventory_source_group_details: ::std::option::Option<
        ::std::boxed::Box<InventorySourceGroupAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "keywordDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Keyword details. This field will be populated when the TargetingType is `TARGETING_TYPE_KEYWORD`. A maximum of 5000 direct negative keywords can be assigned to a line item. No limit on number of positive keywords that can be assigned."]
    pub keyword_details:
        ::std::option::Option<::std::boxed::Box<KeywordAssignedTargetingOptionDetails>>,
    #[serde(rename = "languageDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language details. This field will be populated when the TargetingType is `TARGETING_TYPE_LANGUAGE`."]
    pub language_details:
        ::std::option::Option<::std::boxed::Box<LanguageAssignedTargetingOptionDetails>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name for this assigned targeting option."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negativeKeywordListDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Keyword details. This field will be populated when the TargetingType is `TARGETING_TYPE_NEGATIVE_KEYWORD_LIST`. A maximum of 4 negative keyword lists can be assigned to a line item."]
    pub negative_keyword_list_details:
        ::std::option::Option<::std::boxed::Box<NegativeKeywordListAssignedTargetingOptionDetails>>,
    #[serde(rename = "onScreenPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "On screen position details. This field will be populated when the TargetingType is `TARGETING_TYPE_ON_SCREEN_POSITION`."]
    pub on_screen_position_details:
        ::std::option::Option<::std::boxed::Box<OnScreenPositionAssignedTargetingOptionDetails>>,
    #[serde(rename = "operatingSystemDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system details. This field will be populated when the TargetingType is `TARGETING_TYPE_OPERATING_SYSTEM`."]
    pub operating_system_details:
        ::std::option::Option<::std::boxed::Box<OperatingSystemAssignedTargetingOptionDetails>>,
    #[serde(rename = "parentalStatusDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parental status details. This field will be populated when the TargetingType is `TARGETING_TYPE_PARENTAL_STATUS`."]
    pub parental_status_details:
        ::std::option::Option<::std::boxed::Box<ParentalStatusAssignedTargetingOptionDetails>>,
    #[serde(rename = "proximityLocationListDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proximity location list details. This field will be populated when the TargetingType is `TARGETING_TYPE_PROXIMITY_LOCATION_LIST`."]
    pub proximity_location_list_details: ::std::option::Option<
        ::std::boxed::Box<ProximityLocationListAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "regionalLocationListDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regional location list details. This field will be populated when the TargetingType is `TARGETING_TYPE_REGIONAL_LOCATION_LIST`."]
    pub regional_location_list_details: ::std::option::Option<
        ::std::boxed::Box<RegionalLocationListAssignedTargetingOptionDetails>,
    >,
    #[serde(rename = "sensitiveCategoryExclusionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sensitive category details. This field will be populated when the TargetingType is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`. Sensitive categories are targeting exclusions. Advertiser level sensitive category exclusions, if set, are always applied in serving (even though they aren't visible in line item settings). Line item settings can exclude sensitive categories in addition to advertiser exclusions, but can't override them."]
    pub sensitive_category_exclusion_details:
        ::std::option::Option<::std::boxed::Box<SensitiveCategoryAssignedTargetingOptionDetails>>,
    #[serde(rename = "subExchangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub-exchange details. This field will be populated when the TargetingType is `TARGETING_TYPE_SUB_EXCHANGE`."]
    pub sub_exchange_details:
        ::std::option::Option<::std::boxed::Box<SubExchangeAssignedTargetingOptionDetails>>,
    #[serde(rename = "targetingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies the type of this assigned targeting option."]
    pub targeting_type: ::std::option::Option<AssignedTargetingOptionTargetingTypeEnum>,
    #[serde(rename = "thirdPartyVerifierDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third party verification details. This field will be populated when the TargetingType is `TARGETING_TYPE_THIRD_PARTY_VERIFIER`."]
    pub third_party_verifier_details:
        ::std::option::Option<::std::boxed::Box<ThirdPartyVerifierAssignedTargetingOptionDetails>>,
    #[serde(rename = "urlDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL details. This field will be populated when the TargetingType is `TARGETING_TYPE_URL`."]
    pub url_details: ::std::option::Option<::std::boxed::Box<UrlAssignedTargetingOptionDetails>>,
    #[serde(rename = "userRewardedContentDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User rewarded content details. This field will be populated when the TargetingType is `TARGETING_TYPE_USER_REWARDED_CONTENT`."]
    pub user_rewarded_content_details:
        ::std::option::Option<::std::boxed::Box<UserRewardedContentAssignedTargetingOptionDetails>>,
    #[serde(rename = "videoPlayerSizeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video player size details. This field will be populated when the TargetingType is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`."]
    pub video_player_size_details:
        ::std::option::Option<::std::boxed::Box<VideoPlayerSizeAssignedTargetingOptionDetails>>,
    #[serde(rename = "viewabilityDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Viewability details. This field will be populated when the TargetingType is `TARGETING_TYPE_VIEWABILITY`. You can only target one viewability option per line item."]
    pub viewability_details:
        ::std::option::Option<::std::boxed::Box<ViewabilityAssignedTargetingOptionDetails>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The inheritance status of the assigned targeting option."]
pub enum AssignedTargetingOptionInheritanceEnum {
    #[serde(rename = "INHERITANCE_UNSPECIFIED")]
    #[doc = "The inheritance is unspecified or unknown."]
    InheritanceUnspecified,
    #[serde(rename = "NOT_INHERITED")]
    #[doc = "The assigned targeting option is not inherited from higher level entity."]
    NotInherited,
    #[serde(rename = "INHERITED_FROM_PARTNER")]
    #[doc = "The assigned targeting option is inherited from partner targeting settings."]
    InheritedFromPartner,
    #[serde(rename = "INHERITED_FROM_ADVERTISER")]
    #[doc = "The assigned targeting option is inherited from advertiser targeting settings."]
    InheritedFromAdvertiser,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Identifies the type of this assigned targeting option."]
pub enum AssignedTargetingOptionTargetingTypeEnum {
    #[serde(rename = "TARGETING_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown in this version."]
    TargetingTypeUnspecified,
    #[serde(rename = "TARGETING_TYPE_CHANNEL")]
    #[doc = "Target a channel (a custom group of related websites or apps)."]
    TargetingTypeChannel,
    #[serde(rename = "TARGETING_TYPE_APP_CATEGORY")]
    #[doc = "Target an app category (for example, education or puzzle games)."]
    TargetingTypeAppCategory,
    #[serde(rename = "TARGETING_TYPE_APP")]
    #[doc = "Target a specific app (for example, Angry Birds)."]
    TargetingTypeApp,
    #[serde(rename = "TARGETING_TYPE_URL")]
    #[doc = "Target a specific url (for example, quora.com)."]
    TargetingTypeUrl,
    #[serde(rename = "TARGETING_TYPE_DAY_AND_TIME")]
    #[doc = "Target ads during a chosen time period on a specific day."]
    TargetingTypeDayAndTime,
    #[serde(rename = "TARGETING_TYPE_AGE_RANGE")]
    #[doc = "Target ads to a specific age range (for example, 18-24)."]
    TargetingTypeAgeRange,
    #[serde(rename = "TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    #[doc = "Target ads to the specified regions on a regional location list."]
    TargetingTypeRegionalLocationList,
    #[serde(rename = "TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    #[doc = "Target ads to the specified points of interest on a proximity location list."]
    TargetingTypeProximityLocationList,
    #[serde(rename = "TARGETING_TYPE_GENDER")]
    #[doc = "Target ads to a specific gender (for example, female or male)."]
    TargetingTypeGender,
    #[serde(rename = "TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    #[doc = "Target a specific video player size for video ads."]
    TargetingTypeVideoPlayerSize,
    #[serde(rename = "TARGETING_TYPE_USER_REWARDED_CONTENT")]
    #[doc = "Target user rewarded content for video ads."]
    TargetingTypeUserRewardedContent,
    #[serde(rename = "TARGETING_TYPE_PARENTAL_STATUS")]
    #[doc = "Target ads to a specific parental status (for example, parent or not a parent)."]
    TargetingTypeParentalStatus,
    #[serde(rename = "TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    #[doc = "Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll)."]
    TargetingTypeContentInstreamPosition,
    #[serde(rename = "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    #[doc = "Target ads in a specific content outstream position."]
    TargetingTypeContentOutstreamPosition,
    #[serde(rename = "TARGETING_TYPE_DEVICE_TYPE")]
    #[doc = "Target ads to a specific device type (for example, tablet or connected TV)."]
    TargetingTypeDeviceType,
    #[serde(rename = "TARGETING_TYPE_AUDIENCE_GROUP")]
    #[doc = "Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time."]
    TargetingTypeAudienceGroup,
    #[serde(rename = "TARGETING_TYPE_BROWSER")]
    #[doc = "Target ads to specific web browsers (for example, Chrome)."]
    TargetingTypeBrowser,
    #[serde(rename = "TARGETING_TYPE_HOUSEHOLD_INCOME")]
    #[doc = "Target ads to a specific household income range (for example, top 10%)."]
    TargetingTypeHouseholdIncome,
    #[serde(rename = "TARGETING_TYPE_ON_SCREEN_POSITION")]
    #[doc = "Target ads in a specific on screen position."]
    TargetingTypeOnScreenPosition,
    #[serde(rename = "TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    #[doc = "Filter web sites through third party verification (for example, IAS or DoubleVerify)."]
    TargetingTypeThirdPartyVerifier,
    #[serde(rename = "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    #[doc = "Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences)."]
    TargetingTypeDigitalContentLabelExclusion,
    #[serde(rename = "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    #[doc = "Filter website content by sensitive categories (for example, adult)."]
    TargetingTypeSensitiveCategoryExclusion,
    #[serde(rename = "TARGETING_TYPE_ENVIRONMENT")]
    #[doc = "Target ads to a specific environment (for example, web or app)."]
    TargetingTypeEnvironment,
    #[serde(rename = "TARGETING_TYPE_CARRIER_AND_ISP")]
    #[doc = "Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange)."]
    TargetingTypeCarrierAndIsp,
    #[serde(rename = "TARGETING_TYPE_OPERATING_SYSTEM")]
    #[doc = "Target ads to a specific operating system (for example, macOS)."]
    TargetingTypeOperatingSystem,
    #[serde(rename = "TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    #[doc = "Target ads to a specific device make or model (for example, Roku or Samsung)."]
    TargetingTypeDeviceMakeModel,
    #[serde(rename = "TARGETING_TYPE_KEYWORD")]
    #[doc = "Target ads to a specific keyword (for example, dog or retriever)."]
    TargetingTypeKeyword,
    #[serde(rename = "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    #[doc = "Target ads to a specific negative keyword list."]
    TargetingTypeNegativeKeywordList,
    #[serde(rename = "TARGETING_TYPE_VIEWABILITY")]
    #[doc = "Target ads to a specific viewability (for example, 80% viewable)."]
    TargetingTypeViewability,
    #[serde(rename = "TARGETING_TYPE_CATEGORY")]
    #[doc = "Target ads to a specific content category (for example, arts & entertainment)."]
    TargetingTypeCategory,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE")]
    #[doc = "Purchase impressions from specific deals and auction packages."]
    TargetingTypeInventorySource,
    #[serde(rename = "TARGETING_TYPE_LANGUAGE")]
    #[doc = "Target ads to a specific language (for example, English or Japanese)."]
    TargetingTypeLanguage,
    #[serde(rename = "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    #[doc = "Target ads to ads.txt authorized sellers."]
    TargetingTypeAuthorizedSellerStatus,
    #[serde(rename = "TARGETING_TYPE_GEO_REGION")]
    #[doc = "Target ads to a specific regional location (for example, a city or state)."]
    TargetingTypeGeoRegion,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    #[doc = "Purchase impressions from a group of deals and auction packages."]
    TargetingTypeInventorySourceGroup,
    #[serde(rename = "TARGETING_TYPE_EXCHANGE")]
    #[doc = "Purchase impressions from specific exchanges."]
    TargetingTypeExchange,
    #[serde(rename = "TARGETING_TYPE_SUB_EXCHANGE")]
    #[doc = "Purchase impressions from specific sub-exchanges."]
    TargetingTypeSubExchange,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single assigned user role, which defines a user's authorized interaction with a specified partner or advertiser."]
pub struct AssignedUserRole {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the advertiser that the assigend user role applies to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assignedUserRoleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of the assigned user role."]
    pub assigned_user_role_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the partner that the assigned user role applies to."]
    pub partner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user role to assign to a user for the entity."]
    pub user_role: ::std::option::Option<AssignedUserRoleUserRoleEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The user role to assign to a user for the entity."]
pub enum AssignedUserRoleUserRoleEnum {
    #[serde(rename = "USER_ROLE_UNSPECIFIED")]
    #[doc = "Default value when the user role is not specified or is unknown in this version."]
    UserRoleUnspecified,
    #[serde(rename = "ADMIN")]
    #[doc = "The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They can view and edit billing information, create or modify users, and enable or disable exchanges. This role can only be assigned for a partner entity."]
    Admin,
    #[serde(rename = "ADMIN_PARTNER_CLIENT")]
    #[doc = "The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They can create and modify other `ADMIN_PARTNER_CLIENT` users and view billing information. They cannot view revenue models, markups, or any other reseller-sensitive fields. This role can only be assigned for a partner entity."]
    AdminPartnerClient,
    #[serde(rename = "STANDARD")]
    #[doc = "The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They cannot create and modify users or view billing information."]
    Standard,
    #[serde(rename = "STANDARD_PLANNER")]
    #[doc = "The user can view all campaigns, creatives, insertion orders, line items, and reports for the entity, including all cost data. They can create and modify planning-related features, including plans and inventory."]
    StandardPlanner,
    #[serde(rename = "STANDARD_PLANNER_LIMITED")]
    #[doc = "The user can view all campaigns, creatives, insertion orders, line items, and reports for the entity. They can create or modify planning-related features, including plans and inventory. They have no access to cost data and cannot start, accept, or negotiate deals."]
    StandardPlannerLimited,
    #[serde(rename = "STANDARD_PARTNER_CLIENT")]
    #[doc = "The user can manage campaigns, creatives, insertion orders, line items, and reports for the entity. They cannot create or modify other users or view billing information. They cannot view revenue models, markups, or any other reseller-sensitive fields. This role can only be assigned for an advertiser entity."]
    StandardPartnerClient,
    #[serde(rename = "READ_ONLY")]
    #[doc = "The user can only build reports and view data for the entity."]
    ReadOnly,
    #[serde(rename = "REPORTING_ONLY")]
    #[doc = "The user can only create and manage reports."]
    ReportingOnly,
    #[serde(rename = "LIMITED_REPORTING_ONLY")]
    #[doc = "The user can only create and manage the following client-safe reports: General, Audience Performance, Cross-Partner, Keyword, Order ID, Category, and Third-Party Data Provider."]
    LimitedReportingOnly,
    #[serde(rename = "CREATIVE")]
    #[doc = "The user can view media plan information they need to collaborate, but can't view cost-related data or Marketplace."]
    Creative,
    #[serde(rename = "CREATIVE_ADMIN")]
    #[doc = "The user can view media plan information they need to collaborate, but can't view cost-related data or Marketplace. In addition, they can add other creative admins or creative users to the entity."]
    CreativeAdmin,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned audience group targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AUDIENCE_GROUP`. The relation between each group is UNION, except for excluded_first_and_third_party_audience_group and excluded_google_audience_group, of which COMPLEMENT is UNION'ed with other groups."]
pub struct AudienceGroupAssignedTargetingOptionDetails {
    #[serde(rename = "excludedFirstAndThirdPartyAudienceGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The first and third party audience ids and recencies of the excluded first and third party audience group. Used for negative targeting. Its COMPLEMENT is used to UNION other audience groups."]
    pub excluded_first_and_third_party_audience_group:
        ::std::option::Option<::std::boxed::Box<FirstAndThirdPartyAudienceGroup>>,
    #[serde(rename = "excludedGoogleAudienceGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google audience ids of the excluded Google audience group. Used for negative targeting. It's COMPLEMENT is used to UNION other audience groups. Only contains Affinity, In-market and Installed-apps type Google audiences. All items are logically ‘OR’ of each other."]
    pub excluded_google_audience_group:
        ::std::option::Option<::std::boxed::Box<GoogleAudienceGroup>>,
    #[serde(rename = "includedCombinedAudienceGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The combined audience ids of the included combined audience group. Contains combined audience ids only."]
    pub included_combined_audience_group:
        ::std::option::Option<::std::boxed::Box<CombinedAudienceGroup>>,
    #[serde(rename = "includedCustomListGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom list ids of the included custom list group. Contains custom list ids only."]
    pub included_custom_list_group: ::std::option::Option<::std::boxed::Box<CustomListGroup>>,
    #[serde(rename = "includedFirstAndThirdPartyAudienceGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The first and third party audience ids and recencies of included first and third party audience groups. Each first and third party audience group contains first and third party audience ids only. The relation between each first and third party audience group is INTERSECTION, and the result is UNION'ed with other audience groups. Repeated groups with same settings will be ignored."]
    pub included_first_and_third_party_audience_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirstAndThirdPartyAudienceGroup>>>,
    #[serde(rename = "includedGoogleAudienceGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google audience ids of the included Google audience group. Contains Google audience ids only."]
    pub included_google_audience_group:
        ::std::option::Option<::std::boxed::Box<GoogleAudienceGroup>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The length an audio or a video has been played."]
pub struct AudioVideoOffset {
    #[serde(rename = "percentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset in percentage of the audio or video duration."]
    pub percentage: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset in seconds from the start of the audio or video."]
    pub seconds: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for AdvertiserService.AuditAdvertiser."]
pub struct AuditAdvertiserResponse {
    #[serde(rename = "adGroupCriteriaCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of individual targeting options from the following targeting types that are assigned to a line item under this advertiser. These individual targeting options count towards the limit of 4500000 ad group targeting options per advertiser. Qualifying Targeting types: * Channels, URLs, apps, and collections * Demographic * Google Audiences, including Affinity, Custom Affinity, and In-market audiences * Inventory source * Keyword * Mobile app category * User lists * Video targeting * Viewability"]
    pub ad_group_criteria_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignCriteriaCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of individual targeting options from the following targeting types that are assigned to a line item under this advertiser. These individual targeting options count towards the limit of 900000 campaign targeting options per advertiser. Qualifying Targeting types: * Position * Browser * Connection speed * Day and time * Device and operating system * Digital content label * Sensitive categories * Environment * Geography, including business chains and proximity * ISP * Language * Third-party verification"]
    pub campaign_criteria_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channelsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of channels created under this advertiser. These channels count towards the limit of 1000 channels per advertiser."]
    pub channels_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negativeKeywordListsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of negative keyword lists created under this advertiser. These negative keyword lists count towards the limit of 20 negative keyword lists per advertiser."]
    pub negative_keyword_lists_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negativelyTargetedChannelsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of negatively targeted channels created under this advertiser. These negatively targeted channels count towards the limit of 5 negatively targeted channels per advertiser."]
    pub negatively_targeted_channels_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usedCampaignsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of ACTIVE and PAUSED campaigns under this advertiser. These campaigns count towards the limit of 9999 campaigns per advertiser."]
    pub used_campaigns_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usedInsertionOrdersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of ACTIVE, PAUSED and DRAFT insertion orders under this advertiser. These insertion orders count towards the limit of 9999 insertion orders per advertiser."]
    pub used_insertion_orders_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usedLineItemsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of ACTIVE, PAUSED, and DRAFT line items under this advertiser. These line items count towards the limit of 9999 line items per advertiser."]
    pub used_line_items_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an assigned authorized seller status. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`."]
pub struct AuthorizedSellerStatusAssignedTargetingOptionDetails {
    #[serde(rename = "authorizedSellerStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The authorized seller status to target."]
    pub authorized_seller_status: ::std::option::Option<
        AuthorizedSellerStatusAssignedTargetingOptionDetailsAuthorizedSellerStatusEnum,
    >,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The authorized seller status to target."]
pub enum AuthorizedSellerStatusAssignedTargetingOptionDetailsAuthorizedSellerStatusEnum {
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_UNSPECIFIED")]
    #[doc = "Default value when authorized seller status is not specified in this version. This enum is a placeholder for default value and does not represent a real authorized seller status option."]
    AuthorizedSellerStatusUnspecified,
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY")]
    #[doc = "Only authorized sellers that directly own the inventory being monetized, as indicated by a DIRECT declaration in the ads.txt file."]
    AuthorizedSellerStatusAuthorizedDirectSellersOnly,
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS")]
    #[doc = "All authorized sellers, including publishers that have not posted an ads.txt file. Display & Video 360 automatically disallows unauthorized sellers."]
    AuthorizedSellerStatusAuthorizedAndNonParticipatingPublishers,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable authorized seller status. This will be populated in the authorized_seller_status_details field when targeting_type is `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS`."]
pub struct AuthorizedSellerStatusTargetingOptionDetails {
    #[serde(rename = "authorizedSellerStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The authorized seller status."]
    pub authorized_seller_status: ::std::option::Option<
        AuthorizedSellerStatusTargetingOptionDetailsAuthorizedSellerStatusEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The authorized seller status."]
pub enum AuthorizedSellerStatusTargetingOptionDetailsAuthorizedSellerStatusEnum {
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_UNSPECIFIED")]
    #[doc = "Default value when authorized seller status is not specified in this version. This enum is a placeholder for default value and does not represent a real authorized seller status option."]
    AuthorizedSellerStatusUnspecified,
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_AUTHORIZED_DIRECT_SELLERS_ONLY")]
    #[doc = "Only authorized sellers that directly own the inventory being monetized, as indicated by a DIRECT declaration in the ads.txt file."]
    AuthorizedSellerStatusAuthorizedDirectSellersOnly,
    #[serde(rename = "AUTHORIZED_SELLER_STATUS_AUTHORIZED_AND_NON_PARTICIPATING_PUBLISHERS")]
    #[doc = "All authorized sellers, including publishers that have not posted an ads.txt file. Display & Video 360 automatically disallows unauthorized sellers."]
    AuthorizedSellerStatusAuthorizedAndNonParticipatingPublishers,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the bid strategy. Bid strategy determines the bid price."]
pub struct BiddingStrategy {
    #[serde(rename = "fixedBid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A strategy that uses a fixed bid price."]
    pub fixed_bid: ::std::option::Option<::std::boxed::Box<FixedBidStrategy>>,
    #[serde(rename = "maximizeSpendAutoBid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A strategy that automatically adjusts the bid to optimize to your performance goal while spending the full budget. At insertion order level, the markup_type of line items cannot be set to `PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM`. In addition, when performance_goal_type is one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED` , the line_item_type of the insertion order line items must be either: * `LINE_ITEM_TYPE_DISPLAY_DEFAULT` * `LINE_ITEM_TYPE_VIDEO_DEFAULT` , and when performance_goal_type is either: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN` the line_item_type of the insertion order line items must be `LINE_ITEM_TYPE_VIDEO_DEFAULT`."]
    pub maximize_spend_auto_bid: ::std::option::Option<::std::boxed::Box<MaximizeSpendBidStrategy>>,
    #[serde(rename = "performanceGoalAutoBid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A strategy that automatically adjusts the bid to meet or beat a specified performance goal. It is to be used only for a line item entity."]
    pub performance_goal_auto_bid:
        ::std::option::Option<::std::boxed::Box<PerformanceGoalBidStrategy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned browser targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_BROWSER`."]
pub struct BrowserAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the browser."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted. All assigned browser targeting options on the same line item must have the same value for this field."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_BROWSER`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable browser. This will be populated in the browser_details field when targeting_type is `TARGETING_TYPE_BROWSER`."]
pub struct BrowserTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the browser."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BulkEditAdvertiserAssignedTargetingOptions."]
pub struct BulkEditAdvertiserAssignedTargetingOptionsRequest {
    #[serde(rename = "createRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"]
    pub create_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<CreateAssignedTargetingOptionsRequest>>,
    >,
    #[serde(rename = "deleteRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"]
    pub delete_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DeleteAssignedTargetingOptionsRequest>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkEditAdvertiserAssignedTargetingOptionsResponse {
    #[serde(rename = "createdAssignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options that have been successfully created. This list will be absent if empty."]
    pub created_assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for AssignedInventorySourceService.BulkEdit."]
pub struct BulkEditAssignedInventorySourcesRequest {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the advertiser that owns the parent inventory source group. The parent partner does not have access to these assigned inventory sources."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAssignedInventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned inventory sources to create in bulk, specified as a list of AssignedInventorySources."]
    pub created_assigned_inventory_sources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedInventorySource>>>,
    #[serde(rename = "deletedAssignedInventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the assigned inventory sources to delete in bulk, specified as a list of assigned_inventory_source_ids."]
    pub deleted_assigned_inventory_sources:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the partner that owns the inventory source group. Only this partner has write access to these assigned inventory sources."]
    pub partner_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for AssignedInventorySourceService.BulkEdit."]
pub struct BulkEditAssignedInventorySourcesResponse {
    #[serde(rename = "assignedInventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned inventory sources that have been successfully created. This list will be absent if empty."]
    pub assigned_inventory_sources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedInventorySource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for AssignedLocationService.BulkEditAssignedLocations."]
pub struct BulkEditAssignedLocationsRequest {
    #[serde(rename = "createdAssignedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned locations to create in bulk, specified as a list of AssignedLocations."]
    pub created_assigned_locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedLocation>>>,
    #[serde(rename = "deletedAssignedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the assigned locations to delete in bulk, specified as a list of assigned_location_ids."]
    pub deleted_assigned_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for AssignedLocationService.BulkEditAssignedLocations."]
pub struct BulkEditAssignedLocationsResponse {
    #[serde(rename = "assignedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned locations that have been successfully created. This list will be absent if empty."]
    pub assigned_locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedLocation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BulkEditAssignedUserRoles."]
pub struct BulkEditAssignedUserRolesRequest {
    #[serde(rename = "createdAssignedUserRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned user roles to create in batch, specified as a list of AssignedUserRoles."]
    pub created_assigned_user_roles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedUserRole>>>,
    #[serde(rename = "deletedAssignedUserRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned user roles to delete in batch, specified as a list of assigned_user_role_ids. The format of assigned_user_role_id is `entityType-entityid`, for example `partner-123`."]
    pub deleted_assigned_user_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkEditAssignedUserRolesResponse {
    #[serde(rename = "createdAssignedUserRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned user roles that have been successfully created. This list will be absent if empty."]
    pub created_assigned_user_roles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedUserRole>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BulkEditLineItemAssignedTargetingOptions."]
pub struct BulkEditLineItemAssignedTargetingOptionsRequest {
    #[serde(rename = "createRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`."]
    pub create_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<CreateAssignedTargetingOptionsRequest>>,
    >,
    #[serde(rename = "deleteRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`."]
    pub delete_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DeleteAssignedTargetingOptionsRequest>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkEditLineItemAssignedTargetingOptionsResponse {
    #[serde(rename = "createdAssignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options that have been successfully created. This list will be absent if empty."]
    pub created_assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for NegativeKeywordService.BulkEditNegativeKeywords."]
pub struct BulkEditNegativeKeywordsRequest {
    #[serde(rename = "createdNegativeKeywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The negative keywords to create in batch, specified as a list of NegativeKeywords."]
    pub created_negative_keywords:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NegativeKeyword>>>,
    #[serde(rename = "deletedNegativeKeywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The negative keywords to delete in batch, specified as a list of keyword_values."]
    pub deleted_negative_keywords: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for NegativeKeywordService.BulkEditNegativeKeywords."]
pub struct BulkEditNegativeKeywordsResponse {
    #[serde(rename = "negativeKeywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of negative keywords that have been successfully created. This list will be absent if empty."]
    pub negative_keywords:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NegativeKeyword>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BulkEditPartnerAssignedTargetingOptions."]
pub struct BulkEditPartnerAssignedTargetingOptionsRequest {
    #[serde(rename = "createRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to create in batch, specified as a list of `CreateAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"]
    pub create_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<CreateAssignedTargetingOptionsRequest>>,
    >,
    #[serde(rename = "deleteRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned targeting options to delete in batch, specified as a list of `DeleteAssignedTargetingOptionsRequest`. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"]
    pub delete_requests: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DeleteAssignedTargetingOptionsRequest>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkEditPartnerAssignedTargetingOptionsResponse {
    #[serde(rename = "createdAssignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options that have been successfully created. This list will be absent if empty."]
    pub created_assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SiteService.BulkEditSites."]
pub struct BulkEditSitesRequest {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the advertiser that owns the parent channel."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdSites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sites to create in batch, specified as a list of Sites."]
    pub created_sites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Site>>>,
    #[serde(rename = "deletedSites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sites to delete in batch, specified as a list of site url_or_app_ids."]
    pub deleted_sites: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the partner that owns the parent channel."]
    pub partner_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SiteService.BulkEditSites."]
pub struct BulkEditSitesResponse {
    #[serde(rename = "sites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of sites that have been successfully created. This list will be absent if empty."]
    pub sites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Site>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkListAdvertiserAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListAdvertiserAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for BulkListInsertionOrderAssignedTargetingOptions."]
pub struct BulkListInsertionOrderAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListInsertionOrderAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkListLineItemAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent BulkListLineItemAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single campaign."]
pub struct Campaign {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser the campaign belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignFlight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The planned spend and duration of the campaign."]
    pub campaign_flight: ::std::option::Option<::std::boxed::Box<CampaignFlight>>,
    #[serde(rename = "campaignGoal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The goal of the campaign."]
    pub campaign_goal: ::std::option::Option<::std::boxed::Box<CampaignGoal>>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the campaign. Assigned by the system."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the campaign. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Controls whether or not the insertion orders under this campaign can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. * For CreateCampaign method, `ENTITY_STATUS_ARCHIVED` is not allowed."]
    pub entity_status: ::std::option::Option<CampaignEntityStatusEnum>,
    #[serde(rename = "frequencyCap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The frequency cap setting of the campaign."]
    pub frequency_cap: ::std::option::Option<::std::boxed::Box<FrequencyCap>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the campaign."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the campaign was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Controls whether or not the insertion orders under this campaign can spend their budgets and bid on inventory. * Accepted values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. * For CreateCampaign method, `ENTITY_STATUS_ARCHIVED` is not allowed."]
pub enum CampaignEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that track the planned spend and duration of a campaign."]
pub struct CampaignFlight {
    #[serde(rename = "plannedDates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The dates that the campaign is expected to run. They are resolved relative to the parent advertiser's time zone. * The dates specified here will not affect serving. They are used to generate alerts and warnings. For example, if the flight date of any child insertion order is outside the range of these dates, the user interface will show a warning. * `start_date` is required and must be the current date or later. * `end_date` is optional. If specified, it must be the `start_date` or later. * Any specified date must be before the year 2037."]
    pub planned_dates: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "plannedSpendAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount the campaign is expected to spend for its given planned_dates. This will not limit serving, but will be used for tracking spend in the DV360 UI. The amount is in micros. Must be greater than or equal to 0. For example, 500000000 represents 500 standard units of the currency."]
    pub planned_spend_amount_micros: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the goal of a campaign."]
pub struct CampaignGoal {
    #[serde(rename = "campaignGoalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the campaign goal."]
    pub campaign_goal_type: ::std::option::Option<CampaignGoalCampaignGoalTypeEnum>,
    #[serde(rename = "performanceGoal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The performance goal of the campaign. Acceptable values for performance_goal_type are: * `PERFORMANCE_GOAL_TYPE_CPM` * `PERFORMANCE_GOAL_TYPE_CPC` * `PERFORMANCE_GOAL_TYPE_CPA` * `PERFORMANCE_GOAL_TYPE_CPIAVC` * `PERFORMANCE_GOAL_TYPE_CTR` * `PERFORMANCE_GOAL_TYPE_VIEWABILITY` * `PERFORMANCE_GOAL_TYPE_OTHER`"]
    pub performance_goal: ::std::option::Option<::std::boxed::Box<PerformanceGoal>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the campaign goal."]
pub enum CampaignGoalCampaignGoalTypeEnum {
    #[serde(rename = "CAMPAIGN_GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Goal value is not specified or unknown in this version."]
    CampaignGoalTypeUnspecified,
    #[serde(rename = "CAMPAIGN_GOAL_TYPE_APP_INSTALL")]
    #[doc = "Drive app installs or engagements."]
    CampaignGoalTypeAppInstall,
    #[serde(rename = "CAMPAIGN_GOAL_TYPE_BRAND_AWARENESS")]
    #[doc = "Raise awareness of a brand or product."]
    CampaignGoalTypeBrandAwareness,
    #[serde(rename = "CAMPAIGN_GOAL_TYPE_OFFLINE_ACTION")]
    #[doc = "Drive offline or in-store sales."]
    CampaignGoalTypeOfflineAction,
    #[serde(rename = "CAMPAIGN_GOAL_TYPE_ONLINE_ACTION")]
    #[doc = "Drive online action or visits."]
    CampaignGoalTypeOnlineAction,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned carrier and ISP targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_CARRIER_AND_ISP`."]
pub struct CarrierAndIspAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the carrier or ISP."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted. All assigned carrier and ISP targeting options on the same line item must have the same value for this field."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_CARRIER_AND_ISP`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable carrier or ISP. This will be populated in the carrier_and_isp_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_CARRIER_AND_ISP`."]
pub struct CarrierAndIspTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the carrier or ISP."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type indicating if it's carrier or ISP."]
    pub _type: ::std::option::Option<CarrierAndIspTargetingOptionDetailsTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type indicating if it's carrier or ISP."]
pub enum CarrierAndIspTargetingOptionDetailsTypeEnum {
    #[serde(rename = "CARRIER_AND_ISP_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown in this version."]
    CarrierAndIspTypeUnspecified,
    #[serde(rename = "CARRIER_AND_ISP_TYPE_ISP")]
    #[doc = "Indicates this targeting resource refers to an ISP."]
    CarrierAndIspTypeIsp,
    #[serde(rename = "CARRIER_AND_ISP_TYPE_CARRIER")]
    #[doc = "Indicates this targeting resource refers to a mobile carrier."]
    CarrierAndIspTypeCarrier,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned category targeting option details. This will be populated in the category_details field when targeting_type is `TARGETING_TYPE_CATEGORY`."]
pub struct CategoryAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the category."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CATEGORY`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable category. This will be populated in the category_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_CATEGORY`."]
pub struct CategoryTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the category."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single channel. Channels are custom groups of related websites and apps."]
pub struct Channel {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the advertiser that owns the channel."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the channel. Assigned by the system."]
    pub channel_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the channel. Must be UTF-8 encoded with a maximum length of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the channel."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the partner that owns the channel."]
    pub partner_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned channel targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_CHANNEL`."]
pub struct ChannelAssignedTargetingOptionDetails {
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the channel. Should refer to the channel ID field on a [Partner-owned channel](partners.channels#Channel.FIELDS.channel_id) or [advertiser-owned channel](advertisers.channels#Channel.FIELDS.channel_id) resource."]
    pub channel_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted. For advertiser level assigned targeting option, this field must be true."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for advertisers that use both Campaign Manager 360 (CM360) and third-party ad servers."]
pub struct CmHybridConfig {
    #[serde(rename = "cmAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Account ID of the CM360 Floodlight configuration linked with the DV360 advertiser."]
    pub cm_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cmFloodlightConfigId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. ID of the CM360 Floodlight configuration linked with the DV360 advertiser."]
    pub cm_floodlight_config_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cmFloodlightLinkingAuthorized")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. By setting this field to `true`, you, on behalf of your company, authorize the sharing of information from the given Floodlight configuration to this Display & Video 360 advertiser."]
    pub cm_floodlight_linking_authorized: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "cmSyncableSiteIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of CM360 sites whose placements will be synced to DV360 as creatives. If absent or empty in CreateAdvertiser method, the system will automatically create a CM360 site. Removing sites from this list may cause DV360 creatives synced from CM360 to be deleted. At least one site must be specified."]
    pub cm_syncable_site_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dv360ToCmCostReportingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to report DV360 cost to CM360."]
    pub dv360_to_cm_cost_reporting_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dv360ToCmDataSharingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to include DV360 data in CM360 data transfer reports."]
    pub dv360_to_cm_data_sharing_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Campaign Manager 360 tracking ad."]
pub struct CmTrackingAd {
    #[serde(rename = "cmAdId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ad ID of the campaign manager 360 tracking Ad."]
    pub cm_ad_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cmCreativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creative ID of the campaign manager 360 tracking Ad."]
    pub cm_creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cmPlacementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The placement ID of the campaign manager 360 tracking Ad."]
    pub cm_placement_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a combined audience resource."]
pub struct CombinedAudience {
    #[serde(rename = "combinedAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the combined audience. Assigned by the system."]
    pub combined_audience_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the combined audience. ."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the combined audience."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of combined audience group. All combined audience targeting settings are logically ‘OR’ of each other."]
pub struct CombinedAudienceGroup {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All combined audience targeting settings in combined audience group. Repeated settings with same id will be ignored. The number of combined audience settings should be no more than five, error will be thrown otherwise."]
    pub settings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CombinedAudienceTargetingSetting>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of combined audience targeting setting."]
pub struct CombinedAudienceTargetingSetting {
    #[serde(rename = "combinedAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Combined audience id of combined audience targeting setting. This id is combined_audience_id."]
    pub combined_audience_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned content instream position targeting option details. This will be populated in the content_instream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`."]
pub struct ContentInstreamPositionAssignedTargetingOptionDetails {
    #[serde(rename = "adType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`. * `AD_TYPE_AUDIO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_AUDIO_DEFAULT`."]
    pub ad_type:
        ::std::option::Option<ContentInstreamPositionAssignedTargetingOptionDetailsAdTypeEnum>,
    #[serde(rename = "contentInstreamPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The content instream position for video or audio ads."]
    pub content_instream_position: ::std::option::Option<
        ContentInstreamPositionAssignedTargetingOptionDetailsContentInstreamPositionEnum,
    >,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`. * `AD_TYPE_AUDIO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_AUDIO_DEFAULT`."]
pub enum ContentInstreamPositionAssignedTargetingOptionDetailsAdTypeEnum {
    #[serde(rename = "AD_TYPE_UNSPECIFIED")]
    #[doc = "Ad type is not specified or is unknown in this version."]
    AdTypeUnspecified,
    #[serde(rename = "AD_TYPE_DISPLAY")]
    #[doc = "Display creatives, e.g. image and HTML5."]
    AdTypeDisplay,
    #[serde(rename = "AD_TYPE_VIDEO")]
    #[doc = "Video creatives, e.g. video ads that play during streaming content in video players."]
    AdTypeVideo,
    #[serde(rename = "AD_TYPE_AUDIO")]
    #[doc = "Audio creatives, e.g. audio ads that play during audio content."]
    AdTypeAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The content instream position for video or audio ads."]
pub enum ContentInstreamPositionAssignedTargetingOptionDetailsContentInstreamPositionEnum {
    #[serde(rename = "CONTENT_INSTREAM_POSITION_UNSPECIFIED")]
    #[doc = "Content instream position is not specified in this version. This enum is a place holder for a default value and does not represent a real in stream ad position."]
    ContentInstreamPositionUnspecified,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_PRE_ROLL")]
    #[doc = "Ads that play before streaming content."]
    ContentInstreamPositionPreRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_MID_ROLL")]
    #[doc = "Ads that play between the beginning and end of streaming content."]
    ContentInstreamPositionMidRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_POST_ROLL")]
    #[doc = "Ads that play at the end of streaming content."]
    ContentInstreamPositionPostRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_UNKNOWN")]
    #[doc = "Ads instream position is unknown."]
    ContentInstreamPositionUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable content instream position, which could be used by video and audio ads. This will be populated in the content_instream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_INSTREAM_POSITION`."]
pub struct ContentInstreamPositionTargetingOptionDetails {
    #[serde(rename = "contentInstreamPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The content instream position."]
    pub content_instream_position: ::std::option::Option<
        ContentInstreamPositionTargetingOptionDetailsContentInstreamPositionEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The content instream position."]
pub enum ContentInstreamPositionTargetingOptionDetailsContentInstreamPositionEnum {
    #[serde(rename = "CONTENT_INSTREAM_POSITION_UNSPECIFIED")]
    #[doc = "Content instream position is not specified in this version. This enum is a place holder for a default value and does not represent a real in stream ad position."]
    ContentInstreamPositionUnspecified,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_PRE_ROLL")]
    #[doc = "Ads that play before streaming content."]
    ContentInstreamPositionPreRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_MID_ROLL")]
    #[doc = "Ads that play between the beginning and end of streaming content."]
    ContentInstreamPositionMidRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_POST_ROLL")]
    #[doc = "Ads that play at the end of streaming content."]
    ContentInstreamPositionPostRoll,
    #[serde(rename = "CONTENT_INSTREAM_POSITION_UNKNOWN")]
    #[doc = "Ads instream position is unknown."]
    ContentInstreamPositionUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned content outstream position targeting option details. This will be populated in the content_outstream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`."]
pub struct ContentOutstreamPositionAssignedTargetingOptionDetails {
    #[serde(rename = "adType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`."]
    pub ad_type:
        ::std::option::Option<ContentOutstreamPositionAssignedTargetingOptionDetailsAdTypeEnum>,
    #[serde(rename = "contentOutstreamPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The content outstream position."]
    pub content_outstream_position: ::std::option::Option<
        ContentOutstreamPositionAssignedTargetingOptionDetailsContentOutstreamPositionEnum,
    >,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`."]
pub enum ContentOutstreamPositionAssignedTargetingOptionDetailsAdTypeEnum {
    #[serde(rename = "AD_TYPE_UNSPECIFIED")]
    #[doc = "Ad type is not specified or is unknown in this version."]
    AdTypeUnspecified,
    #[serde(rename = "AD_TYPE_DISPLAY")]
    #[doc = "Display creatives, e.g. image and HTML5."]
    AdTypeDisplay,
    #[serde(rename = "AD_TYPE_VIDEO")]
    #[doc = "Video creatives, e.g. video ads that play during streaming content in video players."]
    AdTypeVideo,
    #[serde(rename = "AD_TYPE_AUDIO")]
    #[doc = "Audio creatives, e.g. audio ads that play during audio content."]
    AdTypeAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The content outstream position."]
pub enum ContentOutstreamPositionAssignedTargetingOptionDetailsContentOutstreamPositionEnum {
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED")]
    #[doc = "Content outstream position is not specified in this version. This enum is a place holder for a default value and does not represent a real content outstream position."]
    ContentOutstreamPositionUnspecified,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_UNKNOWN")]
    #[doc = "The ad position is unknown in the content outstream."]
    ContentOutstreamPositionUnknown,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE")]
    #[doc = "Ads that appear between the paragraphs of your pages."]
    ContentOutstreamPositionInArticle,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_BANNER")]
    #[doc = "Ads that display on the top and the sides of a page."]
    ContentOutstreamPositionInBanner,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_FEED")]
    #[doc = "Ads that appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services)."]
    ContentOutstreamPositionInFeed,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL")]
    #[doc = "Ads shown before or between content loads."]
    ContentOutstreamPositionInterstitial,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable content outstream position, which could be used by display and video ads. This will be populated in the content_outstream_position_details field when targeting_type is `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION`."]
pub struct ContentOutstreamPositionTargetingOptionDetails {
    #[serde(rename = "contentOutstreamPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The content outstream position."]
    pub content_outstream_position: ::std::option::Option<
        ContentOutstreamPositionTargetingOptionDetailsContentOutstreamPositionEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The content outstream position."]
pub enum ContentOutstreamPositionTargetingOptionDetailsContentOutstreamPositionEnum {
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_UNSPECIFIED")]
    #[doc = "Content outstream position is not specified in this version. This enum is a place holder for a default value and does not represent a real content outstream position."]
    ContentOutstreamPositionUnspecified,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_UNKNOWN")]
    #[doc = "The ad position is unknown in the content outstream."]
    ContentOutstreamPositionUnknown,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_ARTICLE")]
    #[doc = "Ads that appear between the paragraphs of your pages."]
    ContentOutstreamPositionInArticle,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_BANNER")]
    #[doc = "Ads that display on the top and the sides of a page."]
    ContentOutstreamPositionInBanner,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_IN_FEED")]
    #[doc = "Ads that appear in a scrollable stream of content. A feed is typically editorial (e.g. a list of articles or news) or listings (e.g. a list of products or services)."]
    ContentOutstreamPositionInFeed,
    #[serde(rename = "CONTENT_OUTSTREAM_POSITION_INTERSTITIAL")]
    #[doc = "Ads shown before or between content loads."]
    ContentOutstreamPositionInterstitial,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how conversions are counted. All post-click conversions will be counted. A percentage value can be set for post-view conversions counting."]
pub struct ConversionCountingConfig {
    #[serde(rename = "floodlightActivityConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Floodlight activity configs used to track conversions. The number of conversions counted is the sum of all of the conversions counted by all of the Floodlight activity IDs specified in this field."]
    pub floodlight_activity_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrackingFloodlightActivityConfig>>>,
    #[serde(rename = "postViewCountPercentageMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of post-view conversions to count, in millis (1/1000 of a percent). Must be between 0 and 100000 inclusive. For example, to track 50% of the post-click conversions, set a value of 50000."]
    pub post_view_count_percentage_millis: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Counter event of the creative."]
pub struct CounterEvent {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the counter event."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportingName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name used to identify this counter event in reports."]
    pub reporting_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request message for CreateAsset."]
pub struct CreateAssetRequest {
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The filename of the asset, including the file extension. The filename must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub filename: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for CreateAsset."]
pub struct CreateAssetResponse {
    #[serde(rename = "asset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uploaded asset, if successful."]
    pub asset: ::std::option::Option<::std::boxed::Box<Asset>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request listing which assigned targeting options of a given targeting type should be created and added."]
pub struct CreateAssignedTargetingOptionsRequest {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The assigned targeting options to create and add."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "targetingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifies the type of this assigned targeting option."]
    pub targeting_type:
        ::std::option::Option<CreateAssignedTargetingOptionsRequestTargetingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Identifies the type of this assigned targeting option."]
pub enum CreateAssignedTargetingOptionsRequestTargetingTypeEnum {
    #[serde(rename = "TARGETING_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown in this version."]
    TargetingTypeUnspecified,
    #[serde(rename = "TARGETING_TYPE_CHANNEL")]
    #[doc = "Target a channel (a custom group of related websites or apps)."]
    TargetingTypeChannel,
    #[serde(rename = "TARGETING_TYPE_APP_CATEGORY")]
    #[doc = "Target an app category (for example, education or puzzle games)."]
    TargetingTypeAppCategory,
    #[serde(rename = "TARGETING_TYPE_APP")]
    #[doc = "Target a specific app (for example, Angry Birds)."]
    TargetingTypeApp,
    #[serde(rename = "TARGETING_TYPE_URL")]
    #[doc = "Target a specific url (for example, quora.com)."]
    TargetingTypeUrl,
    #[serde(rename = "TARGETING_TYPE_DAY_AND_TIME")]
    #[doc = "Target ads during a chosen time period on a specific day."]
    TargetingTypeDayAndTime,
    #[serde(rename = "TARGETING_TYPE_AGE_RANGE")]
    #[doc = "Target ads to a specific age range (for example, 18-24)."]
    TargetingTypeAgeRange,
    #[serde(rename = "TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    #[doc = "Target ads to the specified regions on a regional location list."]
    TargetingTypeRegionalLocationList,
    #[serde(rename = "TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    #[doc = "Target ads to the specified points of interest on a proximity location list."]
    TargetingTypeProximityLocationList,
    #[serde(rename = "TARGETING_TYPE_GENDER")]
    #[doc = "Target ads to a specific gender (for example, female or male)."]
    TargetingTypeGender,
    #[serde(rename = "TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    #[doc = "Target a specific video player size for video ads."]
    TargetingTypeVideoPlayerSize,
    #[serde(rename = "TARGETING_TYPE_USER_REWARDED_CONTENT")]
    #[doc = "Target user rewarded content for video ads."]
    TargetingTypeUserRewardedContent,
    #[serde(rename = "TARGETING_TYPE_PARENTAL_STATUS")]
    #[doc = "Target ads to a specific parental status (for example, parent or not a parent)."]
    TargetingTypeParentalStatus,
    #[serde(rename = "TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    #[doc = "Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll)."]
    TargetingTypeContentInstreamPosition,
    #[serde(rename = "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    #[doc = "Target ads in a specific content outstream position."]
    TargetingTypeContentOutstreamPosition,
    #[serde(rename = "TARGETING_TYPE_DEVICE_TYPE")]
    #[doc = "Target ads to a specific device type (for example, tablet or connected TV)."]
    TargetingTypeDeviceType,
    #[serde(rename = "TARGETING_TYPE_AUDIENCE_GROUP")]
    #[doc = "Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time."]
    TargetingTypeAudienceGroup,
    #[serde(rename = "TARGETING_TYPE_BROWSER")]
    #[doc = "Target ads to specific web browsers (for example, Chrome)."]
    TargetingTypeBrowser,
    #[serde(rename = "TARGETING_TYPE_HOUSEHOLD_INCOME")]
    #[doc = "Target ads to a specific household income range (for example, top 10%)."]
    TargetingTypeHouseholdIncome,
    #[serde(rename = "TARGETING_TYPE_ON_SCREEN_POSITION")]
    #[doc = "Target ads in a specific on screen position."]
    TargetingTypeOnScreenPosition,
    #[serde(rename = "TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    #[doc = "Filter web sites through third party verification (for example, IAS or DoubleVerify)."]
    TargetingTypeThirdPartyVerifier,
    #[serde(rename = "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    #[doc = "Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences)."]
    TargetingTypeDigitalContentLabelExclusion,
    #[serde(rename = "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    #[doc = "Filter website content by sensitive categories (for example, adult)."]
    TargetingTypeSensitiveCategoryExclusion,
    #[serde(rename = "TARGETING_TYPE_ENVIRONMENT")]
    #[doc = "Target ads to a specific environment (for example, web or app)."]
    TargetingTypeEnvironment,
    #[serde(rename = "TARGETING_TYPE_CARRIER_AND_ISP")]
    #[doc = "Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange)."]
    TargetingTypeCarrierAndIsp,
    #[serde(rename = "TARGETING_TYPE_OPERATING_SYSTEM")]
    #[doc = "Target ads to a specific operating system (for example, macOS)."]
    TargetingTypeOperatingSystem,
    #[serde(rename = "TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    #[doc = "Target ads to a specific device make or model (for example, Roku or Samsung)."]
    TargetingTypeDeviceMakeModel,
    #[serde(rename = "TARGETING_TYPE_KEYWORD")]
    #[doc = "Target ads to a specific keyword (for example, dog or retriever)."]
    TargetingTypeKeyword,
    #[serde(rename = "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    #[doc = "Target ads to a specific negative keyword list."]
    TargetingTypeNegativeKeywordList,
    #[serde(rename = "TARGETING_TYPE_VIEWABILITY")]
    #[doc = "Target ads to a specific viewability (for example, 80% viewable)."]
    TargetingTypeViewability,
    #[serde(rename = "TARGETING_TYPE_CATEGORY")]
    #[doc = "Target ads to a specific content category (for example, arts & entertainment)."]
    TargetingTypeCategory,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE")]
    #[doc = "Purchase impressions from specific deals and auction packages."]
    TargetingTypeInventorySource,
    #[serde(rename = "TARGETING_TYPE_LANGUAGE")]
    #[doc = "Target ads to a specific language (for example, English or Japanese)."]
    TargetingTypeLanguage,
    #[serde(rename = "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    #[doc = "Target ads to ads.txt authorized sellers."]
    TargetingTypeAuthorizedSellerStatus,
    #[serde(rename = "TARGETING_TYPE_GEO_REGION")]
    #[doc = "Target ads to a specific regional location (for example, a city or state)."]
    TargetingTypeGeoRegion,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    #[doc = "Purchase impressions from a group of deals and auction packages."]
    TargetingTypeInventorySourceGroup,
    #[serde(rename = "TARGETING_TYPE_EXCHANGE")]
    #[doc = "Purchase impressions from specific exchanges."]
    TargetingTypeExchange,
    #[serde(rename = "TARGETING_TYPE_SUB_EXCHANGE")]
    #[doc = "Purchase impressions from specific sub-exchanges."]
    TargetingTypeSubExchange,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for [SdfDownloadTaskService.CreateSdfDownloadTask]."]
pub struct CreateSdfDownloadTaskRequest {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the advertiser to download SDF for."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters on entities by their entity IDs."]
    pub id_filter: ::std::option::Option<::std::boxed::Box<IdFilter>>,
    #[serde(rename = "inventorySourceFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters on Inventory Sources by their IDs."]
    pub inventory_source_filter: ::std::option::Option<::std::boxed::Box<InventorySourceFilter>>,
    #[serde(rename = "parentEntityFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters on selected file types. The entities in each file are filtered by a chosen set of filter entities. The filter entities must be the same type as, or a parent type of, the selected file types."]
    pub parent_entity_filter: ::std::option::Option<::std::boxed::Box<ParentEntityFilter>>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the partner to download SDF for."]
    pub partner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The SDF version of the downloaded file. If set to `SDF_VERSION_UNSPECIFIED`, this will default to the version specified by the advertiser or partner identified by `root_id`. An advertiser inherits its SDF version from its partner unless configured otherwise."]
    pub version: ::std::option::Option<CreateSdfDownloadTaskRequestVersionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The SDF version of the downloaded file. If set to `SDF_VERSION_UNSPECIFIED`, this will default to the version specified by the advertiser or partner identified by `root_id`. An advertiser inherits its SDF version from its partner unless configured otherwise."]
pub enum CreateSdfDownloadTaskRequestVersionEnum {
    #[serde(rename = "SDF_VERSION_UNSPECIFIED")]
    #[doc = "SDF version value is not specified or is unknown in this version."]
    SdfVersionUnspecified,
    #[serde(rename = "SDF_VERSION_3_1")]
    #[doc = "SDF version 3.1"]
    SdfVersion31,
    #[serde(rename = "SDF_VERSION_4")]
    #[doc = "SDF version 4"]
    SdfVersion4,
    #[serde(rename = "SDF_VERSION_4_1")]
    #[doc = "SDF version 4.1"]
    SdfVersion41,
    #[serde(rename = "SDF_VERSION_4_2")]
    #[doc = "SDF version 4.2"]
    SdfVersion42,
    #[serde(rename = "SDF_VERSION_5")]
    #[doc = "SDF version 5."]
    SdfVersion5,
    #[serde(rename = "SDF_VERSION_5_1")]
    #[doc = "SDF version 5.1"]
    SdfVersion51,
    #[serde(rename = "SDF_VERSION_5_2")]
    #[doc = "SDF version 5.2"]
    SdfVersion52,
    #[serde(rename = "SDF_VERSION_5_3")]
    #[doc = "SDF version 5.3"]
    SdfVersion53,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single Creative."]
pub struct Creative {
    #[serde(rename = "additionalDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional dimensions. Applicable when creative_type is one of: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE` * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_LIGHTBOX` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_PUBLISHER_HOSTED` If this field is specified, width_pixels and height_pixels are both required and must be greater than or equal to 0."]
    pub additional_dimensions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimensions>>>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser the creative belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appendedTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party HTML tracking tag to be appended to the creative tag."]
    pub appended_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "assets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Assets associated to this creative. Assets can be associated to the creative in one of following roles: * `ASSET_ROLE_UNSPECIFIED` * `ASSET_ROLE_MAIN` * `ASSET_ROLE_BACKUP` * `ASSET_ROLE_POLITE_LOAD`"]
    pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssetAssociation>>>,
    #[serde(rename = "cmPlacementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the Campaign Manager 360 placement associated with the creative. This field is only applicable for creatives that are synced from Campaign Manager."]
    pub cm_placement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cmTrackingAd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Campaign Manager 360 tracking ad associated with the creative. Optional for the following creative_type when created by an advertiser that uses both Campaign Manager 360 and third-party ad serving: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` Output only for other cases."]
    pub cm_tracking_ad: ::std::option::Option<::std::boxed::Box<CmTrackingAd>>,
    #[serde(rename = "companionCreativeIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of companion creatives for a video creative. You can assign existing display creatives (with image or HTML5 assets) to serve surrounding the publisher's video player. Companions display around the video player while the video is playing and remain after the video has completed. Creatives contain additional dimensions can not be companion creatives. This field is only supported for following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`"]
    pub companion_creative_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "counterEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Counter events for a rich media creative. Counters track the number of times that a user interacts with any part of a rich media creative in a specified way (mouse-overs, mouse-outs, clicks, taps, data loading, keyboard entries, etc.). Any event that can be captured in the creative can be recorded as a counter. Leave it empty or unset for creatives containing image assets only."]
    pub counter_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CounterEvent>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the creative was created. Assigned by the system."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of attributes of the creative that is generated by the system."]
    pub creative_attributes: ::std::option::Option<::std::vec::Vec<CreativeCreativeAttributesEnum>>,
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the creative. Assigned by the system."]
    pub creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The type of the creative."]
    pub creative_type: ::std::option::Option<CreativeCreativeTypeEnum>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Primary dimensions of the creative. Applicable to all creative types. The value of width_pixels and height_pixels defaults to `0` when creative_type is one of: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL` * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO`"]
    pub dimensions: ::std::option::Option<::std::boxed::Box<Dimensions>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the creative. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dynamic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates whether the creative is dynamic."]
    pub dynamic: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Controls whether or not the creative can serve. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED` * `ENTITY_STATUS_PAUSED`"]
    pub entity_status: ::std::option::Option<CreativeEntityStatusEnum>,
    #[serde(rename = "exitEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Exit events for this creative. An exit (also known as a click tag) is any area in your creative that someone can click or tap to open an advertiser's landing page. Every creative must include at least one exit. You can add an exit to your creative in any of the following ways: * Use Google Web Designer's tap area. * Define a JavaScript variable called \"clickTag\". * Use the Enabler (Enabler.exit()) to track exits in rich media formats."]
    pub exit_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExitEvent>>>,
    #[serde(rename = "expandOnHover")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates the creative will automatically expand on hover. Optional and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`"]
    pub expand_on_hover: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandingDirection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the expanding direction of the creative. Required and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`"]
    pub expanding_direction: ::std::option::Option<CreativeExpandingDirectionEnum>,
    #[serde(rename = "hostingSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates where the creative is hosted."]
    pub hosting_source: ::std::option::Option<CreativeHostingSourceEnum>,
    #[serde(rename = "html5Video")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the third-party VAST tag creative requires HTML5 Video support. Output only and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub html5_video: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "iasCampaignMonitoring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether Integral Ad Science (IAS) campaign monitoring is enabled. To enable this for the creative, make sure the Advertiser.creative_config.ias_client_id has been set to your IAS client ID."]
    pub ias_campaign_monitoring: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "integrationCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID information used to link this creative to an external system. Must be UTF-8 encoded with a length of no more than 10,000 characters."]
    pub integration_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jsTrackerUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JavaScript measurement URL from supported third-party verification providers (ComScore, DoubleVerify, IAS, Moat). HTML script tags are not supported. This field is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    pub js_tracker_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItemIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The IDs of the line items this creative is associated with. To associate a creative to a line item, use LineItem.creative_ids instead."]
    pub line_item_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mediaDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Media duration of the creative. Applicable when creative_type is one of: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_PUBLISHER_HOSTED`"]
    pub media_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mp3Audio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the third-party audio creative supports MP3. Output only and only valid for third-party audio creatives. Third-party audio creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO`"]
    pub mp3_audio: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the creative."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User notes for this creative. Must be UTF-8 encoded with a length of no more than 20,000 characters."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obaIcon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the OBA icon for a video creative. This field is only supported in following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub oba_icon: ::std::option::Option<::std::boxed::Box<ObaIcon>>,
    #[serde(rename = "oggAudio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the third-party audio creative supports OGG. Output only and only valid for third-party audio creatives. Third-party audio creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO`"]
    pub ogg_audio: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "progressOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play the video before counting a view. This field is required when skippable is true. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub progress_offset: ::std::option::Option<::std::boxed::Box<AudioVideoOffset>>,
    #[serde(rename = "requireHtml5")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that the creative relies on HTML5 to render properly. Optional and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`"]
    pub require_html5: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requireMraid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that the creative requires MRAID (Mobile Rich Media Ad Interface Definitions system). Set this if the creative relies on mobile gestures for interactivity, such as swiping or tapping. Optional and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`"]
    pub require_mraid: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requirePingForAttribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates that the creative will wait for a return ping for attribution. Only valid when using a Campaign Manager 360 tracking ad with a third-party ad server parameter and the ${DC_DBM_TOKEN} macro. Optional and only valid for third-party tag creatives or third-party VAST tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE` Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`"]
    pub require_ping_for_attribution: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reviewStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current status of the creative review process."]
    pub review_status: ::std::option::Option<::std::boxed::Box<ReviewStatusInfo>>,
    #[serde(rename = "skipOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play the video before the skip button appears. This field is required when skippable is true. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub skip_offset: ::std::option::Option<::std::boxed::Box<AudioVideoOffset>>,
    #[serde(rename = "skippable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can choose to skip a video creative. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub skippable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "thirdPartyTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The original third-party tag used for the creative. Required and only valid for third-party tag creatives. Third-party tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_EXPANDABLE`"]
    pub third_party_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirdPartyUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tracking URLs from third parties to track interactions with a video creative. This field is only supported for the following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    pub third_party_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThirdPartyUrl>>>,
    #[serde(rename = "timerEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timer custom events for a rich media creative. Timers track the time during which a user views and interacts with a specified part of a rich media creative. A creative can have multiple timer events, each timed independently. Leave it empty or unset for creatives containing image assets only."]
    pub timer_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimerEvent>>>,
    #[serde(rename = "trackerUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tracking URLs for analytics providers or third-party ad technology vendors. The URLs must start with https (except on inventory that doesn't require SSL compliance). If using macros in your URL, use only macros supported by Display & Video 360. Standard URLs only, no IMG or SCRIPT tags. This field is only supported in following creative_type: * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO`"]
    pub tracker_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "transcodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Audio/Video transcodes. Display & Video 360 transcodes the main asset into a number of alternative versions that use different file formats or have different properties (resolution, audio bit rate, and video bit rate), each designed for specific video players or bandwidths. These transcodes give a publisher's system more options to choose from for each impression on your video and ensures that the appropriate file serves based on the viewer’s connection and screen size. This field is only supported in following creative_type: * `CREATIVE_TYPE_VIDEO` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_AUDIO`"]
    pub transcodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Transcode>>>,
    #[serde(rename = "universalAdId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. An optional creative identifier provided by a registry that is unique across all platforms. Universal Ad ID is part of the VAST 4.0 standard. It can be modified after the creative is created. This field is only supported for the following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub universal_ad_id: ::std::option::Option<::std::boxed::Box<UniversalAdId>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the creative was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vastTagUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The URL of the VAST tag for a third-party VAST tag creative. Required and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_VIDEO`"]
    pub vast_tag_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vpaid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the third-party VAST tag creative requires VPAID (Digital Video Player-Ad Interface). Output only and only valid for third-party VAST tag creatives. Third-party VAST tag creatives are creatives with following hosting_source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_VIDEO`"]
    pub vpaid: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeCreativeAttributesEnum {
    #[serde(rename = "CREATIVE_ATTRIBUTE_UNSPECIFIED")]
    #[doc = "The creative attribute is not specified or is unknown in this version."]
    CreativeAttributeUnspecified,
    #[serde(rename = "CREATIVE_ATTRIBUTE_VAST")]
    #[doc = "The creative is a VAST creative."]
    CreativeAttributeVast,
    #[serde(rename = "CREATIVE_ATTRIBUTE_VPAID_LINEAR")]
    #[doc = "The creative is a linear VPAID creative."]
    CreativeAttributeVpaidLinear,
    #[serde(rename = "CREATIVE_ATTRIBUTE_VPAID_NON_LINEAR")]
    #[doc = "The creative is a non-linear VPAID creative."]
    CreativeAttributeVpaidNonLinear,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The type of the creative."]
pub enum CreativeCreativeTypeEnum {
    #[serde(rename = "CREATIVE_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    CreativeTypeUnspecified,
    #[serde(rename = "CREATIVE_TYPE_STANDARD")]
    #[doc = "Standard display creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeStandard,
    #[serde(rename = "CREATIVE_TYPE_EXPANDABLE")]
    #[doc = "Expandable creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeExpandable,
    #[serde(rename = "CREATIVE_TYPE_VIDEO")]
    #[doc = "Video creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeVideo,
    #[serde(rename = "CREATIVE_TYPE_NATIVE")]
    #[doc = "Native creative rendered by publishers with assets from advertiser. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNative,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL")]
    #[doc = "Templated app install mobile creative (banner). Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstall,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_SITE_SQUARE")]
    #[doc = "Square native creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeSiteSquare,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL")]
    #[doc = "Interstitial creative including both display and video. Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstallInterstitial,
    #[serde(rename = "CREATIVE_TYPE_LIGHTBOX")]
    #[doc = "Responsive and expandable Lightbox creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypeLightbox,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_APP_INSTALL")]
    #[doc = "Native app install creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeAppInstall,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE")]
    #[doc = "Square native app install creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeAppInstallSquare,
    #[serde(rename = "CREATIVE_TYPE_AUDIO")]
    #[doc = "Audio creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeAudio,
    #[serde(rename = "CREATIVE_TYPE_PUBLISHER_HOSTED")]
    #[doc = "Publisher hosted creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypePublisherHosted,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_VIDEO")]
    #[doc = "Native video creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeVideo,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO")]
    #[doc = "Templated app install mobile video creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstallVideo,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Controls whether or not the creative can serve. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED` * `ENTITY_STATUS_PAUSED`"]
pub enum CreativeEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Specifies the expanding direction of the creative. Required and only valid for third-party expandable creatives. Third-party expandable creatives are creatives with following hosting source: * `HOSTING_SOURCE_THIRD_PARTY` combined with following creative_type: * `CREATIVE_TYPE_EXPANDABLE`"]
pub enum CreativeExpandingDirectionEnum {
    #[serde(rename = "EXPANDING_DIRECTION_UNSPECIFIED")]
    #[doc = "The expanding direction is not specified."]
    ExpandingDirectionUnspecified,
    #[serde(rename = "EXPANDING_DIRECTION_NONE")]
    #[doc = "Does not expand in any direction."]
    ExpandingDirectionNone,
    #[serde(rename = "EXPANDING_DIRECTION_UP")]
    #[doc = "Expands up."]
    ExpandingDirectionUp,
    #[serde(rename = "EXPANDING_DIRECTION_DOWN")]
    #[doc = "Expands down."]
    ExpandingDirectionDown,
    #[serde(rename = "EXPANDING_DIRECTION_LEFT")]
    #[doc = "Expands left."]
    ExpandingDirectionLeft,
    #[serde(rename = "EXPANDING_DIRECTION_RIGHT")]
    #[doc = "Expands right."]
    ExpandingDirectionRight,
    #[serde(rename = "EXPANDING_DIRECTION_UP_AND_LEFT")]
    #[doc = "Expands up and to the left side."]
    ExpandingDirectionUpAndLeft,
    #[serde(rename = "EXPANDING_DIRECTION_UP_AND_RIGHT")]
    #[doc = "Expands up and to the right side."]
    ExpandingDirectionUpAndRight,
    #[serde(rename = "EXPANDING_DIRECTION_DOWN_AND_LEFT")]
    #[doc = "Expands down and to the left side."]
    ExpandingDirectionDownAndLeft,
    #[serde(rename = "EXPANDING_DIRECTION_DOWN_AND_RIGHT")]
    #[doc = "Expands down and to the right side."]
    ExpandingDirectionDownAndRight,
    #[serde(rename = "EXPANDING_DIRECTION_UP_OR_DOWN")]
    #[doc = "Expands either up or down."]
    ExpandingDirectionUpOrDown,
    #[serde(rename = "EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
    #[doc = "Expands to either the left or the right side."]
    ExpandingDirectionLeftOrRight,
    #[serde(rename = "EXPANDING_DIRECTION_ANY_DIAGONAL")]
    #[doc = "Can expand in any diagonal direction."]
    ExpandingDirectionAnyDiagonal,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Indicates where the creative is hosted."]
pub enum CreativeHostingSourceEnum {
    #[serde(rename = "HOSTING_SOURCE_UNSPECIFIED")]
    #[doc = "Hosting source is not specified or is unknown in this version."]
    HostingSourceUnspecified,
    #[serde(rename = "HOSTING_SOURCE_CM")]
    #[doc = "A creative synced from Campaign Manager 360. Create and update methods are **not** supported for this hosting type."]
    HostingSourceCm,
    #[serde(rename = "HOSTING_SOURCE_THIRD_PARTY")]
    #[doc = "A creative hosted by a third-party ad server (3PAS). Create and update methods are supported for this hosting type if the creative_type is one of the following: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_EXPANDABLE` * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`"]
    HostingSourceThirdParty,
    #[serde(rename = "HOSTING_SOURCE_HOSTED")]
    #[doc = "A creative created in DV360 and hosted by Campaign Manager 360. Create and update methods are supported for this hosting type if the creative_type is one of the following: * `CREATIVE_TYPE_AUDIO` * `CREATIVE_TYPE_NATIVE` * `CREATIVE_TYPE_NATIVE_APP_INSTALL` * `CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE` * `CREATIVE_TYPE_NATIVE_SITE_SQUARE` * `CREATIVE_TYPE_NATIVE_VIDEO` * `CREATIVE_TYPE_STANDARD` * `CREATIVE_TYPE_VIDEO`"]
    HostingSourceHosted,
    #[serde(rename = "HOSTING_SOURCE_RICH_MEDIA")]
    #[doc = "A rich media creative created in Studio and hosted by Campaign Manager 360. Create and update methods are **not** supported for this hosting type."]
    HostingSourceRichMedia,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative requirements configuration for the inventory source."]
pub struct CreativeConfig {
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of creative that can be assigned to the inventory source."]
    pub creative_type: ::std::option::Option<CreativeConfigCreativeTypeEnum>,
    #[serde(rename = "displayCreativeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for display creatives. Applicable when creative_type is `CREATIVE_TYPE_STANDARD`."]
    pub display_creative_config:
        ::std::option::Option<::std::boxed::Box<InventorySourceDisplayCreativeConfig>>,
    #[serde(rename = "videoCreativeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for video creatives. Applicable when creative_type is `CREATIVE_TYPE_VIDEO`."]
    pub video_creative_config:
        ::std::option::Option<::std::boxed::Box<InventorySourceVideoCreativeConfig>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of creative that can be assigned to the inventory source."]
pub enum CreativeConfigCreativeTypeEnum {
    #[serde(rename = "CREATIVE_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    CreativeTypeUnspecified,
    #[serde(rename = "CREATIVE_TYPE_STANDARD")]
    #[doc = "Standard display creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeStandard,
    #[serde(rename = "CREATIVE_TYPE_EXPANDABLE")]
    #[doc = "Expandable creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeExpandable,
    #[serde(rename = "CREATIVE_TYPE_VIDEO")]
    #[doc = "Video creative. Create and update methods are supported for this creative type if the hosting_source is one of the following: * `HOSTING_SOURCE_HOSTED` * `HOSTING_SOURCE_THIRD_PARTY`"]
    CreativeTypeVideo,
    #[serde(rename = "CREATIVE_TYPE_NATIVE")]
    #[doc = "Native creative rendered by publishers with assets from advertiser. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNative,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL")]
    #[doc = "Templated app install mobile creative (banner). Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstall,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_SITE_SQUARE")]
    #[doc = "Square native creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeSiteSquare,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_INTERSTITIAL")]
    #[doc = "Interstitial creative including both display and video. Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstallInterstitial,
    #[serde(rename = "CREATIVE_TYPE_LIGHTBOX")]
    #[doc = "Responsive and expandable Lightbox creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypeLightbox,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_APP_INSTALL")]
    #[doc = "Native app install creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeAppInstall,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_APP_INSTALL_SQUARE")]
    #[doc = "Square native app install creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeAppInstallSquare,
    #[serde(rename = "CREATIVE_TYPE_AUDIO")]
    #[doc = "Audio creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeAudio,
    #[serde(rename = "CREATIVE_TYPE_PUBLISHER_HOSTED")]
    #[doc = "Publisher hosted creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypePublisherHosted,
    #[serde(rename = "CREATIVE_TYPE_NATIVE_VIDEO")]
    #[doc = "Native video creative. Create and update methods are supported for this creative type if the hosting_source is `HOSTING_SOURCE_HOSTED`"]
    CreativeTypeNativeVideo,
    #[serde(rename = "CREATIVE_TYPE_TEMPLATED_APP_INSTALL_VIDEO")]
    #[doc = "Templated app install mobile video creative. Create and update methods are **not** supported for this creative type."]
    CreativeTypeTemplatedAppInstallVideo,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single custom bidding algorithm."]
pub struct CustomBiddingAlgorithm {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The unique ID of the advertiser that owns the custom bidding algorithm."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customBiddingAlgorithmId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the custom bidding algorithm. Assigned by the system."]
    pub custom_bidding_algorithm_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customBiddingAlgorithmType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The type of custom bidding algorithm."]
    pub custom_bidding_algorithm_type:
        ::std::option::Option<CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the custom bidding algorithm. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls whether or not the custom bidding algorithm can be used as a bidding strategy. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED`"]
    pub entity_status: ::std::option::Option<CustomBiddingAlgorithmEntityStatusEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the custom bidding algorithm."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The unique ID of the partner that owns the custom bidding algorithm."]
    pub partner_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The type of custom bidding algorithm."]
pub enum CustomBiddingAlgorithmCustomBiddingAlgorithmTypeEnum {
    #[serde(rename = "CUSTOM_BIDDING_ALGORITHM_TYPE_UNSPECIFIED")]
    #[doc = "Algorithm type is not specified or is unknown in this version."]
    CustomBiddingAlgorithmTypeUnspecified,
    #[serde(rename = "SCRIPT_BASED")]
    #[doc = "Algorithm generated through customer-uploaded custom bidding script files."]
    ScriptBased,
    #[serde(rename = "ADS_DATA_HUB_BASED")]
    #[doc = "Algorithm created through Ads Data Hub product."]
    AdsDataHubBased,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Controls whether or not the custom bidding algorithm can be used as a bidding strategy. Accepted values are: * `ENTITY_STATUS_ACTIVE` * `ENTITY_STATUS_ARCHIVED`"]
pub enum CustomBiddingAlgorithmEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a custom list entity, such as a custom affinity or custom intent audience list."]
pub struct CustomList {
    #[serde(rename = "customListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the custom list. Assigned by the system."]
    pub custom_list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the custom list. ."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the custom list."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of custom list group. All custom list targeting settings are logically ‘OR’ of each other."]
pub struct CustomListGroup {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All custom list targeting settings in custom list group. Repeated settings with same id will be ignored."]
    pub settings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomListTargetingSetting>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of custom list targeting setting."]
pub struct CustomListTargetingSetting {
    #[serde(rename = "customListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Custom id of custom list targeting setting. This id is custom_list_id."]
    pub custom_list_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A date range."]
pub struct DateRange {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The upper bound of the date range, inclusive. Must specify a positive value for `year`, `month`, and `day`."]
    pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The lower bound of the date range, inclusive. Must specify a positive value for `year`, `month`, and `day`."]
    pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a segment of time defined on a specific day of the week and with a start and end time. The time represented by `start_hour` must be before the time represented by `end_hour`."]
pub struct DayAndTimeAssignedTargetingOptionDetails {
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The day of the week for this day and time targeting setting."]
    pub day_of_week: ::std::option::Option<DayAndTimeAssignedTargetingOptionDetailsDayOfWeekEnum>,
    #[serde(rename = "endHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The end hour for day and time targeting. Must be between 1 (1 hour after start of day) and 24 (end of day)."]
    pub end_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The start hour for day and time targeting. Must be between 0 (start of day) and 23 (1 hour before end of day)."]
    pub start_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timeZoneResolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The mechanism used to determine which timezone to use for this day and time targeting setting."]
    pub time_zone_resolution:
        ::std::option::Option<DayAndTimeAssignedTargetingOptionDetailsTimeZoneResolutionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The day of the week for this day and time targeting setting."]
pub enum DayAndTimeAssignedTargetingOptionDetailsDayOfWeekEnum {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    #[doc = "The day of the week is unspecified."]
    DayOfWeekUnspecified,
    #[serde(rename = "MONDAY")]
    #[doc = "Monday"]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = "Tuesday"]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = "Wednesday"]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = "Thursday"]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = "Friday"]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = "Saturday"]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = "Sunday"]
    Sunday,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The mechanism used to determine which timezone to use for this day and time targeting setting."]
pub enum DayAndTimeAssignedTargetingOptionDetailsTimeZoneResolutionEnum {
    #[serde(rename = "TIME_ZONE_RESOLUTION_UNSPECIFIED")]
    #[doc = "Time zone resolution is either unspecific or unknown."]
    TimeZoneResolutionUnspecified,
    #[serde(rename = "TIME_ZONE_RESOLUTION_END_USER")]
    #[doc = "Times are resolved in the time zone of the user that saw the ad."]
    TimeZoneResolutionEndUser,
    #[serde(rename = "TIME_ZONE_RESOLUTION_ADVERTISER")]
    #[doc = "Times are resolved in the time zone of the advertiser that served the ad."]
    TimeZoneResolutionAdvertiser,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for ManualTriggerService.DeactivateManualTrigger."]
pub struct DeactivateManualTriggerRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request listing which assigned targeting options of a given targeting type should be deleted."]
pub struct DeleteAssignedTargetingOptionsRequest {
    #[serde(rename = "assignedTargetingOptionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The assigned targeting option IDs to delete."]
    pub assigned_targeting_option_ids:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "targetingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifies the type of this assigned targeting option."]
    pub targeting_type:
        ::std::option::Option<DeleteAssignedTargetingOptionsRequestTargetingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Identifies the type of this assigned targeting option."]
pub enum DeleteAssignedTargetingOptionsRequestTargetingTypeEnum {
    #[serde(rename = "TARGETING_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown in this version."]
    TargetingTypeUnspecified,
    #[serde(rename = "TARGETING_TYPE_CHANNEL")]
    #[doc = "Target a channel (a custom group of related websites or apps)."]
    TargetingTypeChannel,
    #[serde(rename = "TARGETING_TYPE_APP_CATEGORY")]
    #[doc = "Target an app category (for example, education or puzzle games)."]
    TargetingTypeAppCategory,
    #[serde(rename = "TARGETING_TYPE_APP")]
    #[doc = "Target a specific app (for example, Angry Birds)."]
    TargetingTypeApp,
    #[serde(rename = "TARGETING_TYPE_URL")]
    #[doc = "Target a specific url (for example, quora.com)."]
    TargetingTypeUrl,
    #[serde(rename = "TARGETING_TYPE_DAY_AND_TIME")]
    #[doc = "Target ads during a chosen time period on a specific day."]
    TargetingTypeDayAndTime,
    #[serde(rename = "TARGETING_TYPE_AGE_RANGE")]
    #[doc = "Target ads to a specific age range (for example, 18-24)."]
    TargetingTypeAgeRange,
    #[serde(rename = "TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    #[doc = "Target ads to the specified regions on a regional location list."]
    TargetingTypeRegionalLocationList,
    #[serde(rename = "TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    #[doc = "Target ads to the specified points of interest on a proximity location list."]
    TargetingTypeProximityLocationList,
    #[serde(rename = "TARGETING_TYPE_GENDER")]
    #[doc = "Target ads to a specific gender (for example, female or male)."]
    TargetingTypeGender,
    #[serde(rename = "TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    #[doc = "Target a specific video player size for video ads."]
    TargetingTypeVideoPlayerSize,
    #[serde(rename = "TARGETING_TYPE_USER_REWARDED_CONTENT")]
    #[doc = "Target user rewarded content for video ads."]
    TargetingTypeUserRewardedContent,
    #[serde(rename = "TARGETING_TYPE_PARENTAL_STATUS")]
    #[doc = "Target ads to a specific parental status (for example, parent or not a parent)."]
    TargetingTypeParentalStatus,
    #[serde(rename = "TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    #[doc = "Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll)."]
    TargetingTypeContentInstreamPosition,
    #[serde(rename = "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    #[doc = "Target ads in a specific content outstream position."]
    TargetingTypeContentOutstreamPosition,
    #[serde(rename = "TARGETING_TYPE_DEVICE_TYPE")]
    #[doc = "Target ads to a specific device type (for example, tablet or connected TV)."]
    TargetingTypeDeviceType,
    #[serde(rename = "TARGETING_TYPE_AUDIENCE_GROUP")]
    #[doc = "Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time."]
    TargetingTypeAudienceGroup,
    #[serde(rename = "TARGETING_TYPE_BROWSER")]
    #[doc = "Target ads to specific web browsers (for example, Chrome)."]
    TargetingTypeBrowser,
    #[serde(rename = "TARGETING_TYPE_HOUSEHOLD_INCOME")]
    #[doc = "Target ads to a specific household income range (for example, top 10%)."]
    TargetingTypeHouseholdIncome,
    #[serde(rename = "TARGETING_TYPE_ON_SCREEN_POSITION")]
    #[doc = "Target ads in a specific on screen position."]
    TargetingTypeOnScreenPosition,
    #[serde(rename = "TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    #[doc = "Filter web sites through third party verification (for example, IAS or DoubleVerify)."]
    TargetingTypeThirdPartyVerifier,
    #[serde(rename = "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    #[doc = "Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences)."]
    TargetingTypeDigitalContentLabelExclusion,
    #[serde(rename = "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    #[doc = "Filter website content by sensitive categories (for example, adult)."]
    TargetingTypeSensitiveCategoryExclusion,
    #[serde(rename = "TARGETING_TYPE_ENVIRONMENT")]
    #[doc = "Target ads to a specific environment (for example, web or app)."]
    TargetingTypeEnvironment,
    #[serde(rename = "TARGETING_TYPE_CARRIER_AND_ISP")]
    #[doc = "Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange)."]
    TargetingTypeCarrierAndIsp,
    #[serde(rename = "TARGETING_TYPE_OPERATING_SYSTEM")]
    #[doc = "Target ads to a specific operating system (for example, macOS)."]
    TargetingTypeOperatingSystem,
    #[serde(rename = "TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    #[doc = "Target ads to a specific device make or model (for example, Roku or Samsung)."]
    TargetingTypeDeviceMakeModel,
    #[serde(rename = "TARGETING_TYPE_KEYWORD")]
    #[doc = "Target ads to a specific keyword (for example, dog or retriever)."]
    TargetingTypeKeyword,
    #[serde(rename = "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    #[doc = "Target ads to a specific negative keyword list."]
    TargetingTypeNegativeKeywordList,
    #[serde(rename = "TARGETING_TYPE_VIEWABILITY")]
    #[doc = "Target ads to a specific viewability (for example, 80% viewable)."]
    TargetingTypeViewability,
    #[serde(rename = "TARGETING_TYPE_CATEGORY")]
    #[doc = "Target ads to a specific content category (for example, arts & entertainment)."]
    TargetingTypeCategory,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE")]
    #[doc = "Purchase impressions from specific deals and auction packages."]
    TargetingTypeInventorySource,
    #[serde(rename = "TARGETING_TYPE_LANGUAGE")]
    #[doc = "Target ads to a specific language (for example, English or Japanese)."]
    TargetingTypeLanguage,
    #[serde(rename = "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    #[doc = "Target ads to ads.txt authorized sellers."]
    TargetingTypeAuthorizedSellerStatus,
    #[serde(rename = "TARGETING_TYPE_GEO_REGION")]
    #[doc = "Target ads to a specific regional location (for example, a city or state)."]
    TargetingTypeGeoRegion,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    #[doc = "Purchase impressions from a group of deals and auction packages."]
    TargetingTypeInventorySourceGroup,
    #[serde(rename = "TARGETING_TYPE_EXCHANGE")]
    #[doc = "Purchase impressions from specific exchanges."]
    TargetingTypeExchange,
    #[serde(rename = "TARGETING_TYPE_SUB_EXCHANGE")]
    #[doc = "Purchase impressions from specific sub-exchanges."]
    TargetingTypeSubExchange,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned device make and model targeting option details. This will be populated in the device_make_model_details field when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`."]
pub struct DeviceMakeModelAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the device make and model."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable device make and model. This will be populated in the device_make_model_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_MAKE_MODEL`."]
pub struct DeviceMakeModelTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the device make and model."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for device type. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_TYPE`."]
pub struct DeviceTypeAssignedTargetingOptionDetails {
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the device type."]
    pub device_type: ::std::option::Option<DeviceTypeAssignedTargetingOptionDetailsDeviceTypeEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the device type."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The display name of the device type."]
pub enum DeviceTypeAssignedTargetingOptionDetailsDeviceTypeEnum {
    #[serde(rename = "DEVICE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when device type is not specified in this version. This enum is a placeholder for default value and does not represent a real device type option."]
    DeviceTypeUnspecified,
    #[serde(rename = "DEVICE_TYPE_COMPUTER")]
    #[doc = "The device type is computer."]
    DeviceTypeComputer,
    #[serde(rename = "DEVICE_TYPE_CONNECTED_TV")]
    #[doc = "The device type is connected TV."]
    DeviceTypeConnectedTv,
    #[serde(rename = "DEVICE_TYPE_SMART_PHONE")]
    #[doc = "The device type is smart phone.."]
    DeviceTypeSmartPhone,
    #[serde(rename = "DEVICE_TYPE_TABLET")]
    #[doc = "The device type is tablet."]
    DeviceTypeTablet,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable device type. This will be populated in the device_type_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_DEVICE_TYPE`."]
pub struct DeviceTypeTargetingOptionDetails {
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The device type that is used to be targeted."]
    pub device_type: ::std::option::Option<DeviceTypeTargetingOptionDetailsDeviceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The device type that is used to be targeted."]
pub enum DeviceTypeTargetingOptionDetailsDeviceTypeEnum {
    #[serde(rename = "DEVICE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when device type is not specified in this version. This enum is a placeholder for default value and does not represent a real device type option."]
    DeviceTypeUnspecified,
    #[serde(rename = "DEVICE_TYPE_COMPUTER")]
    #[doc = "The device type is computer."]
    DeviceTypeComputer,
    #[serde(rename = "DEVICE_TYPE_CONNECTED_TV")]
    #[doc = "The device type is connected TV."]
    DeviceTypeConnectedTv,
    #[serde(rename = "DEVICE_TYPE_SMART_PHONE")]
    #[doc = "The device type is smart phone.."]
    DeviceTypeSmartPhone,
    #[serde(rename = "DEVICE_TYPE_TABLET")]
    #[doc = "The device type is tablet."]
    DeviceTypeTablet,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for digital content label. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`."]
pub struct DigitalContentLabelAssignedTargetingOptionDetails {
    #[serde(rename = "contentRatingTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the digital content label rating tier."]
    pub content_rating_tier: ::std::option::Option<
        DigitalContentLabelAssignedTargetingOptionDetailsContentRatingTierEnum,
    >,
    #[serde(rename = "excludedTargetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the digital content label to be EXCLUDED."]
    pub excluded_targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The display name of the digital content label rating tier."]
pub enum DigitalContentLabelAssignedTargetingOptionDetailsContentRatingTierEnum {
    #[serde(rename = "CONTENT_RATING_TIER_UNSPECIFIED")]
    #[doc = "Content label is not specified in this version. This enum is a place holder for a default value and does not represent a real content rating."]
    ContentRatingTierUnspecified,
    #[serde(rename = "CONTENT_RATING_TIER_UNRATED")]
    #[doc = "Content that has not been labeled."]
    ContentRatingTierUnrated,
    #[serde(rename = "CONTENT_RATING_TIER_GENERAL")]
    #[doc = "Content suitable for general audiences."]
    ContentRatingTierGeneral,
    #[serde(rename = "CONTENT_RATING_TIER_PARENTAL_GUIDANCE")]
    #[doc = "Content suitable for most audiences with parental guidance."]
    ContentRatingTierParentalGuidance,
    #[serde(rename = "CONTENT_RATING_TIER_TEENS")]
    #[doc = "Content suitable for teen and older audiences."]
    ContentRatingTierTeens,
    #[serde(rename = "CONTENT_RATING_TIER_MATURE")]
    #[doc = "Content suitable only for mature audiences."]
    ContentRatingTierMature,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable digital content label rating tier. This will be populated in the digital_content_label_details field of the TargetingOption when targeting_type is `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION`."]
pub struct DigitalContentLabelTargetingOptionDetails {
    #[serde(rename = "contentRatingTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An enum for the content label brand safety tiers."]
    pub content_rating_tier:
        ::std::option::Option<DigitalContentLabelTargetingOptionDetailsContentRatingTierEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. An enum for the content label brand safety tiers."]
pub enum DigitalContentLabelTargetingOptionDetailsContentRatingTierEnum {
    #[serde(rename = "CONTENT_RATING_TIER_UNSPECIFIED")]
    #[doc = "Content label is not specified in this version. This enum is a place holder for a default value and does not represent a real content rating."]
    ContentRatingTierUnspecified,
    #[serde(rename = "CONTENT_RATING_TIER_UNRATED")]
    #[doc = "Content that has not been labeled."]
    ContentRatingTierUnrated,
    #[serde(rename = "CONTENT_RATING_TIER_GENERAL")]
    #[doc = "Content suitable for general audiences."]
    ContentRatingTierGeneral,
    #[serde(rename = "CONTENT_RATING_TIER_PARENTAL_GUIDANCE")]
    #[doc = "Content suitable for most audiences with parental guidance."]
    ContentRatingTierParentalGuidance,
    #[serde(rename = "CONTENT_RATING_TIER_TEENS")]
    #[doc = "Content suitable for teen and older audiences."]
    ContentRatingTierTeens,
    #[serde(rename = "CONTENT_RATING_TIER_MATURE")]
    #[doc = "Content suitable only for mature audiences."]
    ContentRatingTierMature,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dimensions."]
pub struct Dimensions {
    #[serde(rename = "heightPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height in pixels."]
    pub height_pixels: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "widthPixels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width in pixels."]
    pub width_pixels: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of DoubleVerify settings."]
pub struct DoubleVerify {
    #[serde(rename = "appStarRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid bidding on apps with the star ratings."]
    pub app_star_rating: ::std::option::Option<::std::boxed::Box<DoubleVerifyAppStarRating>>,
    #[serde(rename = "avoidedAgeRatings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid bidding on apps with the age rating."]
    pub avoided_age_ratings:
        ::std::option::Option<::std::vec::Vec<DoubleVerifyAvoidedAgeRatingsEnum>>,
    #[serde(rename = "brandSafetyCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DV Brand Safety Controls."]
    pub brand_safety_categories:
        ::std::option::Option<::std::boxed::Box<DoubleVerifyBrandSafetyCategories>>,
    #[serde(rename = "customSegmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom segment ID provided by DoubleVerify. The ID must start with \"51\" and consist of eight digits. Custom segment ID cannot be specified along with any of the following fields: * brand_safety_categories * avoided_age_ratings * app_star_rating * fraud_invalid_traffic"]
    pub custom_segment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayViewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display viewability settings (applicable to display line items only)."]
    pub display_viewability:
        ::std::option::Option<::std::boxed::Box<DoubleVerifyDisplayViewability>>,
    #[serde(rename = "fraudInvalidTraffic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid Sites and Apps with historical Fraud & IVT Rates."]
    pub fraud_invalid_traffic:
        ::std::option::Option<::std::boxed::Box<DoubleVerifyFraudInvalidTraffic>>,
    #[serde(rename = "videoViewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video viewability settings (applicable to video line items only)."]
    pub video_viewability: ::std::option::Option<::std::boxed::Box<DoubleVerifyVideoViewability>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DoubleVerifyAvoidedAgeRatingsEnum {
    #[serde(rename = "AGE_RATING_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any age rating options."]
    AgeRatingUnspecified,
    #[serde(rename = "APP_AGE_RATE_UNKNOWN")]
    #[doc = "Apps with unknown age rating."]
    AppAgeRateUnknown,
    #[serde(rename = "APP_AGE_RATE_4_PLUS")]
    #[doc = "Apps rated for Everyone (4+)."]
    AppAgeRate4Plus,
    #[serde(rename = "APP_AGE_RATE_9_PLUS")]
    #[doc = "Apps rated for Everyone (9+)."]
    AppAgeRate9Plus,
    #[serde(rename = "APP_AGE_RATE_12_PLUS")]
    #[doc = "Apps rated for Teens (12+)."]
    AppAgeRate12Plus,
    #[serde(rename = "APP_AGE_RATE_17_PLUS")]
    #[doc = "Apps rated for Mature (17+)."]
    AppAgeRate17Plus,
    #[serde(rename = "APP_AGE_RATE_18_PLUS")]
    #[doc = "Apps rated for Adults Only (18+)."]
    AppAgeRate18Plus,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of DoubleVerify star ratings settings."]
pub struct DoubleVerifyAppStarRating {
    #[serde(rename = "avoidInsufficientStarRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid bidding on apps with insufficient star ratings."]
    pub avoid_insufficient_star_rating: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "avoidedStarRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid bidding on apps with the star ratings."]
    pub avoided_star_rating: ::std::option::Option<DoubleVerifyAppStarRatingAvoidedStarRatingEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Avoid bidding on apps with the star ratings."]
pub enum DoubleVerifyAppStarRatingAvoidedStarRatingEnum {
    #[serde(rename = "APP_STAR_RATE_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any app star rating options."]
    AppStarRateUnspecified,
    #[serde(rename = "APP_STAR_RATE_1_POINT_5_LESS")]
    #[doc = "Official Apps with rating < 1.5 Stars."]
    AppStarRate1Point5Less,
    #[serde(rename = "APP_STAR_RATE_2_LESS")]
    #[doc = "Official Apps with rating < 2 Stars."]
    AppStarRate2Less,
    #[serde(rename = "APP_STAR_RATE_2_POINT_5_LESS")]
    #[doc = "Official Apps with rating < 2.5 Stars."]
    AppStarRate2Point5Less,
    #[serde(rename = "APP_STAR_RATE_3_LESS")]
    #[doc = "Official Apps with rating < 3 Stars."]
    AppStarRate3Less,
    #[serde(rename = "APP_STAR_RATE_3_POINT_5_LESS")]
    #[doc = "Official Apps with rating < 3.5 Stars."]
    AppStarRate3Point5Less,
    #[serde(rename = "APP_STAR_RATE_4_LESS")]
    #[doc = "Official Apps with rating < 4 Stars."]
    AppStarRate4Less,
    #[serde(rename = "APP_STAR_RATE_4_POINT_5_LESS")]
    #[doc = "Official Apps with rating < 4.5 Stars."]
    AppStarRate4Point5Less,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for brand safety controls."]
pub struct DoubleVerifyBrandSafetyCategories {
    #[serde(rename = "avoidUnknownBrandSafetyCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unknown or unrateable."]
    pub avoid_unknown_brand_safety_category: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "avoidedHighSeverityCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand safety high severity avoidance categories."]
    pub avoided_high_severity_categories: ::std::option::Option<
        ::std::vec::Vec<DoubleVerifyBrandSafetyCategoriesAvoidedHighSeverityCategoriesEnum>,
    >,
    #[serde(rename = "avoidedMediumSeverityCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand safety medium severity avoidance categories."]
    pub avoided_medium_severity_categories: ::std::option::Option<
        ::std::vec::Vec<DoubleVerifyBrandSafetyCategoriesAvoidedMediumSeverityCategoriesEnum>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DoubleVerifyBrandSafetyCategoriesAvoidedHighSeverityCategoriesEnum {
    #[serde(rename = "HIGHER_SEVERITY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any high severity categories."]
    HigherSeverityUnspecified,
    #[serde(rename = "ADULT_CONTENT_PORNOGRAPHY")]
    #[doc = "Adult Content: Pornography, Mature Topics & Nudity."]
    AdultContentPornography,
    #[serde(rename = "COPYRIGHT_INFRINGEMENT")]
    #[doc = "Copyright Infringement."]
    CopyrightInfringement,
    #[serde(rename = "SUBSTANCE_ABUSE")]
    #[doc = "Drugs/Alcohol/Controlled Substances: Substance Abuse."]
    SubstanceAbuse,
    #[serde(rename = "GRAPHIC_VIOLENCE_WEAPONS")]
    #[doc = "Extreme Graphic/Explicit Violence/Weapons."]
    GraphicViolenceWeapons,
    #[serde(rename = "HATE_PROFANITY")]
    #[doc = "Hate/Profanity."]
    HateProfanity,
    #[serde(rename = "CRIMINAL_SKILLS")]
    #[doc = "Illegal Activities: Criminal Skills."]
    CriminalSkills,
    #[serde(rename = "NUISANCE_INCENTIVIZED_MALWARE_CLUTTER")]
    #[doc = "Incentivized/Malware/Clutter."]
    NuisanceIncentivizedMalwareClutter,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DoubleVerifyBrandSafetyCategoriesAvoidedMediumSeverityCategoriesEnum {
    #[serde(rename = "MEDIUM_SEVERITY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any medium severity categories."]
    MediumSeverityUnspecified,
    #[serde(rename = "AD_SERVERS")]
    #[doc = "Ad Servers."]
    AdServers,
    #[serde(rename = "ADULT_CONTENT_SWIMSUIT")]
    #[doc = "Adult Content: Swimsuit."]
    AdultContentSwimsuit,
    #[serde(rename = "ALTERNATIVE_LIFESTYLES")]
    #[doc = "Controversial Subjects: Alternative Lifestyles."]
    AlternativeLifestyles,
    #[serde(rename = "CELEBRITY_GOSSIP")]
    #[doc = "Controversial Subjects: Celebrity Gossip."]
    CelebrityGossip,
    #[serde(rename = "GAMBLING")]
    #[doc = "Controversial Subjects: Gambling."]
    Gambling,
    #[serde(rename = "OCCULT")]
    #[doc = "Controversial Subjects: Occult."]
    Occult,
    #[serde(rename = "SEX_EDUCATION")]
    #[doc = "Controversial Subjects: Sex Education."]
    SexEducation,
    #[serde(rename = "DISASTER_AVIATION")]
    #[doc = "Disaster: Aviation."]
    DisasterAviation,
    #[serde(rename = "DISASTER_MAN_MADE")]
    #[doc = "Disaster: Man-made."]
    DisasterManMade,
    #[serde(rename = "DISASTER_NATURAL")]
    #[doc = "Disaster: Natural."]
    DisasterNatural,
    #[serde(rename = "DISASTER_TERRORIST_EVENTS")]
    #[doc = "Disaster: Terrorist Events."]
    DisasterTerroristEvents,
    #[serde(rename = "DISASTER_VEHICLE")]
    #[doc = "Disaster: Vehicle."]
    DisasterVehicle,
    #[serde(rename = "ALCOHOL")]
    #[doc = "Drugs/Alcohol/Controlled Substances: Alcohol."]
    Alcohol,
    #[serde(rename = "SMOKING")]
    #[doc = "Drugs/Alcohol/Controlled Substances: Smoking."]
    Smoking,
    #[serde(rename = "NEGATIVE_NEWS_FINANCIAL")]
    #[doc = "Negative News: Financial."]
    NegativeNewsFinancial,
    #[serde(rename = "NON_ENGLISH")]
    #[doc = "Non-Std Content: Non-English."]
    NonEnglish,
    #[serde(rename = "PARKING_PAGE")]
    #[doc = "Non-Std Content: Parking Page."]
    ParkingPage,
    #[serde(rename = "UNMODERATED_UGC")]
    #[doc = "Unmoderated UGC: Forums, Images & Video."]
    UnmoderatedUgc,
    #[serde(rename = "INFLAMMATORY_POLITICS_AND_NEWS")]
    #[doc = "Controversial Subjects: Inflammatory Politics and News."]
    InflammatoryPoliticsAndNews,
    #[serde(rename = "NEGATIVE_NEWS_PHARMACEUTICAL")]
    #[doc = "Negative News: Pharmaceutical."]
    NegativeNewsPharmaceutical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of DoubleVerify display viewability settings."]
pub struct DoubleVerifyDisplayViewability {
    #[serde(rename = "iab")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate."]
    pub iab: ::std::option::Option<DoubleVerifyDisplayViewabilityIabEnum>,
    #[serde(rename = "viewableDuring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target web and app inventory to maximize 100% viewable duration."]
    pub viewable_during: ::std::option::Option<DoubleVerifyDisplayViewabilityViewableDuringEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target web and app inventory to maximize IAB viewable rate."]
pub enum DoubleVerifyDisplayViewabilityIabEnum {
    #[serde(rename = "IAB_VIEWED_RATE_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any IAB viewed rate options."]
    IabViewedRateUnspecified,
    #[serde(rename = "IAB_VIEWED_RATE_80_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 80% or higher."]
    IabViewedRate80PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_75_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 75% or higher."]
    IabViewedRate75PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_70_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 70% or higher."]
    IabViewedRate70PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_65_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 65% or higher."]
    IabViewedRate65PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_60_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 60% or higher."]
    IabViewedRate60PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_55_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 55% or higher."]
    IabViewedRate55PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_50_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 50% or higher."]
    IabViewedRate50PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_40_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 40% or higher."]
    IabViewedRate40PercentHigher,
    #[serde(rename = "IAB_VIEWED_RATE_30_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 30% or higher."]
    IabViewedRate30PercentHigher,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target web and app inventory to maximize 100% viewable duration."]
pub enum DoubleVerifyDisplayViewabilityViewableDuringEnum {
    #[serde(rename = "AVERAGE_VIEW_DURATION_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any average view duration options."]
    AverageViewDurationUnspecified,
    #[serde(rename = "AVERAGE_VIEW_DURATION_5_SEC")]
    #[doc = "Target web and app inventory to maximize 100% viewable duration 5 seconds or more."]
    AverageViewDuration5Sec,
    #[serde(rename = "AVERAGE_VIEW_DURATION_10_SEC")]
    #[doc = "Target web and app inventory to maximize 100% viewable duration 10 seconds or more."]
    AverageViewDuration10Sec,
    #[serde(rename = "AVERAGE_VIEW_DURATION_15_SEC")]
    #[doc = "Target web and app inventory to maximize 100% viewable duration 15 seconds or more."]
    AverageViewDuration15Sec,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DoubleVerify Fraud & Invalid Traffic settings."]
pub struct DoubleVerifyFraudInvalidTraffic {
    #[serde(rename = "avoidInsufficientOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Insufficient Historical Fraud & IVT Stats."]
    pub avoid_insufficient_option: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "avoidedFraudOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Avoid Sites and Apps with historical Fraud & IVT."]
    pub avoided_fraud_option:
        ::std::option::Option<DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Avoid Sites and Apps with historical Fraud & IVT."]
pub enum DoubleVerifyFraudInvalidTrafficAvoidedFraudOptionEnum {
    #[serde(rename = "FRAUD_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any fraud and invalid traffic options."]
    FraudUnspecified,
    #[serde(rename = "AD_IMPRESSION_FRAUD_100")]
    #[doc = "100% Fraud & IVT."]
    AdImpressionFraud100,
    #[serde(rename = "AD_IMPRESSION_FRAUD_50")]
    #[doc = "50% or Higher Fraud & IVT."]
    AdImpressionFraud50,
    #[serde(rename = "AD_IMPRESSION_FRAUD_25")]
    #[doc = "25% or Higher Fraud & IVT."]
    AdImpressionFraud25,
    #[serde(rename = "AD_IMPRESSION_FRAUD_10")]
    #[doc = "10% or Higher Fraud & IVT."]
    AdImpressionFraud10,
    #[serde(rename = "AD_IMPRESSION_FRAUD_8")]
    #[doc = "8% or Higher Fraud & IVT."]
    AdImpressionFraud8,
    #[serde(rename = "AD_IMPRESSION_FRAUD_6")]
    #[doc = "6% or Higher Fraud & IVT."]
    AdImpressionFraud6,
    #[serde(rename = "AD_IMPRESSION_FRAUD_4")]
    #[doc = "4% or Higher Fraud & IVT."]
    AdImpressionFraud4,
    #[serde(rename = "AD_IMPRESSION_FRAUD_2")]
    #[doc = "2% or Higher Fraud & IVT."]
    AdImpressionFraud2,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of DoubleVerify video viewability settings."]
pub struct DoubleVerifyVideoViewability {
    #[serde(rename = "playerImpressionRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target inventory to maximize impressions with 400x300 or greater player size."]
    pub player_impression_rate:
        ::std::option::Option<DoubleVerifyVideoViewabilityPlayerImpressionRateEnum>,
    #[serde(rename = "videoIab")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target web inventory to maximize IAB viewable rate."]
    pub video_iab: ::std::option::Option<DoubleVerifyVideoViewabilityVideoIabEnum>,
    #[serde(rename = "videoViewableRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target web inventory to maximize fully viewable rate."]
    pub video_viewable_rate:
        ::std::option::Option<DoubleVerifyVideoViewabilityVideoViewableRateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target inventory to maximize impressions with 400x300 or greater player size."]
pub enum DoubleVerifyVideoViewabilityPlayerImpressionRateEnum {
    #[serde(rename = "PLAYER_SIZE_400X300_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any impressions options."]
    PlayerSize400X300Unspecified,
    #[serde(rename = "PLAYER_SIZE_400X300_95")]
    #[doc = "Sites with 95%+ of impressions."]
    PlayerSize400X30095,
    #[serde(rename = "PLAYER_SIZE_400X300_70")]
    #[doc = "Sites with 70%+ of impressions."]
    PlayerSize400X30070,
    #[serde(rename = "PLAYER_SIZE_400X300_25")]
    #[doc = "Sites with 25%+ of impressions."]
    PlayerSize400X30025,
    #[serde(rename = "PLAYER_SIZE_400X300_5")]
    #[doc = "Sites with 5%+ of impressions."]
    PlayerSize400X3005,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target web inventory to maximize IAB viewable rate."]
pub enum DoubleVerifyVideoViewabilityVideoIabEnum {
    #[serde(rename = "VIDEO_IAB_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any video IAB viewable rate options."]
    VideoIabUnspecified,
    #[serde(rename = "IAB_VIEWABILITY_80_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 80% or higher."]
    IabViewability80PercentHigher,
    #[serde(rename = "IAB_VIEWABILITY_75_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 75% or higher."]
    IabViewability75PercentHigher,
    #[serde(rename = "IAB_VIEWABILITY_70_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 70% or higher."]
    IabViewability70PercentHigher,
    #[serde(rename = "IAB_VIEWABILITY_65_PERCENT_HIHGER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 65% or higher."]
    IabViewability65PercentHihger,
    #[serde(rename = "IAB_VIEWABILITY_60_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 60% or higher."]
    IabViewability60PercentHigher,
    #[serde(rename = "IAB_VIEWABILITY_55_PERCENT_HIHGER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 55% or higher."]
    IabViewability55PercentHihger,
    #[serde(rename = "IAB_VIEWABILITY_50_PERCENT_HIGHER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 50% or higher."]
    IabViewability50PercentHigher,
    #[serde(rename = "IAB_VIEWABILITY_40_PERCENT_HIHGER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 40% or higher."]
    IabViewability40PercentHihger,
    #[serde(rename = "IAB_VIEWABILITY_30_PERCENT_HIHGER")]
    #[doc = "Target web and app inventory to maximize IAB viewable rate 30% or higher."]
    IabViewability30PercentHihger,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target web inventory to maximize fully viewable rate."]
pub enum DoubleVerifyVideoViewabilityVideoViewableRateEnum {
    #[serde(rename = "VIDEO_VIEWABLE_RATE_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any video viewable rate options."]
    VideoViewableRateUnspecified,
    #[serde(rename = "VIEWED_PERFORMANCE_40_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 40% or higher."]
    ViewedPerformance40PercentHigher,
    #[serde(rename = "VIEWED_PERFORMANCE_35_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 35% or higher."]
    ViewedPerformance35PercentHigher,
    #[serde(rename = "VIEWED_PERFORMANCE_30_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 30% or higher."]
    ViewedPerformance30PercentHigher,
    #[serde(rename = "VIEWED_PERFORMANCE_25_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 25% or higher."]
    ViewedPerformance25PercentHigher,
    #[serde(rename = "VIEWED_PERFORMANCE_20_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 20% or higher."]
    ViewedPerformance20PercentHigher,
    #[serde(rename = "VIEWED_PERFORMANCE_10_PERCENT_HIGHER")]
    #[doc = "Target web inventory to maximize fully viewable rate 10% or higher."]
    ViewedPerformance10PercentHigher,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned environment targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_ENVIRONMENT`."]
pub struct EnvironmentAssignedTargetingOptionDetails {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The serving environment."]
    pub environment:
        ::std::option::Option<EnvironmentAssignedTargetingOptionDetailsEnvironmentEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_ENVIRONMENT` (e.g., \"508010\" for targeting the `ENVIRONMENT_WEB_OPTIMIZED` option)."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The serving environment."]
pub enum EnvironmentAssignedTargetingOptionDetailsEnvironmentEnum {
    #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
    #[doc = "Default value when environment is not specified in this version. This enum is a placeholder for default value and does not represent a real environment option."]
    EnvironmentUnspecified,
    #[serde(rename = "ENVIRONMENT_WEB_OPTIMIZED")]
    #[doc = "Target inventory displayed in browsers. This includes inventory that was designed for the device it was viewed on, such as mobile websites viewed on a mobile device. ENVIRONMENT_WEB_NOT_OPTIMIZED, if targeted, should be deleted prior to the deletion of this targeting option."]
    EnvironmentWebOptimized,
    #[serde(rename = "ENVIRONMENT_WEB_NOT_OPTIMIZED")]
    #[doc = "Target inventory displayed in browsers. This includes inventory that was not designed for the device but viewed on it, such as websites optimized for desktop but viewed on a mobile device. ENVIRONMENT_WEB_OPTIMIZED should be targeted prior to the addition of this targeting option."]
    EnvironmentWebNotOptimized,
    #[serde(rename = "ENVIRONMENT_APP")]
    #[doc = "Target inventory displayed in apps."]
    EnvironmentApp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable environment. This will be populated in the environment_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_ENVIRONMENT`."]
pub struct EnvironmentTargetingOptionDetails {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The serving environment."]
    pub environment: ::std::option::Option<EnvironmentTargetingOptionDetailsEnvironmentEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The serving environment."]
pub enum EnvironmentTargetingOptionDetailsEnvironmentEnum {
    #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
    #[doc = "Default value when environment is not specified in this version. This enum is a placeholder for default value and does not represent a real environment option."]
    EnvironmentUnspecified,
    #[serde(rename = "ENVIRONMENT_WEB_OPTIMIZED")]
    #[doc = "Target inventory displayed in browsers. This includes inventory that was designed for the device it was viewed on, such as mobile websites viewed on a mobile device. ENVIRONMENT_WEB_NOT_OPTIMIZED, if targeted, should be deleted prior to the deletion of this targeting option."]
    EnvironmentWebOptimized,
    #[serde(rename = "ENVIRONMENT_WEB_NOT_OPTIMIZED")]
    #[doc = "Target inventory displayed in browsers. This includes inventory that was not designed for the device but viewed on it, such as websites optimized for desktop but viewed on a mobile device. ENVIRONMENT_WEB_OPTIMIZED should be targeted prior to the addition of this targeting option."]
    EnvironmentWebNotOptimized,
    #[serde(rename = "ENVIRONMENT_APP")]
    #[doc = "Target inventory displayed in apps."]
    EnvironmentApp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned exchange targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_EXCHANGE`."]
pub struct ExchangeAssignedTargetingOptionDetails {
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_EXCHANGE`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control which exchanges are enabled for a partner."]
pub struct ExchangeConfig {
    #[serde(rename = "enabledExchanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All enabled exchanges in the partner. Duplicate enabled exchanges will be ignored."]
    pub enabled_exchanges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExchangeConfigEnabledExchange>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An enabled exchange in the partner."]
pub struct ExchangeConfigEnabledExchange {
    #[serde(rename = "exchange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The enabled exchange."]
    pub exchange: ::std::option::Option<ExchangeConfigEnabledExchangeExchangeEnum>,
    #[serde(rename = "googleAdManagerAgencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Agency ID of Google Ad Manager. The field is only relevant when Google Ad Manager is the enabled exchange."]
    pub google_ad_manager_agency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleAdManagerBuyerNetworkId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Network ID of Google Ad Manager. The field is only relevant when Google Ad Manager is the enabled exchange."]
    pub google_ad_manager_buyer_network_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seatId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Seat ID of the enabled exchange."]
    pub seat_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The enabled exchange."]
pub enum ExchangeConfigEnabledExchangeExchangeEnum {
    #[serde(rename = "EXCHANGE_UNSPECIFIED")]
    #[doc = "Exchange is not specified or is unknown in this version."]
    ExchangeUnspecified,
    #[serde(rename = "EXCHANGE_GOOGLE_AD_MANAGER")]
    #[doc = "Google Ad Manager."]
    ExchangeGoogleAdManager,
    #[serde(rename = "EXCHANGE_APPNEXUS")]
    #[doc = "AppNexus."]
    ExchangeAppnexus,
    #[serde(rename = "EXCHANGE_BRIGHTROLL")]
    #[doc = "BrightRoll Exchange for Video from Yahoo!."]
    ExchangeBrightroll,
    #[serde(rename = "EXCHANGE_ADFORM")]
    #[doc = "Adform."]
    ExchangeAdform,
    #[serde(rename = "EXCHANGE_ADMETA")]
    #[doc = "Admeta."]
    ExchangeAdmeta,
    #[serde(rename = "EXCHANGE_ADMIXER")]
    #[doc = "Admixer."]
    ExchangeAdmixer,
    #[serde(rename = "EXCHANGE_ADSMOGO")]
    #[doc = "AdsMogo."]
    ExchangeAdsmogo,
    #[serde(rename = "EXCHANGE_ADSWIZZ")]
    #[doc = "AdsWizz."]
    ExchangeAdswizz,
    #[serde(rename = "EXCHANGE_BIDSWITCH")]
    #[doc = "BidSwitch."]
    ExchangeBidswitch,
    #[serde(rename = "EXCHANGE_BRIGHTROLL_DISPLAY")]
    #[doc = "BrightRoll Exchange for Display from Yahoo!."]
    ExchangeBrightrollDisplay,
    #[serde(rename = "EXCHANGE_CADREON")]
    #[doc = "Cadreon."]
    ExchangeCadreon,
    #[serde(rename = "EXCHANGE_DAILYMOTION")]
    #[doc = "Dailymotion."]
    ExchangeDailymotion,
    #[serde(rename = "EXCHANGE_FIVE")]
    #[doc = "Five."]
    ExchangeFive,
    #[serde(rename = "EXCHANGE_FLUCT")]
    #[doc = "Fluct."]
    ExchangeFluct,
    #[serde(rename = "EXCHANGE_FREEWHEEL")]
    #[doc = "FreeWheel SSP."]
    ExchangeFreewheel,
    #[serde(rename = "EXCHANGE_GENIEE")]
    #[doc = "Geniee."]
    ExchangeGeniee,
    #[serde(rename = "EXCHANGE_GUMGUM")]
    #[doc = "GumGum."]
    ExchangeGumgum,
    #[serde(rename = "EXCHANGE_IMOBILE")]
    #[doc = "i-mobile."]
    ExchangeImobile,
    #[serde(rename = "EXCHANGE_IBILLBOARD")]
    #[doc = "iBILLBOARD."]
    ExchangeIbillboard,
    #[serde(rename = "EXCHANGE_IMPROVE_DIGITAL")]
    #[doc = "Improve Digital."]
    ExchangeImproveDigital,
    #[serde(rename = "EXCHANGE_INDEX")]
    #[doc = "Index Exchange."]
    ExchangeIndex,
    #[serde(rename = "EXCHANGE_KARGO")]
    #[doc = "Kargo."]
    ExchangeKargo,
    #[serde(rename = "EXCHANGE_MICROAD")]
    #[doc = "MicroAd."]
    ExchangeMicroad,
    #[serde(rename = "EXCHANGE_MOPUB")]
    #[doc = "MoPub."]
    ExchangeMopub,
    #[serde(rename = "EXCHANGE_NEND")]
    #[doc = "Nend."]
    ExchangeNend,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_DISPLAY")]
    #[doc = "ONE by AOL: Display Market Place."]
    ExchangeOneByAolDisplay,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_MOBILE")]
    #[doc = "ONE by AOL: Mobile."]
    ExchangeOneByAolMobile,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_VIDEO")]
    #[doc = "ONE by AOL: Video."]
    ExchangeOneByAolVideo,
    #[serde(rename = "EXCHANGE_OOYALA")]
    #[doc = "Ooyala."]
    ExchangeOoyala,
    #[serde(rename = "EXCHANGE_OPENX")]
    #[doc = "OpenX."]
    ExchangeOpenx,
    #[serde(rename = "EXCHANGE_PERMODO")]
    #[doc = "Permodo."]
    ExchangePermodo,
    #[serde(rename = "EXCHANGE_PLATFORMONE")]
    #[doc = "Platform One."]
    ExchangePlatformone,
    #[serde(rename = "EXCHANGE_PLATFORMID")]
    #[doc = "PlatformId."]
    ExchangePlatformid,
    #[serde(rename = "EXCHANGE_PUBMATIC")]
    #[doc = "PubMatic."]
    ExchangePubmatic,
    #[serde(rename = "EXCHANGE_PULSEPOINT")]
    #[doc = "PulsePoint."]
    ExchangePulsepoint,
    #[serde(rename = "EXCHANGE_REVENUEMAX")]
    #[doc = "RevenueMax."]
    ExchangeRevenuemax,
    #[serde(rename = "EXCHANGE_RUBICON")]
    #[doc = "Rubicon."]
    ExchangeRubicon,
    #[serde(rename = "EXCHANGE_SMARTCLIP")]
    #[doc = "SmartClip."]
    ExchangeSmartclip,
    #[serde(rename = "EXCHANGE_SMARTRTB")]
    #[doc = "SmartRTB+."]
    ExchangeSmartrtb,
    #[serde(rename = "EXCHANGE_SMARTSTREAMTV")]
    #[doc = "SmartstreamTv."]
    ExchangeSmartstreamtv,
    #[serde(rename = "EXCHANGE_SOVRN")]
    #[doc = "Sovrn."]
    ExchangeSovrn,
    #[serde(rename = "EXCHANGE_SPOTXCHANGE")]
    #[doc = "SpotXchange."]
    ExchangeSpotxchange,
    #[serde(rename = "EXCHANGE_STROER")]
    #[doc = "Ströer SSP."]
    ExchangeStroer,
    #[serde(rename = "EXCHANGE_TEADSTV")]
    #[doc = "TeadsTv."]
    ExchangeTeadstv,
    #[serde(rename = "EXCHANGE_TELARIA")]
    #[doc = "Telaria."]
    ExchangeTelaria,
    #[serde(rename = "EXCHANGE_TVN")]
    #[doc = "TVN."]
    ExchangeTvn,
    #[serde(rename = "EXCHANGE_UNITED")]
    #[doc = "United."]
    ExchangeUnited,
    #[serde(rename = "EXCHANGE_YIELDLAB")]
    #[doc = "Yieldlab."]
    ExchangeYieldlab,
    #[serde(rename = "EXCHANGE_YIELDMO")]
    #[doc = "Yieldmo."]
    ExchangeYieldmo,
    #[serde(rename = "EXCHANGE_UNRULYX")]
    #[doc = "UnrulyX."]
    ExchangeUnrulyx,
    #[serde(rename = "EXCHANGE_OPEN8")]
    #[doc = "Open8."]
    ExchangeOpen8,
    #[serde(rename = "EXCHANGE_TRITON")]
    #[doc = "Triton."]
    ExchangeTriton,
    #[serde(rename = "EXCHANGE_TRIPLELIFT")]
    #[doc = "TripleLift."]
    ExchangeTriplelift,
    #[serde(rename = "EXCHANGE_TABOOLA")]
    #[doc = "Taboola."]
    ExchangeTaboola,
    #[serde(rename = "EXCHANGE_INMOBI")]
    #[doc = "InMobi."]
    ExchangeInmobi,
    #[serde(rename = "EXCHANGE_SMAATO")]
    #[doc = "Smaato."]
    ExchangeSmaato,
    #[serde(rename = "EXCHANGE_AJA")]
    #[doc = "Aja."]
    ExchangeAja,
    #[serde(rename = "EXCHANGE_SUPERSHIP")]
    #[doc = "Supership."]
    ExchangeSupership,
    #[serde(rename = "EXCHANGE_NEXSTAR_DIGITAL")]
    #[doc = "Nexstar Digital."]
    ExchangeNexstarDigital,
    #[serde(rename = "EXCHANGE_WAZE")]
    #[doc = "Waze."]
    ExchangeWaze,
    #[serde(rename = "EXCHANGE_SOUNDCAST")]
    #[doc = "SoundCast."]
    ExchangeSoundcast,
    #[serde(rename = "EXCHANGE_SHARETHROUGH")]
    #[doc = "Sharethrough."]
    ExchangeSharethrough,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Exchange review status for the creative."]
pub struct ExchangeReviewStatus {
    #[serde(rename = "exchange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exchange reviewing the creative."]
    pub exchange: ::std::option::Option<ExchangeReviewStatusExchangeEnum>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the exchange review."]
    pub status: ::std::option::Option<ExchangeReviewStatusStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The exchange reviewing the creative."]
pub enum ExchangeReviewStatusExchangeEnum {
    #[serde(rename = "EXCHANGE_UNSPECIFIED")]
    #[doc = "Exchange is not specified or is unknown in this version."]
    ExchangeUnspecified,
    #[serde(rename = "EXCHANGE_GOOGLE_AD_MANAGER")]
    #[doc = "Google Ad Manager."]
    ExchangeGoogleAdManager,
    #[serde(rename = "EXCHANGE_APPNEXUS")]
    #[doc = "AppNexus."]
    ExchangeAppnexus,
    #[serde(rename = "EXCHANGE_BRIGHTROLL")]
    #[doc = "BrightRoll Exchange for Video from Yahoo!."]
    ExchangeBrightroll,
    #[serde(rename = "EXCHANGE_ADFORM")]
    #[doc = "Adform."]
    ExchangeAdform,
    #[serde(rename = "EXCHANGE_ADMETA")]
    #[doc = "Admeta."]
    ExchangeAdmeta,
    #[serde(rename = "EXCHANGE_ADMIXER")]
    #[doc = "Admixer."]
    ExchangeAdmixer,
    #[serde(rename = "EXCHANGE_ADSMOGO")]
    #[doc = "AdsMogo."]
    ExchangeAdsmogo,
    #[serde(rename = "EXCHANGE_ADSWIZZ")]
    #[doc = "AdsWizz."]
    ExchangeAdswizz,
    #[serde(rename = "EXCHANGE_BIDSWITCH")]
    #[doc = "BidSwitch."]
    ExchangeBidswitch,
    #[serde(rename = "EXCHANGE_BRIGHTROLL_DISPLAY")]
    #[doc = "BrightRoll Exchange for Display from Yahoo!."]
    ExchangeBrightrollDisplay,
    #[serde(rename = "EXCHANGE_CADREON")]
    #[doc = "Cadreon."]
    ExchangeCadreon,
    #[serde(rename = "EXCHANGE_DAILYMOTION")]
    #[doc = "Dailymotion."]
    ExchangeDailymotion,
    #[serde(rename = "EXCHANGE_FIVE")]
    #[doc = "Five."]
    ExchangeFive,
    #[serde(rename = "EXCHANGE_FLUCT")]
    #[doc = "Fluct."]
    ExchangeFluct,
    #[serde(rename = "EXCHANGE_FREEWHEEL")]
    #[doc = "FreeWheel SSP."]
    ExchangeFreewheel,
    #[serde(rename = "EXCHANGE_GENIEE")]
    #[doc = "Geniee."]
    ExchangeGeniee,
    #[serde(rename = "EXCHANGE_GUMGUM")]
    #[doc = "GumGum."]
    ExchangeGumgum,
    #[serde(rename = "EXCHANGE_IMOBILE")]
    #[doc = "i-mobile."]
    ExchangeImobile,
    #[serde(rename = "EXCHANGE_IBILLBOARD")]
    #[doc = "iBILLBOARD."]
    ExchangeIbillboard,
    #[serde(rename = "EXCHANGE_IMPROVE_DIGITAL")]
    #[doc = "Improve Digital."]
    ExchangeImproveDigital,
    #[serde(rename = "EXCHANGE_INDEX")]
    #[doc = "Index Exchange."]
    ExchangeIndex,
    #[serde(rename = "EXCHANGE_KARGO")]
    #[doc = "Kargo."]
    ExchangeKargo,
    #[serde(rename = "EXCHANGE_MICROAD")]
    #[doc = "MicroAd."]
    ExchangeMicroad,
    #[serde(rename = "EXCHANGE_MOPUB")]
    #[doc = "MoPub."]
    ExchangeMopub,
    #[serde(rename = "EXCHANGE_NEND")]
    #[doc = "Nend."]
    ExchangeNend,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_DISPLAY")]
    #[doc = "ONE by AOL: Display Market Place."]
    ExchangeOneByAolDisplay,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_MOBILE")]
    #[doc = "ONE by AOL: Mobile."]
    ExchangeOneByAolMobile,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_VIDEO")]
    #[doc = "ONE by AOL: Video."]
    ExchangeOneByAolVideo,
    #[serde(rename = "EXCHANGE_OOYALA")]
    #[doc = "Ooyala."]
    ExchangeOoyala,
    #[serde(rename = "EXCHANGE_OPENX")]
    #[doc = "OpenX."]
    ExchangeOpenx,
    #[serde(rename = "EXCHANGE_PERMODO")]
    #[doc = "Permodo."]
    ExchangePermodo,
    #[serde(rename = "EXCHANGE_PLATFORMONE")]
    #[doc = "Platform One."]
    ExchangePlatformone,
    #[serde(rename = "EXCHANGE_PLATFORMID")]
    #[doc = "PlatformId."]
    ExchangePlatformid,
    #[serde(rename = "EXCHANGE_PUBMATIC")]
    #[doc = "PubMatic."]
    ExchangePubmatic,
    #[serde(rename = "EXCHANGE_PULSEPOINT")]
    #[doc = "PulsePoint."]
    ExchangePulsepoint,
    #[serde(rename = "EXCHANGE_REVENUEMAX")]
    #[doc = "RevenueMax."]
    ExchangeRevenuemax,
    #[serde(rename = "EXCHANGE_RUBICON")]
    #[doc = "Rubicon."]
    ExchangeRubicon,
    #[serde(rename = "EXCHANGE_SMARTCLIP")]
    #[doc = "SmartClip."]
    ExchangeSmartclip,
    #[serde(rename = "EXCHANGE_SMARTRTB")]
    #[doc = "SmartRTB+."]
    ExchangeSmartrtb,
    #[serde(rename = "EXCHANGE_SMARTSTREAMTV")]
    #[doc = "SmartstreamTv."]
    ExchangeSmartstreamtv,
    #[serde(rename = "EXCHANGE_SOVRN")]
    #[doc = "Sovrn."]
    ExchangeSovrn,
    #[serde(rename = "EXCHANGE_SPOTXCHANGE")]
    #[doc = "SpotXchange."]
    ExchangeSpotxchange,
    #[serde(rename = "EXCHANGE_STROER")]
    #[doc = "Ströer SSP."]
    ExchangeStroer,
    #[serde(rename = "EXCHANGE_TEADSTV")]
    #[doc = "TeadsTv."]
    ExchangeTeadstv,
    #[serde(rename = "EXCHANGE_TELARIA")]
    #[doc = "Telaria."]
    ExchangeTelaria,
    #[serde(rename = "EXCHANGE_TVN")]
    #[doc = "TVN."]
    ExchangeTvn,
    #[serde(rename = "EXCHANGE_UNITED")]
    #[doc = "United."]
    ExchangeUnited,
    #[serde(rename = "EXCHANGE_YIELDLAB")]
    #[doc = "Yieldlab."]
    ExchangeYieldlab,
    #[serde(rename = "EXCHANGE_YIELDMO")]
    #[doc = "Yieldmo."]
    ExchangeYieldmo,
    #[serde(rename = "EXCHANGE_UNRULYX")]
    #[doc = "UnrulyX."]
    ExchangeUnrulyx,
    #[serde(rename = "EXCHANGE_OPEN8")]
    #[doc = "Open8."]
    ExchangeOpen8,
    #[serde(rename = "EXCHANGE_TRITON")]
    #[doc = "Triton."]
    ExchangeTriton,
    #[serde(rename = "EXCHANGE_TRIPLELIFT")]
    #[doc = "TripleLift."]
    ExchangeTriplelift,
    #[serde(rename = "EXCHANGE_TABOOLA")]
    #[doc = "Taboola."]
    ExchangeTaboola,
    #[serde(rename = "EXCHANGE_INMOBI")]
    #[doc = "InMobi."]
    ExchangeInmobi,
    #[serde(rename = "EXCHANGE_SMAATO")]
    #[doc = "Smaato."]
    ExchangeSmaato,
    #[serde(rename = "EXCHANGE_AJA")]
    #[doc = "Aja."]
    ExchangeAja,
    #[serde(rename = "EXCHANGE_SUPERSHIP")]
    #[doc = "Supership."]
    ExchangeSupership,
    #[serde(rename = "EXCHANGE_NEXSTAR_DIGITAL")]
    #[doc = "Nexstar Digital."]
    ExchangeNexstarDigital,
    #[serde(rename = "EXCHANGE_WAZE")]
    #[doc = "Waze."]
    ExchangeWaze,
    #[serde(rename = "EXCHANGE_SOUNDCAST")]
    #[doc = "SoundCast."]
    ExchangeSoundcast,
    #[serde(rename = "EXCHANGE_SHARETHROUGH")]
    #[doc = "Sharethrough."]
    ExchangeSharethrough,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the exchange review."]
pub enum ExchangeReviewStatusStatusEnum {
    #[serde(rename = "REVIEW_STATUS_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    ReviewStatusUnspecified,
    #[serde(rename = "REVIEW_STATUS_APPROVED")]
    #[doc = "The creative is approved."]
    ReviewStatusApproved,
    #[serde(rename = "REVIEW_STATUS_REJECTED")]
    #[doc = "The creative is rejected."]
    ReviewStatusRejected,
    #[serde(rename = "REVIEW_STATUS_PENDING")]
    #[doc = "The creative is pending review."]
    ReviewStatusPending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable exchange. This will be populated in the exchange_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_EXCHANGE`."]
pub struct ExchangeTargetingOptionDetails {
    #[serde(rename = "exchange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of exchange."]
    pub exchange: ::std::option::Option<ExchangeTargetingOptionDetailsExchangeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of exchange."]
pub enum ExchangeTargetingOptionDetailsExchangeEnum {
    #[serde(rename = "EXCHANGE_UNSPECIFIED")]
    #[doc = "Exchange is not specified or is unknown in this version."]
    ExchangeUnspecified,
    #[serde(rename = "EXCHANGE_GOOGLE_AD_MANAGER")]
    #[doc = "Google Ad Manager."]
    ExchangeGoogleAdManager,
    #[serde(rename = "EXCHANGE_APPNEXUS")]
    #[doc = "AppNexus."]
    ExchangeAppnexus,
    #[serde(rename = "EXCHANGE_BRIGHTROLL")]
    #[doc = "BrightRoll Exchange for Video from Yahoo!."]
    ExchangeBrightroll,
    #[serde(rename = "EXCHANGE_ADFORM")]
    #[doc = "Adform."]
    ExchangeAdform,
    #[serde(rename = "EXCHANGE_ADMETA")]
    #[doc = "Admeta."]
    ExchangeAdmeta,
    #[serde(rename = "EXCHANGE_ADMIXER")]
    #[doc = "Admixer."]
    ExchangeAdmixer,
    #[serde(rename = "EXCHANGE_ADSMOGO")]
    #[doc = "AdsMogo."]
    ExchangeAdsmogo,
    #[serde(rename = "EXCHANGE_ADSWIZZ")]
    #[doc = "AdsWizz."]
    ExchangeAdswizz,
    #[serde(rename = "EXCHANGE_BIDSWITCH")]
    #[doc = "BidSwitch."]
    ExchangeBidswitch,
    #[serde(rename = "EXCHANGE_BRIGHTROLL_DISPLAY")]
    #[doc = "BrightRoll Exchange for Display from Yahoo!."]
    ExchangeBrightrollDisplay,
    #[serde(rename = "EXCHANGE_CADREON")]
    #[doc = "Cadreon."]
    ExchangeCadreon,
    #[serde(rename = "EXCHANGE_DAILYMOTION")]
    #[doc = "Dailymotion."]
    ExchangeDailymotion,
    #[serde(rename = "EXCHANGE_FIVE")]
    #[doc = "Five."]
    ExchangeFive,
    #[serde(rename = "EXCHANGE_FLUCT")]
    #[doc = "Fluct."]
    ExchangeFluct,
    #[serde(rename = "EXCHANGE_FREEWHEEL")]
    #[doc = "FreeWheel SSP."]
    ExchangeFreewheel,
    #[serde(rename = "EXCHANGE_GENIEE")]
    #[doc = "Geniee."]
    ExchangeGeniee,
    #[serde(rename = "EXCHANGE_GUMGUM")]
    #[doc = "GumGum."]
    ExchangeGumgum,
    #[serde(rename = "EXCHANGE_IMOBILE")]
    #[doc = "i-mobile."]
    ExchangeImobile,
    #[serde(rename = "EXCHANGE_IBILLBOARD")]
    #[doc = "iBILLBOARD."]
    ExchangeIbillboard,
    #[serde(rename = "EXCHANGE_IMPROVE_DIGITAL")]
    #[doc = "Improve Digital."]
    ExchangeImproveDigital,
    #[serde(rename = "EXCHANGE_INDEX")]
    #[doc = "Index Exchange."]
    ExchangeIndex,
    #[serde(rename = "EXCHANGE_KARGO")]
    #[doc = "Kargo."]
    ExchangeKargo,
    #[serde(rename = "EXCHANGE_MICROAD")]
    #[doc = "MicroAd."]
    ExchangeMicroad,
    #[serde(rename = "EXCHANGE_MOPUB")]
    #[doc = "MoPub."]
    ExchangeMopub,
    #[serde(rename = "EXCHANGE_NEND")]
    #[doc = "Nend."]
    ExchangeNend,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_DISPLAY")]
    #[doc = "ONE by AOL: Display Market Place."]
    ExchangeOneByAolDisplay,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_MOBILE")]
    #[doc = "ONE by AOL: Mobile."]
    ExchangeOneByAolMobile,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_VIDEO")]
    #[doc = "ONE by AOL: Video."]
    ExchangeOneByAolVideo,
    #[serde(rename = "EXCHANGE_OOYALA")]
    #[doc = "Ooyala."]
    ExchangeOoyala,
    #[serde(rename = "EXCHANGE_OPENX")]
    #[doc = "OpenX."]
    ExchangeOpenx,
    #[serde(rename = "EXCHANGE_PERMODO")]
    #[doc = "Permodo."]
    ExchangePermodo,
    #[serde(rename = "EXCHANGE_PLATFORMONE")]
    #[doc = "Platform One."]
    ExchangePlatformone,
    #[serde(rename = "EXCHANGE_PLATFORMID")]
    #[doc = "PlatformId."]
    ExchangePlatformid,
    #[serde(rename = "EXCHANGE_PUBMATIC")]
    #[doc = "PubMatic."]
    ExchangePubmatic,
    #[serde(rename = "EXCHANGE_PULSEPOINT")]
    #[doc = "PulsePoint."]
    ExchangePulsepoint,
    #[serde(rename = "EXCHANGE_REVENUEMAX")]
    #[doc = "RevenueMax."]
    ExchangeRevenuemax,
    #[serde(rename = "EXCHANGE_RUBICON")]
    #[doc = "Rubicon."]
    ExchangeRubicon,
    #[serde(rename = "EXCHANGE_SMARTCLIP")]
    #[doc = "SmartClip."]
    ExchangeSmartclip,
    #[serde(rename = "EXCHANGE_SMARTRTB")]
    #[doc = "SmartRTB+."]
    ExchangeSmartrtb,
    #[serde(rename = "EXCHANGE_SMARTSTREAMTV")]
    #[doc = "SmartstreamTv."]
    ExchangeSmartstreamtv,
    #[serde(rename = "EXCHANGE_SOVRN")]
    #[doc = "Sovrn."]
    ExchangeSovrn,
    #[serde(rename = "EXCHANGE_SPOTXCHANGE")]
    #[doc = "SpotXchange."]
    ExchangeSpotxchange,
    #[serde(rename = "EXCHANGE_STROER")]
    #[doc = "Ströer SSP."]
    ExchangeStroer,
    #[serde(rename = "EXCHANGE_TEADSTV")]
    #[doc = "TeadsTv."]
    ExchangeTeadstv,
    #[serde(rename = "EXCHANGE_TELARIA")]
    #[doc = "Telaria."]
    ExchangeTelaria,
    #[serde(rename = "EXCHANGE_TVN")]
    #[doc = "TVN."]
    ExchangeTvn,
    #[serde(rename = "EXCHANGE_UNITED")]
    #[doc = "United."]
    ExchangeUnited,
    #[serde(rename = "EXCHANGE_YIELDLAB")]
    #[doc = "Yieldlab."]
    ExchangeYieldlab,
    #[serde(rename = "EXCHANGE_YIELDMO")]
    #[doc = "Yieldmo."]
    ExchangeYieldmo,
    #[serde(rename = "EXCHANGE_UNRULYX")]
    #[doc = "UnrulyX."]
    ExchangeUnrulyx,
    #[serde(rename = "EXCHANGE_OPEN8")]
    #[doc = "Open8."]
    ExchangeOpen8,
    #[serde(rename = "EXCHANGE_TRITON")]
    #[doc = "Triton."]
    ExchangeTriton,
    #[serde(rename = "EXCHANGE_TRIPLELIFT")]
    #[doc = "TripleLift."]
    ExchangeTriplelift,
    #[serde(rename = "EXCHANGE_TABOOLA")]
    #[doc = "Taboola."]
    ExchangeTaboola,
    #[serde(rename = "EXCHANGE_INMOBI")]
    #[doc = "InMobi."]
    ExchangeInmobi,
    #[serde(rename = "EXCHANGE_SMAATO")]
    #[doc = "Smaato."]
    ExchangeSmaato,
    #[serde(rename = "EXCHANGE_AJA")]
    #[doc = "Aja."]
    ExchangeAja,
    #[serde(rename = "EXCHANGE_SUPERSHIP")]
    #[doc = "Supership."]
    ExchangeSupership,
    #[serde(rename = "EXCHANGE_NEXSTAR_DIGITAL")]
    #[doc = "Nexstar Digital."]
    ExchangeNexstarDigital,
    #[serde(rename = "EXCHANGE_WAZE")]
    #[doc = "Waze."]
    ExchangeWaze,
    #[serde(rename = "EXCHANGE_SOUNDCAST")]
    #[doc = "SoundCast."]
    ExchangeSoundcast,
    #[serde(rename = "EXCHANGE_SHARETHROUGH")]
    #[doc = "Sharethrough."]
    ExchangeSharethrough,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Exit event of the creative."]
pub struct ExitEvent {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the click tag of the exit event. The name must be unique within one creative. Leave it empty or unset for creatives containing image assets only."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportingName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name used to identify this event in reports. Leave it empty or unset for creatives containing image assets only."]
    pub reporting_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the exit event."]
    pub _type: ::std::option::Option<ExitEventTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The click through URL of the exit event. This is required when type is: * `EXIT_EVENT_TYPE_DEFAULT` * `EXIT_EVENT_TYPE_BACKUP`"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the exit event."]
pub enum ExitEventTypeEnum {
    #[serde(rename = "EXIT_EVENT_TYPE_UNSPECIFIED")]
    #[doc = "Exit event type is not specified or is unknown in this version."]
    ExitEventTypeUnspecified,
    #[serde(rename = "EXIT_EVENT_TYPE_DEFAULT")]
    #[doc = "The exit event is the default one."]
    ExitEventTypeDefault,
    #[serde(rename = "EXIT_EVENT_TYPE_BACKUP")]
    #[doc = "The exit event is a backup exit event. There could be multiple backup exit events in a creative."]
    ExitEventTypeBackup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a first or third party audience list used for targeting. First party audiences are created via usage of client data. Third party audiences are provided by Third Party data providers and can only be licensed to customers."]
pub struct FirstAndThirdPartyAudience {
    #[serde(rename = "activeDisplayAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated audience size for the Display network in the past month. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only returned in GET request."]
    pub active_display_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "audienceSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The source of the audience."]
    pub audience_source: ::std::option::Option<FirstAndThirdPartyAudienceAudienceSourceEnum>,
    #[serde(rename = "audienceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the audience."]
    pub audience_type: ::std::option::Option<FirstAndThirdPartyAudienceAudienceTypeEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-provided description of the audience. Only applicable to first party audiences."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated audience size for the Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only returned in GET request."]
    pub display_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayDesktopAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated desktop audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request."]
    pub display_desktop_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayMobileAppAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated mobile app audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request."]
    pub display_mobile_app_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayMobileWebAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated mobile web audience size in Display network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request."]
    pub display_mobile_web_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the first and third party audience."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstAndThirdPartyAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the first and third party audience. Assigned by the system."]
    pub first_and_third_party_audience_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstAndThirdPartyAudienceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the audience is a first or third party audience."]
    pub first_and_third_party_audience_type:
        ::std::option::Option<FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum>,
    #[serde(rename = "gmailAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated audience size for Gmail network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request."]
    pub gmail_audience_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "membershipDurationDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration in days that an entry remains in the audience after the qualifying event. Only applicable to first party audiences."]
    pub membership_duration_days: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the first and third party audience."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "youtubeAudienceSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The estimated audience size for YouTube network. If the size is less than 1000, the number will be hidden and 0 will be returned due to privacy reasons. Otherwise, the number will be rounded off to two significant digits. Only applicable to first party audiences. Only returned in GET request."]
    pub youtube_audience_size: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The source of the audience."]
pub enum FirstAndThirdPartyAudienceAudienceSourceEnum {
    #[serde(rename = "AUDIENCE_SOURCE_UNSPECIFIED")]
    #[doc = "Default value when audience source is not specified or is unknown."]
    AudienceSourceUnspecified,
    #[serde(rename = "DISPLAY_VIDEO_360")]
    #[doc = "Originated from Display & Video 360."]
    DisplayVideo360,
    #[serde(rename = "CAMPAIGN_MANAGER")]
    #[doc = "Originated from Campaign Manager 360."]
    CampaignManager,
    #[serde(rename = "AD_MANAGER")]
    #[doc = "Originated from Google Ad Manager."]
    AdManager,
    #[serde(rename = "SEARCH_ADS_360")]
    #[doc = "Originated from Search Ads 360."]
    SearchAds360,
    #[serde(rename = "YOUTUBE")]
    #[doc = "Originated from Youtube."]
    Youtube,
    #[serde(rename = "ADS_DATA_HUB")]
    #[doc = "Originated from Ads Data Hub."]
    AdsDataHub,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of the audience."]
pub enum FirstAndThirdPartyAudienceAudienceTypeEnum {
    #[serde(rename = "AUDIENCE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown."]
    AudienceTypeUnspecified,
    #[serde(rename = "CUSTOMER_MATCH_CONTACT_INFO")]
    #[doc = "Audience was generated through matching customers to known contact information."]
    CustomerMatchContactInfo,
    #[serde(rename = "CUSTOMER_MATCH_DEVICE_ID")]
    #[doc = "Audience was generated through matching customers to known Mobile device IDs."]
    CustomerMatchDeviceId,
    #[serde(rename = "CUSTOMER_MATCH_USER_ID")]
    #[doc = "Audience was generated through matching customers to known User IDs."]
    CustomerMatchUserId,
    #[serde(rename = "ACTIVITY_BASED")]
    #[doc = "Audience was created based on campaign activity."]
    ActivityBased,
    #[serde(rename = "FREQUENCY_CAP")]
    #[doc = "Audience was created based on excluding the number of impressions they were served."]
    FrequencyCap,
    #[serde(rename = "TAG_BASED")]
    #[doc = "Audience was created based on custom variables attached to pixel."]
    TagBased,
    #[serde(rename = "YOUTUBE_USERS")]
    #[doc = "Audience was created based on past interactions with videos, YouTube ads, or YouTube channel."]
    YoutubeUsers,
    #[serde(rename = "LICENSED")]
    #[doc = "Subtype of third party audience type."]
    Licensed,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Whether the audience is a first or third party audience."]
pub enum FirstAndThirdPartyAudienceFirstAndThirdPartyAudienceTypeEnum {
    #[serde(rename = "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown."]
    FirstAndThirdPartyAudienceTypeUnspecified,
    #[serde(rename = "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_FIRST_PARTY")]
    #[doc = "Audience that is created via usage of client data."]
    FirstAndThirdPartyAudienceTypeFirstParty,
    #[serde(rename = "FIRST_AND_THIRD_PARTY_AUDIENCE_TYPE_THIRD_PARTY")]
    #[doc = "Audience that is provided by Third Party data providers."]
    FirstAndThirdPartyAudienceTypeThirdParty,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of first and third party audience group. All first and third party audience targeting settings are logically ‘OR’ of each other."]
pub struct FirstAndThirdPartyAudienceGroup {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All first and third party audience targeting settings in first and third party audience group. Repeated settings with same id are not allowed."]
    pub settings: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<FirstAndThirdPartyAudienceTargetingSetting>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of first and third party audience targeting setting."]
pub struct FirstAndThirdPartyAudienceTargetingSetting {
    #[serde(rename = "firstAndThirdPartyAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. First and third party audience id of the first and third party audience targeting setting. This id is first_and_third_party_audience_id."]
    pub first_and_third_party_audience_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recency of the first and third party audience targeting setting. Only applicable to first party audiences, otherwise will be ignored. For more info, refer to https://support.google.com/displayvideo/answer/2949947#recency When unspecified, no recency limit will be used."]
    pub recency: ::std::option::Option<FirstAndThirdPartyAudienceTargetingSettingRecencyEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The recency of the first and third party audience targeting setting. Only applicable to first party audiences, otherwise will be ignored. For more info, refer to https://support.google.com/displayvideo/answer/2949947#recency When unspecified, no recency limit will be used."]
pub enum FirstAndThirdPartyAudienceTargetingSettingRecencyEnum {
    #[serde(rename = "RECENCY_NO_LIMIT")]
    #[doc = "No limit of recency."]
    RecencyNoLimit,
    #[serde(rename = "RECENCY_1_MINUTE")]
    #[doc = "Recency is 1 minute."]
    Recency1Minute,
    #[serde(rename = "RECENCY_5_MINUTES")]
    #[doc = "Recency is 5 minutes."]
    Recency5Minutes,
    #[serde(rename = "RECENCY_10_MINUTES")]
    #[doc = "Recency is 10 minutes."]
    Recency10Minutes,
    #[serde(rename = "RECENCY_15_MINUTES")]
    #[doc = "Recency is 15 minutes."]
    Recency15Minutes,
    #[serde(rename = "RECENCY_30_MINUTES")]
    #[doc = "Recency is 30 minutes."]
    Recency30Minutes,
    #[serde(rename = "RECENCY_1_HOUR")]
    #[doc = "Recency is 1 hour."]
    Recency1Hour,
    #[serde(rename = "RECENCY_2_HOURS")]
    #[doc = "Recency is 2 hours."]
    Recency2Hours,
    #[serde(rename = "RECENCY_3_HOURS")]
    #[doc = "Recency is 3 hours."]
    Recency3Hours,
    #[serde(rename = "RECENCY_6_HOURS")]
    #[doc = "Recency is 6 hours."]
    Recency6Hours,
    #[serde(rename = "RECENCY_12_HOURS")]
    #[doc = "Recency is 12 hours."]
    Recency12Hours,
    #[serde(rename = "RECENCY_1_DAY")]
    #[doc = "Recency is 1 day."]
    Recency1Day,
    #[serde(rename = "RECENCY_2_DAYS")]
    #[doc = "Recency is 2 days."]
    Recency2Days,
    #[serde(rename = "RECENCY_3_DAYS")]
    #[doc = "Recency is 3 days."]
    Recency3Days,
    #[serde(rename = "RECENCY_5_DAYS")]
    #[doc = "Recency is 5 days."]
    Recency5Days,
    #[serde(rename = "RECENCY_7_DAYS")]
    #[doc = "Recency is 7 days."]
    Recency7Days,
    #[serde(rename = "RECENCY_10_DAYS")]
    #[doc = "Recency is 10 days."]
    Recency10Days,
    #[serde(rename = "RECENCY_14_DAYS")]
    #[doc = "Recency is 14 days."]
    Recency14Days,
    #[serde(rename = "RECENCY_15_DAYS")]
    #[doc = "Recency is 15 days."]
    Recency15Days,
    #[serde(rename = "RECENCY_21_DAYS")]
    #[doc = "Recency is 21 days."]
    Recency21Days,
    #[serde(rename = "RECENCY_28_DAYS")]
    #[doc = "Recency is 28 days."]
    Recency28Days,
    #[serde(rename = "RECENCY_30_DAYS")]
    #[doc = "Recency is 30 days."]
    Recency30Days,
    #[serde(rename = "RECENCY_40_DAYS")]
    #[doc = "Recency is 40 days."]
    Recency40Days,
    #[serde(rename = "RECENCY_60_DAYS")]
    #[doc = "Recency is 60 days."]
    Recency60Days,
    #[serde(rename = "RECENCY_90_DAYS")]
    #[doc = "Recency is 90 days."]
    Recency90Days,
    #[serde(rename = "RECENCY_120_DAYS")]
    #[doc = "Recency is 120 days."]
    Recency120Days,
    #[serde(rename = "RECENCY_180_DAYS")]
    #[doc = "Recency is 180 days."]
    Recency180Days,
    #[serde(rename = "RECENCY_270_DAYS")]
    #[doc = "Recency is 270 days."]
    Recency270Days,
    #[serde(rename = "RECENCY_365_DAYS")]
    #[doc = "Recency is 365 days."]
    Recency365Days,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A strategy that uses a fixed bidding price."]
pub struct FixedBidStrategy {
    #[serde(rename = "bidAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fixed bid amount, in micros of the advertiser's currency. For insertion order entity, bid_amount_micros should be set as 0. For line item entity, bid_amount_micros must be greater than or equal to billable unit of the given currency and smaller than or equal to the upper limit 1000000000. For example, 1500000 represents 1.5 standard units of the currency."]
    pub bid_amount_micros: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single Floodlight group."]
pub struct FloodlightGroup {
    #[serde(rename = "activeViewConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Active View video viewability metric configuration for the Floodlight group."]
    pub active_view_config:
        ::std::option::Option<::std::boxed::Box<ActiveViewVideoViewabilityMetricConfig>>,
    #[serde(rename = "customVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-defined custom variables owned by the Floodlight group. Use custom Floodlight variables to create reporting data that is tailored to your unique business needs. Custom Floodlight variables use the keys `U1=`, `U2=`, and so on, and can take any values that you choose to pass to them. You can use them to track virtually any type of data that you collect about your customers, such as the genre of movie that a customer purchases, the country to which the item is shipped, and so on. Custom Floodlight variables may not be used to pass any data that could be used or recognized as personally identifiable information (PII). Example: `custom_variables { fields { \"U1\": value { number_value: 123.4 }, \"U2\": value { string_value: \"MyVariable2\" }, \"U3\": value { string_value: \"MyVariable3\" } } }` Acceptable values for keys are \"U1\" through \"U100\", inclusive. String values must be less than 64 characters long, and cannot contain the following characters: `\"<>`."]
    pub custom_variables:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the Floodlight group."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the Floodlight group. Assigned by the system."]
    pub floodlight_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lookbackWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The lookback window for the Floodlight group. Both click_days and impression_days are required. Acceptable values for both are `0` to `90`, inclusive."]
    pub lookback_window: ::std::option::Option<::std::boxed::Box<LookbackWindow>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the Floodlight group."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webTagType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The web tag type enabled for the Floodlight group."]
    pub web_tag_type: ::std::option::Option<FloodlightGroupWebTagTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The web tag type enabled for the Floodlight group."]
pub enum FloodlightGroupWebTagTypeEnum {
    #[serde(rename = "WEB_TAG_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    WebTagTypeUnspecified,
    #[serde(rename = "WEB_TAG_TYPE_NONE")]
    #[doc = "No tag type."]
    WebTagTypeNone,
    #[serde(rename = "WEB_TAG_TYPE_IMAGE")]
    #[doc = "Image tag."]
    WebTagTypeImage,
    #[serde(rename = "WEB_TAG_TYPE_DYNAMIC")]
    #[doc = "Dynamic tag."]
    WebTagTypeDynamic,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the number of times a user may be shown with the same ad during a given time period."]
pub struct FrequencyCap {
    #[serde(rename = "maxImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of times a user may be shown with the same ad during this period. Must be greater than 0. Applicable when unlimited is `false`."]
    pub max_impressions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time unit in which the frequency cap will be applied. Applicable when unlimited is `false`."]
    pub time_unit: ::std::option::Option<FrequencyCapTimeUnitEnum>,
    #[serde(rename = "timeUnitCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of time_unit the frequency cap will last. Applicable when unlimited is `false`. The following restrictions apply based on the value of time_unit: * `TIME_UNIT_LIFETIME` - this field is output only and will default to 1 * `TIME_UNIT_MONTHS` - must be between 1 and 2 * `TIME_UNIT_WEEKS` - must be between 1 and 4 * `TIME_UNIT_DAYS` - must be between 1 and 6 * `TIME_UNIT_HOURS` - must be between 1 and 23 * `TIME_UNIT_MINUTES` - must be between 1 and 59"]
    pub time_unit_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "unlimited")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether unlimited frequency capping is applied. When this field is set to `true`, the remaining frequency cap fields are not applicable."]
    pub unlimited: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The time unit in which the frequency cap will be applied. Applicable when unlimited is `false`."]
pub enum FrequencyCapTimeUnitEnum {
    #[serde(rename = "TIME_UNIT_UNSPECIFIED")]
    #[doc = "Time unit value is not specified or is unknown in this version."]
    TimeUnitUnspecified,
    #[serde(rename = "TIME_UNIT_LIFETIME")]
    #[doc = "The frequency cap will be applied to the whole life time of the line item."]
    TimeUnitLifetime,
    #[serde(rename = "TIME_UNIT_MONTHS")]
    #[doc = "The frequency cap will be applied to a number of months."]
    TimeUnitMonths,
    #[serde(rename = "TIME_UNIT_WEEKS")]
    #[doc = "The frequency cap will be applied to a number of weeks."]
    TimeUnitWeeks,
    #[serde(rename = "TIME_UNIT_DAYS")]
    #[doc = "The frequency cap will be applied to a number of days."]
    TimeUnitDays,
    #[serde(rename = "TIME_UNIT_HOURS")]
    #[doc = "The frequency cap will be applied to a number of hours."]
    TimeUnitHours,
    #[serde(rename = "TIME_UNIT_MINUTES")]
    #[doc = "The frequency cap will be applied to a number of minutes."]
    TimeUnitMinutes,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned gender targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARTGETING_TYPE_GENDER`."]
pub struct GenderAssignedTargetingOptionDetails {
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The gender of the audience."]
    pub gender: ::std::option::Option<GenderAssignedTargetingOptionDetailsGenderEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_GENDER`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The gender of the audience."]
pub enum GenderAssignedTargetingOptionDetailsGenderEnum {
    #[serde(rename = "GENDER_UNSPECIFIED")]
    #[doc = "Default value when gender is not specified in this version. This enum is a place holder for default value and does not represent a real gender option."]
    GenderUnspecified,
    #[serde(rename = "GENDER_MALE")]
    #[doc = "The audience gender is male."]
    GenderMale,
    #[serde(rename = "GENDER_FEMALE")]
    #[doc = "The audience gender is female."]
    GenderFemale,
    #[serde(rename = "GENDER_UNKNOWN")]
    #[doc = "The audience gender is unknown."]
    GenderUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable gender. This will be populated in the gender_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_GENDER`."]
pub struct GenderTargetingOptionDetails {
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The gender of an audience."]
    pub gender: ::std::option::Option<GenderTargetingOptionDetailsGenderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The gender of an audience."]
pub enum GenderTargetingOptionDetailsGenderEnum {
    #[serde(rename = "GENDER_UNSPECIFIED")]
    #[doc = "Default value when gender is not specified in this version. This enum is a place holder for default value and does not represent a real gender option."]
    GenderUnspecified,
    #[serde(rename = "GENDER_MALE")]
    #[doc = "The audience gender is male."]
    GenderMale,
    #[serde(rename = "GENDER_FEMALE")]
    #[doc = "The audience gender is female."]
    GenderFemale,
    #[serde(rename = "GENDER_UNKNOWN")]
    #[doc = "The audience gender is unknown."]
    GenderUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned geographic region targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_GEO_REGION`."]
pub struct GeoRegionAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the geographic region (e.g., \"Ontario, Canada\")."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoRegionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of geographic region targeting."]
    pub geo_region_type:
        ::std::option::Option<GeoRegionAssignedTargetingOptionDetailsGeoRegionTypeEnum>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_GEO_REGION`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of geographic region targeting."]
pub enum GeoRegionAssignedTargetingOptionDetailsGeoRegionTypeEnum {
    #[serde(rename = "GEO_REGION_TYPE_UNKNOWN")]
    #[doc = "The geographic region type is unknown."]
    GeoRegionTypeUnknown,
    #[serde(rename = "GEO_REGION_TYPE_OTHER")]
    #[doc = "The geographic region type is other."]
    GeoRegionTypeOther,
    #[serde(rename = "GEO_REGION_TYPE_COUNTRY")]
    #[doc = "The geographic region is a country."]
    GeoRegionTypeCountry,
    #[serde(rename = "GEO_REGION_TYPE_REGION")]
    #[doc = "The geographic region type is region."]
    GeoRegionTypeRegion,
    #[serde(rename = "GEO_REGION_TYPE_TERRITORY")]
    #[doc = "The geographic region is a territory."]
    GeoRegionTypeTerritory,
    #[serde(rename = "GEO_REGION_TYPE_PROVINCE")]
    #[doc = "The geographic region is a province."]
    GeoRegionTypeProvince,
    #[serde(rename = "GEO_REGION_TYPE_STATE")]
    #[doc = "The geographic region is a state."]
    GeoRegionTypeState,
    #[serde(rename = "GEO_REGION_TYPE_PREFECTURE")]
    #[doc = "The geographic region is a prefecture."]
    GeoRegionTypePrefecture,
    #[serde(rename = "GEO_REGION_TYPE_GOVERNORATE")]
    #[doc = "The geographic region is a governorate."]
    GeoRegionTypeGovernorate,
    #[serde(rename = "GEO_REGION_TYPE_CANTON")]
    #[doc = "The geographic region is a canton."]
    GeoRegionTypeCanton,
    #[serde(rename = "GEO_REGION_TYPE_UNION_TERRITORY")]
    #[doc = "The geographic region is a union territory."]
    GeoRegionTypeUnionTerritory,
    #[serde(rename = "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY")]
    #[doc = "The geographic region is an autonomous community."]
    GeoRegionTypeAutonomousCommunity,
    #[serde(rename = "GEO_REGION_TYPE_DMA_REGION")]
    #[doc = "The geographic region is a designated market area (DMA) region."]
    GeoRegionTypeDmaRegion,
    #[serde(rename = "GEO_REGION_TYPE_METRO")]
    #[doc = "The geographic region type is metro."]
    GeoRegionTypeMetro,
    #[serde(rename = "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT")]
    #[doc = "The geographic region is a congressional district."]
    GeoRegionTypeCongressionalDistrict,
    #[serde(rename = "GEO_REGION_TYPE_COUNTY")]
    #[doc = "The geographic region is a county."]
    GeoRegionTypeCounty,
    #[serde(rename = "GEO_REGION_TYPE_MUNICIPALITY")]
    #[doc = "The geographic region is a municipality."]
    GeoRegionTypeMunicipality,
    #[serde(rename = "GEO_REGION_TYPE_CITY")]
    #[doc = "The geographic region is a city."]
    GeoRegionTypeCity,
    #[serde(rename = "GEO_REGION_TYPE_POSTAL_CODE")]
    #[doc = "The geographic region targeting type is postal code."]
    GeoRegionTypePostalCode,
    #[serde(rename = "GEO_REGION_TYPE_DEPARTMENT")]
    #[doc = "The geographic region targeting type is department."]
    GeoRegionTypeDepartment,
    #[serde(rename = "GEO_REGION_TYPE_AIRPORT")]
    #[doc = "The geographic region is an airport."]
    GeoRegionTypeAirport,
    #[serde(rename = "GEO_REGION_TYPE_TV_REGION")]
    #[doc = "The geographic region is a TV region."]
    GeoRegionTypeTvRegion,
    #[serde(rename = "GEO_REGION_TYPE_OKRUG")]
    #[doc = "The geographic region is an okrug."]
    GeoRegionTypeOkrug,
    #[serde(rename = "GEO_REGION_TYPE_BOROUGH")]
    #[doc = "The geographic region is a borough."]
    GeoRegionTypeBorough,
    #[serde(rename = "GEO_REGION_TYPE_CITY_REGION")]
    #[doc = "The geographic region is a city region."]
    GeoRegionTypeCityRegion,
    #[serde(rename = "GEO_REGION_TYPE_ARRONDISSEMENT")]
    #[doc = "The geographic region is an arrondissement."]
    GeoRegionTypeArrondissement,
    #[serde(rename = "GEO_REGION_TYPE_NEIGHBORHOOD")]
    #[doc = "The geographic region is a neighborhood."]
    GeoRegionTypeNeighborhood,
    #[serde(rename = "GEO_REGION_TYPE_UNIVERSITY")]
    #[doc = "The geographic region is a university."]
    GeoRegionTypeUniversity,
    #[serde(rename = "GEO_REGION_TYPE_DISTRICT")]
    #[doc = "The geographic region is a district."]
    GeoRegionTypeDistrict,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Search terms for geo region targeting options."]
pub struct GeoRegionSearchTerms {
    #[serde(rename = "geoRegionQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search query for the desired geo region. The query can be a prefix, e.g. \"New Yor\", \"Seattle\", \"USA\", etc."]
    pub geo_region_query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable geographic region. This will be populated in the geo_region_details field when targeting_type is `TARGETING_TYPE_GEO_REGION`."]
pub struct GeoRegionTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the geographic region (e.g., \"Ontario, Canada\")."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoRegionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of geographic region targeting."]
    pub geo_region_type: ::std::option::Option<GeoRegionTargetingOptionDetailsGeoRegionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of geographic region targeting."]
pub enum GeoRegionTargetingOptionDetailsGeoRegionTypeEnum {
    #[serde(rename = "GEO_REGION_TYPE_UNKNOWN")]
    #[doc = "The geographic region type is unknown."]
    GeoRegionTypeUnknown,
    #[serde(rename = "GEO_REGION_TYPE_OTHER")]
    #[doc = "The geographic region type is other."]
    GeoRegionTypeOther,
    #[serde(rename = "GEO_REGION_TYPE_COUNTRY")]
    #[doc = "The geographic region is a country."]
    GeoRegionTypeCountry,
    #[serde(rename = "GEO_REGION_TYPE_REGION")]
    #[doc = "The geographic region type is region."]
    GeoRegionTypeRegion,
    #[serde(rename = "GEO_REGION_TYPE_TERRITORY")]
    #[doc = "The geographic region is a territory."]
    GeoRegionTypeTerritory,
    #[serde(rename = "GEO_REGION_TYPE_PROVINCE")]
    #[doc = "The geographic region is a province."]
    GeoRegionTypeProvince,
    #[serde(rename = "GEO_REGION_TYPE_STATE")]
    #[doc = "The geographic region is a state."]
    GeoRegionTypeState,
    #[serde(rename = "GEO_REGION_TYPE_PREFECTURE")]
    #[doc = "The geographic region is a prefecture."]
    GeoRegionTypePrefecture,
    #[serde(rename = "GEO_REGION_TYPE_GOVERNORATE")]
    #[doc = "The geographic region is a governorate."]
    GeoRegionTypeGovernorate,
    #[serde(rename = "GEO_REGION_TYPE_CANTON")]
    #[doc = "The geographic region is a canton."]
    GeoRegionTypeCanton,
    #[serde(rename = "GEO_REGION_TYPE_UNION_TERRITORY")]
    #[doc = "The geographic region is a union territory."]
    GeoRegionTypeUnionTerritory,
    #[serde(rename = "GEO_REGION_TYPE_AUTONOMOUS_COMMUNITY")]
    #[doc = "The geographic region is an autonomous community."]
    GeoRegionTypeAutonomousCommunity,
    #[serde(rename = "GEO_REGION_TYPE_DMA_REGION")]
    #[doc = "The geographic region is a designated market area (DMA) region."]
    GeoRegionTypeDmaRegion,
    #[serde(rename = "GEO_REGION_TYPE_METRO")]
    #[doc = "The geographic region type is metro."]
    GeoRegionTypeMetro,
    #[serde(rename = "GEO_REGION_TYPE_CONGRESSIONAL_DISTRICT")]
    #[doc = "The geographic region is a congressional district."]
    GeoRegionTypeCongressionalDistrict,
    #[serde(rename = "GEO_REGION_TYPE_COUNTY")]
    #[doc = "The geographic region is a county."]
    GeoRegionTypeCounty,
    #[serde(rename = "GEO_REGION_TYPE_MUNICIPALITY")]
    #[doc = "The geographic region is a municipality."]
    GeoRegionTypeMunicipality,
    #[serde(rename = "GEO_REGION_TYPE_CITY")]
    #[doc = "The geographic region is a city."]
    GeoRegionTypeCity,
    #[serde(rename = "GEO_REGION_TYPE_POSTAL_CODE")]
    #[doc = "The geographic region targeting type is postal code."]
    GeoRegionTypePostalCode,
    #[serde(rename = "GEO_REGION_TYPE_DEPARTMENT")]
    #[doc = "The geographic region targeting type is department."]
    GeoRegionTypeDepartment,
    #[serde(rename = "GEO_REGION_TYPE_AIRPORT")]
    #[doc = "The geographic region is an airport."]
    GeoRegionTypeAirport,
    #[serde(rename = "GEO_REGION_TYPE_TV_REGION")]
    #[doc = "The geographic region is a TV region."]
    GeoRegionTypeTvRegion,
    #[serde(rename = "GEO_REGION_TYPE_OKRUG")]
    #[doc = "The geographic region is an okrug."]
    GeoRegionTypeOkrug,
    #[serde(rename = "GEO_REGION_TYPE_BOROUGH")]
    #[doc = "The geographic region is a borough."]
    GeoRegionTypeBorough,
    #[serde(rename = "GEO_REGION_TYPE_CITY_REGION")]
    #[doc = "The geographic region is a city region."]
    GeoRegionTypeCityRegion,
    #[serde(rename = "GEO_REGION_TYPE_ARRONDISSEMENT")]
    #[doc = "The geographic region is an arrondissement."]
    GeoRegionTypeArrondissement,
    #[serde(rename = "GEO_REGION_TYPE_NEIGHBORHOOD")]
    #[doc = "The geographic region is a neighborhood."]
    GeoRegionTypeNeighborhood,
    #[serde(rename = "GEO_REGION_TYPE_UNIVERSITY")]
    #[doc = "The geographic region is a university."]
    GeoRegionTypeUniversity,
    #[serde(rename = "GEO_REGION_TYPE_DISTRICT")]
    #[doc = "The geographic region is a district."]
    GeoRegionTypeDistrict,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a Google audience resource. Includes Google audience lists."]
pub struct GoogleAudience {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the Google audience. ."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the Google audience. Assigned by the system."]
    pub google_audience_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleAudienceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of Google audience. ."]
    pub google_audience_type: ::std::option::Option<GoogleAudienceGoogleAudienceTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the google audience."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of Google audience. ."]
pub enum GoogleAudienceGoogleAudienceTypeEnum {
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown."]
    GoogleAudienceTypeUnspecified,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_AFFINITY")]
    #[doc = "Affinity type Google audience."]
    GoogleAudienceTypeAffinity,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_IN_MARKET")]
    #[doc = "In-Market type Google audience."]
    GoogleAudienceTypeInMarket,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_INSTALLED_APPS")]
    #[doc = "Installed-Apps type Google audience."]
    GoogleAudienceTypeInstalledApps,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_NEW_MOBILE_DEVICES")]
    #[doc = "New-Mobile-Devices type Google audience."]
    GoogleAudienceTypeNewMobileDevices,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_LIFE_EVENT")]
    #[doc = "Life-Event type Google audience."]
    GoogleAudienceTypeLifeEvent,
    #[serde(rename = "GOOGLE_AUDIENCE_TYPE_EXTENDED_DEMOGRAPHIC")]
    #[doc = "Extended-Demographic type Google audience."]
    GoogleAudienceTypeExtendedDemographic,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of Google audience group. All Google audience targeting settings are logically ‘OR’ of each other."]
pub struct GoogleAudienceGroup {
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All Google audience targeting settings in Google audience group. Repeated settings with same id will be ignored."]
    pub settings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAudienceTargetingSetting>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of Google audience targeting setting."]
pub struct GoogleAudienceTargetingSetting {
    #[serde(rename = "googleAudienceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Google audience id of the Google audience targeting setting. This id is google_audience_id."]
    pub google_audience_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Media resource."]
pub struct GoogleBytestreamMedia {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the media resource."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned household income targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_HOUSEHOLD_INCOME`."]
pub struct HouseholdIncomeAssignedTargetingOptionDetails {
    #[serde(rename = "householdIncome")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The household income of the audience."]
    pub household_income:
        ::std::option::Option<HouseholdIncomeAssignedTargetingOptionDetailsHouseholdIncomeEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_HOUSEHOLD_INCOME`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The household income of the audience."]
pub enum HouseholdIncomeAssignedTargetingOptionDetailsHouseholdIncomeEnum {
    #[serde(rename = "HOUSEHOLD_INCOME_UNSPECIFIED")]
    #[doc = "Default value when household income is not specified in this version. This enum is a placeholder for default value and does not represent a real household income option."]
    HouseholdIncomeUnspecified,
    #[serde(rename = "HOUSEHOLD_INCOME_UNKNOWN")]
    #[doc = "The household income of the audience is unknown."]
    HouseholdIncomeUnknown,
    #[serde(rename = "HOUSEHOLD_INCOME_LOWER_50_PERCENT")]
    #[doc = "The audience is in the lower 50% of U.S. household incomes."]
    HouseholdIncomeLower50Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT")]
    #[doc = "The audience is in the top 41-50% of U.S. household incomes."]
    HouseholdIncomeTop41To50Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT")]
    #[doc = "The audience is in the top 31-40% of U.S. household incomes."]
    HouseholdIncomeTop31To40Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT")]
    #[doc = "The audience is in the top 21-30% of U.S. household incomes."]
    HouseholdIncomeTop21To30Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT")]
    #[doc = "The audience is in the top 11-20% of U.S. household incomes."]
    HouseholdIncomeTop11To20Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_10_PERCENT")]
    #[doc = "The audience is in the top 10% of U.S. household incomes."]
    HouseholdIncomeTop10Percent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable household income. This will be populated in the household_income_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_HOUSEHOLD_INCOME`."]
pub struct HouseholdIncomeTargetingOptionDetails {
    #[serde(rename = "householdIncome")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The household income of an audience."]
    pub household_income:
        ::std::option::Option<HouseholdIncomeTargetingOptionDetailsHouseholdIncomeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The household income of an audience."]
pub enum HouseholdIncomeTargetingOptionDetailsHouseholdIncomeEnum {
    #[serde(rename = "HOUSEHOLD_INCOME_UNSPECIFIED")]
    #[doc = "Default value when household income is not specified in this version. This enum is a placeholder for default value and does not represent a real household income option."]
    HouseholdIncomeUnspecified,
    #[serde(rename = "HOUSEHOLD_INCOME_UNKNOWN")]
    #[doc = "The household income of the audience is unknown."]
    HouseholdIncomeUnknown,
    #[serde(rename = "HOUSEHOLD_INCOME_LOWER_50_PERCENT")]
    #[doc = "The audience is in the lower 50% of U.S. household incomes."]
    HouseholdIncomeLower50Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_41_TO_50_PERCENT")]
    #[doc = "The audience is in the top 41-50% of U.S. household incomes."]
    HouseholdIncomeTop41To50Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_31_TO_40_PERCENT")]
    #[doc = "The audience is in the top 31-40% of U.S. household incomes."]
    HouseholdIncomeTop31To40Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_21_TO_30_PERCENT")]
    #[doc = "The audience is in the top 21-30% of U.S. household incomes."]
    HouseholdIncomeTop21To30Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_11_TO_20_PERCENT")]
    #[doc = "The audience is in the top 11-20% of U.S. household incomes."]
    HouseholdIncomeTop11To20Percent,
    #[serde(rename = "HOUSEHOLD_INCOME_TOP_10_PERCENT")]
    #[doc = "The audience is in the top 10% of U.S. household incomes."]
    HouseholdIncomeTop10Percent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filtering option that filters entities by their entity IDs."]
pub struct IdFilter {
    #[serde(rename = "adGroupAdIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "YouTube Ads to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub ad_group_ad_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "adGroupIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "YouTube Ad Groups to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub ad_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "campaignIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaigns to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub campaign_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "insertionOrderIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Insertion Orders to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub insertion_order_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lineItemIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line Items to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub line_item_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mediaProductIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Media Products to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    pub media_product_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single insertion order."]
pub struct InsertionOrder {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser the insertion order belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bidStrategy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bidding strategy of the insertion order. By default, fixed_bid is set."]
    pub bid_strategy: ::std::option::Option<::std::boxed::Box<BiddingStrategy>>,
    #[serde(rename = "budget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The budget allocation settings of the insertion order."]
    pub budget: ::std::option::Option<::std::boxed::Box<InsertionOrderBudget>>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The unique ID of the campaign that the insertion order belongs to."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the insertion order. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Controls whether or not the insertion order can spend its budget and bid on inventory. * For CreateInsertionOrder method, only `ENTITY_STATUS_DRAFT` is allowed. To activate an insertion order, use UpdateInsertionOrder method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * An insertion order cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * An insertion order cannot be set to `ENTITY_STATUS_ACTIVE` if its parent campaign is not active."]
    pub entity_status: ::std::option::Option<InsertionOrderEntityStatusEnum>,
    #[serde(rename = "frequencyCap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The frequency capping setting of the insertion order."]
    pub frequency_cap: ::std::option::Option<::std::boxed::Box<FrequencyCap>>,
    #[serde(rename = "insertionOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the insertion order. Assigned by the system."]
    pub insertion_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertionOrderType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of insertion order. If this field is unspecified in creation, the value defaults to `RTB`."]
    pub insertion_order_type: ::std::option::Option<InsertionOrderInsertionOrderTypeEnum>,
    #[serde(rename = "integrationDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional integration details of the insertion order."]
    pub integration_details: ::std::option::Option<::std::boxed::Box<IntegrationDetails>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the insertion order."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pacing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The budget spending speed setting of the insertion order."]
    pub pacing: ::std::option::Option<::std::boxed::Box<Pacing>>,
    #[serde(rename = "partnerCosts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partner costs associated with the insertion order. If absent or empty in CreateInsertionOrder method, the newly created insertion order will inherit partner costs from the partner settings."]
    pub partner_costs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartnerCost>>>,
    #[serde(rename = "performanceGoal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Performance goal of the insertion order."]
    pub performance_goal: ::std::option::Option<::std::boxed::Box<PerformanceGoal>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the insertion order was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Controls whether or not the insertion order can spend its budget and bid on inventory. * For CreateInsertionOrder method, only `ENTITY_STATUS_DRAFT` is allowed. To activate an insertion order, use UpdateInsertionOrder method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * An insertion order cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * An insertion order cannot be set to `ENTITY_STATUS_ACTIVE` if its parent campaign is not active."]
pub enum InsertionOrderEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of insertion order. If this field is unspecified in creation, the value defaults to `RTB`."]
pub enum InsertionOrderInsertionOrderTypeEnum {
    #[serde(rename = "INSERTION_ORDER_TYPE_UNSPECIFIED")]
    #[doc = "Insertion order type is not specified or is unknown."]
    InsertionOrderTypeUnspecified,
    #[serde(rename = "RTB")]
    #[doc = "Real-time bidding."]
    Rtb,
    #[serde(rename = "OVER_THE_TOP")]
    #[doc = "Over-the-top."]
    OverTheTop,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how insertion order budget is allocated."]
pub struct InsertionOrderBudget {
    #[serde(rename = "automationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of automation used to manage bid and budget for the insertion order. If this field is unspecified in creation, the value defaults to `INSERTION_ORDER_AUTOMATION_TYPE_NONE`."]
    pub automation_type: ::std::option::Option<InsertionOrderBudgetAutomationTypeEnum>,
    #[serde(rename = "budgetSegments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of budget segments. Use a budget segment to specify a specific budget for a given period of time an insertion order is running."]
    pub budget_segments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InsertionOrderBudgetSegment>>>,
    #[serde(rename = "budgetUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The budget unit specifies whether the budget is currency based or impression based."]
    pub budget_unit: ::std::option::Option<InsertionOrderBudgetBudgetUnitEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of automation used to manage bid and budget for the insertion order. If this field is unspecified in creation, the value defaults to `INSERTION_ORDER_AUTOMATION_TYPE_NONE`."]
pub enum InsertionOrderBudgetAutomationTypeEnum {
    #[serde(rename = "INSERTION_ORDER_AUTOMATION_TYPE_UNSPECIFIED")]
    #[doc = "Insertion order automation option is not specified or is unknown in this version."]
    InsertionOrderAutomationTypeUnspecified,
    #[serde(rename = "INSERTION_ORDER_AUTOMATION_TYPE_BUDGET")]
    #[doc = "Automatic budget allocation. Allow the system to automatically shift budget to owning line items to optimize performance defined by performance_goal. No automation on bid settings."]
    InsertionOrderAutomationTypeBudget,
    #[serde(rename = "INSERTION_ORDER_AUTOMATION_TYPE_NONE")]
    #[doc = "No automation of bid or budget on insertion order level. Bid and budget must be manually configured at the line item level."]
    InsertionOrderAutomationTypeNone,
    #[serde(rename = "INSERTION_ORDER_AUTOMATION_TYPE_BID_BUDGET")]
    #[doc = "Allow the system to automatically adjust bids and shift budget to owning line items to optimize performance defined by performance_goal."]
    InsertionOrderAutomationTypeBidBudget,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The budget unit specifies whether the budget is currency based or impression based."]
pub enum InsertionOrderBudgetBudgetUnitEnum {
    #[serde(rename = "BUDGET_UNIT_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    BudgetUnitUnspecified,
    #[serde(rename = "BUDGET_UNIT_CURRENCY")]
    #[doc = "Budgeting in currency amounts."]
    BudgetUnitCurrency,
    #[serde(rename = "BUDGET_UNIT_IMPRESSIONS")]
    #[doc = "Budgeting in impression amounts."]
    BudgetUnitImpressions,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the budget of a single budget segment."]
pub struct InsertionOrderBudgetSegment {
    #[serde(rename = "budgetAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The budget amount the insertion order will spend for the given date_range. The amount is in micros. Must be greater than 0. For example, 500000000 represents 500 standard units of the currency."]
    pub budget_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignBudgetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the campaign budget linked to this insertion order budget segment."]
    pub campaign_budget_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The start and end date settings of the budget segment. They are resolved relative to the parent advertiser's time zone. * When creating a new budget segment, both `start_date` and `end_date` must be in the future. * An existing budget segment with a `start_date` in the past has a mutable `end_date` but an immutable `start_date`. * `end_date` must be the `start_date` or later, both before the year 2037."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The budget segment description. It can be used to enter Purchase Order information for each budget segment and have that information printed on the invoices. Must be UTF-8 encoded with a length of no more than 80 characters."]
    pub description: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of Integral Ad Science settings."]
pub struct IntegralAdScience {
    #[serde(rename = "customSegmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom segment ID provided by Integral Ad Science. The ID must be between `1000001` and `1999999`, inclusive."]
    pub custom_segment_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayViewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display Viewability section (applicable to display line items only)."]
    pub display_viewability: ::std::option::Option<IntegralAdScienceDisplayViewabilityEnum>,
    #[serde(rename = "excludeUnrateable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Unrateable**."]
    pub exclude_unrateable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "excludedAdFraudRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad Fraud settings."]
    pub excluded_ad_fraud_risk: ::std::option::Option<IntegralAdScienceExcludedAdFraudRiskEnum>,
    #[serde(rename = "excludedAdultRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Adult content**."]
    pub excluded_adult_risk: ::std::option::Option<IntegralAdScienceExcludedAdultRiskEnum>,
    #[serde(rename = "excludedAlcoholRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Alcohol**."]
    pub excluded_alcohol_risk: ::std::option::Option<IntegralAdScienceExcludedAlcoholRiskEnum>,
    #[serde(rename = "excludedDrugsRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Drugs**."]
    pub excluded_drugs_risk: ::std::option::Option<IntegralAdScienceExcludedDrugsRiskEnum>,
    #[serde(rename = "excludedGamblingRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Gambling**."]
    pub excluded_gambling_risk: ::std::option::Option<IntegralAdScienceExcludedGamblingRiskEnum>,
    #[serde(rename = "excludedHateSpeechRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Hate speech**."]
    pub excluded_hate_speech_risk:
        ::std::option::Option<IntegralAdScienceExcludedHateSpeechRiskEnum>,
    #[serde(rename = "excludedIllegalDownloadsRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Illegal downloads**."]
    pub excluded_illegal_downloads_risk:
        ::std::option::Option<IntegralAdScienceExcludedIllegalDownloadsRiskEnum>,
    #[serde(rename = "excludedOffensiveLanguageRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Offensive language**."]
    pub excluded_offensive_language_risk:
        ::std::option::Option<IntegralAdScienceExcludedOffensiveLanguageRiskEnum>,
    #[serde(rename = "excludedViolenceRisk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand Safety - **Violence**."]
    pub excluded_violence_risk: ::std::option::Option<IntegralAdScienceExcludedViolenceRiskEnum>,
    #[serde(rename = "traqScoreOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True advertising quality (applicable to Display line items only)."]
    pub traq_score_option: ::std::option::Option<IntegralAdScienceTraqScoreOptionEnum>,
    #[serde(rename = "videoViewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video Viewability Section (applicable to video line items only)."]
    pub video_viewability: ::std::option::Option<IntegralAdScienceVideoViewabilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Display Viewability section (applicable to display line items only)."]
pub enum IntegralAdScienceDisplayViewabilityEnum {
    #[serde(rename = "PERFORMANCE_VIEWABILITY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any display viewability options."]
    PerformanceViewabilityUnspecified,
    #[serde(rename = "PERFORMANCE_VIEWABILITY_40")]
    #[doc = "Target 40% Viewability or Higher."]
    PerformanceViewability40,
    #[serde(rename = "PERFORMANCE_VIEWABILITY_50")]
    #[doc = "Target 50% Viewability or Higher."]
    PerformanceViewability50,
    #[serde(rename = "PERFORMANCE_VIEWABILITY_60")]
    #[doc = "Target 60% Viewability or Higher."]
    PerformanceViewability60,
    #[serde(rename = "PERFORMANCE_VIEWABILITY_70")]
    #[doc = "Target 70% Viewability or Higher."]
    PerformanceViewability70,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Ad Fraud settings."]
pub enum IntegralAdScienceExcludedAdFraudRiskEnum {
    #[serde(rename = "SUSPICIOUS_ACTIVITY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any ad fraud prevention options."]
    SuspiciousActivityUnspecified,
    #[serde(rename = "SUSPICIOUS_ACTIVITY_HR")]
    #[doc = "Ad Fraud - Exclude High Risk."]
    SuspiciousActivityHr,
    #[serde(rename = "SUSPICIOUS_ACTIVITY_HMR")]
    #[doc = "Ad Fraud - Exclude High and Moderate Risk."]
    SuspiciousActivityHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Adult content**."]
pub enum IntegralAdScienceExcludedAdultRiskEnum {
    #[serde(rename = "ADULT_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any adult options."]
    AdultUnspecified,
    #[serde(rename = "ADULT_HR")]
    #[doc = "Adult - Exclude High Risk."]
    AdultHr,
    #[serde(rename = "ADULT_HMR")]
    #[doc = "Adult - Exclude High and Moderate Risk."]
    AdultHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Alcohol**."]
pub enum IntegralAdScienceExcludedAlcoholRiskEnum {
    #[serde(rename = "ALCOHOL_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any alcohol options."]
    AlcoholUnspecified,
    #[serde(rename = "ALCOHOL_HR")]
    #[doc = "Alcohol - Exclude High Risk."]
    AlcoholHr,
    #[serde(rename = "ALCOHOL_HMR")]
    #[doc = "Alcohol - Exclude High and Moderate Risk."]
    AlcoholHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Drugs**."]
pub enum IntegralAdScienceExcludedDrugsRiskEnum {
    #[serde(rename = "DRUGS_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any drugs options."]
    DrugsUnspecified,
    #[serde(rename = "DRUGS_HR")]
    #[doc = "Drugs - Exclude High Risk."]
    DrugsHr,
    #[serde(rename = "DRUGS_HMR")]
    #[doc = "Drugs - Exclude High and Moderate Risk."]
    DrugsHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Gambling**."]
pub enum IntegralAdScienceExcludedGamblingRiskEnum {
    #[serde(rename = "GAMBLING_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any gambling options."]
    GamblingUnspecified,
    #[serde(rename = "GAMBLING_HR")]
    #[doc = "Gambling - Exclude High Risk."]
    GamblingHr,
    #[serde(rename = "GAMBLING_HMR")]
    #[doc = "Gambling - Exclude High and Moderate Risk."]
    GamblingHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Hate speech**."]
pub enum IntegralAdScienceExcludedHateSpeechRiskEnum {
    #[serde(rename = "HATE_SPEECH_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any hate speech options."]
    HateSpeechUnspecified,
    #[serde(rename = "HATE_SPEECH_HR")]
    #[doc = "Hate Speech - Exclude High Risk."]
    HateSpeechHr,
    #[serde(rename = "HATE_SPEECH_HMR")]
    #[doc = "Hate Speech - Exclude High and Moderate Risk."]
    HateSpeechHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Illegal downloads**."]
pub enum IntegralAdScienceExcludedIllegalDownloadsRiskEnum {
    #[serde(rename = "ILLEGAL_DOWNLOADS_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any illegal downloads options."]
    IllegalDownloadsUnspecified,
    #[serde(rename = "ILLEGAL_DOWNLOADS_HR")]
    #[doc = "Illegal Downloads - Exclude High Risk."]
    IllegalDownloadsHr,
    #[serde(rename = "ILLEGAL_DOWNLOADS_HMR")]
    #[doc = "Illegal Downloads - Exclude High and Moderate Risk."]
    IllegalDownloadsHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Offensive language**."]
pub enum IntegralAdScienceExcludedOffensiveLanguageRiskEnum {
    #[serde(rename = "OFFENSIVE_LANGUAGE_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any language options."]
    OffensiveLanguageUnspecified,
    #[serde(rename = "OFFENSIVE_LANGUAGE_HR")]
    #[doc = "Offensive Language - Exclude High Risk."]
    OffensiveLanguageHr,
    #[serde(rename = "OFFENSIVE_LANGUAGE_HMR")]
    #[doc = "Offensive Language - Exclude High and Moderate Risk."]
    OffensiveLanguageHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Brand Safety - **Violence**."]
pub enum IntegralAdScienceExcludedViolenceRiskEnum {
    #[serde(rename = "VIOLENCE_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any violence options."]
    ViolenceUnspecified,
    #[serde(rename = "VIOLENCE_HR")]
    #[doc = "Violence - Exclude High Risk."]
    ViolenceHr,
    #[serde(rename = "VIOLENCE_HMR")]
    #[doc = "Violence - Exclude High and Moderate Risk."]
    ViolenceHmr,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "True advertising quality (applicable to Display line items only)."]
pub enum IntegralAdScienceTraqScoreOptionEnum {
    #[serde(rename = "TRAQ_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any true advertising quality scores."]
    TraqUnspecified,
    #[serde(rename = "TRAQ_250")]
    #[doc = "TRAQ score 250-1000."]
    Traq250,
    #[serde(rename = "TRAQ_500")]
    #[doc = "TRAQ score 500-1000."]
    Traq500,
    #[serde(rename = "TRAQ_600")]
    #[doc = "TRAQ score 600-1000."]
    Traq600,
    #[serde(rename = "TRAQ_700")]
    #[doc = "TRAQ score 700-1000."]
    Traq700,
    #[serde(rename = "TRAQ_750")]
    #[doc = "TRAQ score 750-1000."]
    Traq750,
    #[serde(rename = "TRAQ_875")]
    #[doc = "TRAQ score 875-1000."]
    Traq875,
    #[serde(rename = "TRAQ_1000")]
    #[doc = "TRAQ score 1000."]
    Traq1000,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Video Viewability Section (applicable to video line items only)."]
pub enum IntegralAdScienceVideoViewabilityEnum {
    #[serde(rename = "VIDEO_VIEWABILITY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and it doesn't specify any video viewability options."]
    VideoViewabilityUnspecified,
    #[serde(rename = "VIDEO_VIEWABILITY_40")]
    #[doc = "40%+ in view (IAB video viewability standard)."]
    VideoViewability40,
    #[serde(rename = "VIDEO_VIEWABILITY_50")]
    #[doc = "50%+ in view (IAB video viewability standard)."]
    VideoViewability50,
    #[serde(rename = "VIDEO_VIEWABILITY_60")]
    #[doc = "60%+ in view (IAB video viewability standard)."]
    VideoViewability60,
    #[serde(rename = "VIDEO_VIEWABILITY_70")]
    #[doc = "70%+ in view (IAB video viewability standard)."]
    VideoViewability70,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Integration details of an entry."]
pub struct IntegrationDetails {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional details of the entry in string format. Must be UTF-8 encoded with a length of no more than 1000 characters."]
    pub details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "integrationCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An external identifier to be associated with the entry. The integration code will show up together with the entry in many places in the system, for example, reporting. Must be UTF-8 encoded with a length of no more than 500 characters."]
    pub integration_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An inventory source."]
pub struct InventorySource {
    #[serde(rename = "commitment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the inventory source has a guaranteed or non-guaranteed delivery."]
    pub commitment: ::std::option::Option<InventorySourceCommitmentEnum>,
    #[serde(rename = "creativeConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creative requirements of the inventory source. Not applicable for auction packages."]
    pub creative_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeConfig>>>,
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID in the exchange space that uniquely identifies the inventory source. Must be unique across buyers within each exchange but not necessarily unique across exchanges."]
    pub deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delivery method of the inventory source. * For non-guaranteed inventory sources, the only acceptable value is `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`. * For guaranteed inventory sources, acceptable values are `INVENTORY_SOURCE_DELIVERY_METHOD_TAG` and `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`."]
    pub delivery_method: ::std::option::Option<InventorySourceDeliveryMethodEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the inventory source. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exchange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exchange to which the inventory source belongs."]
    pub exchange: ::std::option::Option<InventorySourceExchangeEnum>,
    #[serde(rename = "inventorySourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the inventory source. Assigned by the system."]
    pub inventory_source_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventorySourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Denotes the type of the inventory source."]
    pub inventory_source_type: ::std::option::Option<InventorySourceInventorySourceTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the inventory source."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The publisher/seller name of the inventory source."]
    pub publisher_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rateDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The rate details of the inventory source."]
    pub rate_details: ::std::option::Option<::std::boxed::Box<RateDetails>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status settings of the inventory source."]
    pub status: ::std::option::Option<::std::boxed::Box<InventorySourceStatus>>,
    #[serde(rename = "timeRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time range when this inventory source starts and stops serving."]
    pub time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the inventory source was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the inventory source has a guaranteed or non-guaranteed delivery."]
pub enum InventorySourceCommitmentEnum {
    #[serde(rename = "INVENTORY_SOURCE_COMMITMENT_UNSPECIFIED")]
    #[doc = "The commitment is not specified or is unknown in this version."]
    InventorySourceCommitmentUnspecified,
    #[serde(rename = "INVENTORY_SOURCE_COMMITMENT_GUARANTEED")]
    #[doc = "The commitment is guaranteed delivery."]
    InventorySourceCommitmentGuaranteed,
    #[serde(rename = "INVENTORY_SOURCE_COMMITMENT_NON_GUARANTEED")]
    #[doc = "The commitment is non-guaranteed delivery."]
    InventorySourceCommitmentNonGuaranteed,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The delivery method of the inventory source. * For non-guaranteed inventory sources, the only acceptable value is `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`. * For guaranteed inventory sources, acceptable values are `INVENTORY_SOURCE_DELIVERY_METHOD_TAG` and `INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC`."]
pub enum InventorySourceDeliveryMethodEnum {
    #[serde(rename = "INVENTORY_SOURCE_DELIVERY_METHOD_UNSPECIFIED")]
    #[doc = "The delivery method is not specified or is unknown in this version."]
    InventorySourceDeliveryMethodUnspecified,
    #[serde(rename = "INVENTORY_SOURCE_DELIVERY_METHOD_PROGRAMMATIC")]
    #[doc = "The delivery method is programmatic."]
    InventorySourceDeliveryMethodProgrammatic,
    #[serde(rename = "INVENTORY_SOURCE_DELIVERY_METHOD_TAG")]
    #[doc = "The delivery method is tag."]
    InventorySourceDeliveryMethodTag,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The exchange to which the inventory source belongs."]
pub enum InventorySourceExchangeEnum {
    #[serde(rename = "EXCHANGE_UNSPECIFIED")]
    #[doc = "Exchange is not specified or is unknown in this version."]
    ExchangeUnspecified,
    #[serde(rename = "EXCHANGE_GOOGLE_AD_MANAGER")]
    #[doc = "Google Ad Manager."]
    ExchangeGoogleAdManager,
    #[serde(rename = "EXCHANGE_APPNEXUS")]
    #[doc = "AppNexus."]
    ExchangeAppnexus,
    #[serde(rename = "EXCHANGE_BRIGHTROLL")]
    #[doc = "BrightRoll Exchange for Video from Yahoo!."]
    ExchangeBrightroll,
    #[serde(rename = "EXCHANGE_ADFORM")]
    #[doc = "Adform."]
    ExchangeAdform,
    #[serde(rename = "EXCHANGE_ADMETA")]
    #[doc = "Admeta."]
    ExchangeAdmeta,
    #[serde(rename = "EXCHANGE_ADMIXER")]
    #[doc = "Admixer."]
    ExchangeAdmixer,
    #[serde(rename = "EXCHANGE_ADSMOGO")]
    #[doc = "AdsMogo."]
    ExchangeAdsmogo,
    #[serde(rename = "EXCHANGE_ADSWIZZ")]
    #[doc = "AdsWizz."]
    ExchangeAdswizz,
    #[serde(rename = "EXCHANGE_BIDSWITCH")]
    #[doc = "BidSwitch."]
    ExchangeBidswitch,
    #[serde(rename = "EXCHANGE_BRIGHTROLL_DISPLAY")]
    #[doc = "BrightRoll Exchange for Display from Yahoo!."]
    ExchangeBrightrollDisplay,
    #[serde(rename = "EXCHANGE_CADREON")]
    #[doc = "Cadreon."]
    ExchangeCadreon,
    #[serde(rename = "EXCHANGE_DAILYMOTION")]
    #[doc = "Dailymotion."]
    ExchangeDailymotion,
    #[serde(rename = "EXCHANGE_FIVE")]
    #[doc = "Five."]
    ExchangeFive,
    #[serde(rename = "EXCHANGE_FLUCT")]
    #[doc = "Fluct."]
    ExchangeFluct,
    #[serde(rename = "EXCHANGE_FREEWHEEL")]
    #[doc = "FreeWheel SSP."]
    ExchangeFreewheel,
    #[serde(rename = "EXCHANGE_GENIEE")]
    #[doc = "Geniee."]
    ExchangeGeniee,
    #[serde(rename = "EXCHANGE_GUMGUM")]
    #[doc = "GumGum."]
    ExchangeGumgum,
    #[serde(rename = "EXCHANGE_IMOBILE")]
    #[doc = "i-mobile."]
    ExchangeImobile,
    #[serde(rename = "EXCHANGE_IBILLBOARD")]
    #[doc = "iBILLBOARD."]
    ExchangeIbillboard,
    #[serde(rename = "EXCHANGE_IMPROVE_DIGITAL")]
    #[doc = "Improve Digital."]
    ExchangeImproveDigital,
    #[serde(rename = "EXCHANGE_INDEX")]
    #[doc = "Index Exchange."]
    ExchangeIndex,
    #[serde(rename = "EXCHANGE_KARGO")]
    #[doc = "Kargo."]
    ExchangeKargo,
    #[serde(rename = "EXCHANGE_MICROAD")]
    #[doc = "MicroAd."]
    ExchangeMicroad,
    #[serde(rename = "EXCHANGE_MOPUB")]
    #[doc = "MoPub."]
    ExchangeMopub,
    #[serde(rename = "EXCHANGE_NEND")]
    #[doc = "Nend."]
    ExchangeNend,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_DISPLAY")]
    #[doc = "ONE by AOL: Display Market Place."]
    ExchangeOneByAolDisplay,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_MOBILE")]
    #[doc = "ONE by AOL: Mobile."]
    ExchangeOneByAolMobile,
    #[serde(rename = "EXCHANGE_ONE_BY_AOL_VIDEO")]
    #[doc = "ONE by AOL: Video."]
    ExchangeOneByAolVideo,
    #[serde(rename = "EXCHANGE_OOYALA")]
    #[doc = "Ooyala."]
    ExchangeOoyala,
    #[serde(rename = "EXCHANGE_OPENX")]
    #[doc = "OpenX."]
    ExchangeOpenx,
    #[serde(rename = "EXCHANGE_PERMODO")]
    #[doc = "Permodo."]
    ExchangePermodo,
    #[serde(rename = "EXCHANGE_PLATFORMONE")]
    #[doc = "Platform One."]
    ExchangePlatformone,
    #[serde(rename = "EXCHANGE_PLATFORMID")]
    #[doc = "PlatformId."]
    ExchangePlatformid,
    #[serde(rename = "EXCHANGE_PUBMATIC")]
    #[doc = "PubMatic."]
    ExchangePubmatic,
    #[serde(rename = "EXCHANGE_PULSEPOINT")]
    #[doc = "PulsePoint."]
    ExchangePulsepoint,
    #[serde(rename = "EXCHANGE_REVENUEMAX")]
    #[doc = "RevenueMax."]
    ExchangeRevenuemax,
    #[serde(rename = "EXCHANGE_RUBICON")]
    #[doc = "Rubicon."]
    ExchangeRubicon,
    #[serde(rename = "EXCHANGE_SMARTCLIP")]
    #[doc = "SmartClip."]
    ExchangeSmartclip,
    #[serde(rename = "EXCHANGE_SMARTRTB")]
    #[doc = "SmartRTB+."]
    ExchangeSmartrtb,
    #[serde(rename = "EXCHANGE_SMARTSTREAMTV")]
    #[doc = "SmartstreamTv."]
    ExchangeSmartstreamtv,
    #[serde(rename = "EXCHANGE_SOVRN")]
    #[doc = "Sovrn."]
    ExchangeSovrn,
    #[serde(rename = "EXCHANGE_SPOTXCHANGE")]
    #[doc = "SpotXchange."]
    ExchangeSpotxchange,
    #[serde(rename = "EXCHANGE_STROER")]
    #[doc = "Ströer SSP."]
    ExchangeStroer,
    #[serde(rename = "EXCHANGE_TEADSTV")]
    #[doc = "TeadsTv."]
    ExchangeTeadstv,
    #[serde(rename = "EXCHANGE_TELARIA")]
    #[doc = "Telaria."]
    ExchangeTelaria,
    #[serde(rename = "EXCHANGE_TVN")]
    #[doc = "TVN."]
    ExchangeTvn,
    #[serde(rename = "EXCHANGE_UNITED")]
    #[doc = "United."]
    ExchangeUnited,
    #[serde(rename = "EXCHANGE_YIELDLAB")]
    #[doc = "Yieldlab."]
    ExchangeYieldlab,
    #[serde(rename = "EXCHANGE_YIELDMO")]
    #[doc = "Yieldmo."]
    ExchangeYieldmo,
    #[serde(rename = "EXCHANGE_UNRULYX")]
    #[doc = "UnrulyX."]
    ExchangeUnrulyx,
    #[serde(rename = "EXCHANGE_OPEN8")]
    #[doc = "Open8."]
    ExchangeOpen8,
    #[serde(rename = "EXCHANGE_TRITON")]
    #[doc = "Triton."]
    ExchangeTriton,
    #[serde(rename = "EXCHANGE_TRIPLELIFT")]
    #[doc = "TripleLift."]
    ExchangeTriplelift,
    #[serde(rename = "EXCHANGE_TABOOLA")]
    #[doc = "Taboola."]
    ExchangeTaboola,
    #[serde(rename = "EXCHANGE_INMOBI")]
    #[doc = "InMobi."]
    ExchangeInmobi,
    #[serde(rename = "EXCHANGE_SMAATO")]
    #[doc = "Smaato."]
    ExchangeSmaato,
    #[serde(rename = "EXCHANGE_AJA")]
    #[doc = "Aja."]
    ExchangeAja,
    #[serde(rename = "EXCHANGE_SUPERSHIP")]
    #[doc = "Supership."]
    ExchangeSupership,
    #[serde(rename = "EXCHANGE_NEXSTAR_DIGITAL")]
    #[doc = "Nexstar Digital."]
    ExchangeNexstarDigital,
    #[serde(rename = "EXCHANGE_WAZE")]
    #[doc = "Waze."]
    ExchangeWaze,
    #[serde(rename = "EXCHANGE_SOUNDCAST")]
    #[doc = "SoundCast."]
    ExchangeSoundcast,
    #[serde(rename = "EXCHANGE_SHARETHROUGH")]
    #[doc = "Sharethrough."]
    ExchangeSharethrough,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Denotes the type of the inventory source."]
pub enum InventorySourceInventorySourceTypeEnum {
    #[serde(rename = "INVENTORY_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "The inventory source type is not specified or is unknown in this version."]
    InventorySourceTypeUnspecified,
    #[serde(rename = "INVENTORY_SOURCE_TYPE_PRIVATE")]
    #[doc = "Private inventory source."]
    InventorySourceTypePrivate,
    #[serde(rename = "INVENTORY_SOURCE_TYPE_AUCTION_PACKAGE")]
    #[doc = "Auction package."]
    InventorySourceTypeAuctionPackage,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for inventory source. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE`."]
pub struct InventorySourceAssignedTargetingOptionDetails {
    #[serde(rename = "inventorySourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the inventory source. Should refer to the inventory_source_id field of an InventorySource resource."]
    pub inventory_source_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for display creatives."]
pub struct InventorySourceDisplayCreativeConfig {
    #[serde(rename = "creativeSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size requirements for display creatives that can be assigned to the inventory source."]
    pub creative_size: ::std::option::Option<::std::boxed::Box<Dimensions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filtering option for filtering on Inventory Source entities."]
pub struct InventorySourceFilter {
    #[serde(rename = "inventorySourceIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory Sources to download by ID. All IDs must belong to the same Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Leave empty to download all Inventory Sources for the selected Advertiser or Partner."]
    pub inventory_source_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of targetable inventory sources."]
pub struct InventorySourceGroup {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the inventory source group. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventorySourceGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the inventory source group. Assigned by the system."]
    pub inventory_source_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the inventory source group."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for inventory source group. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_INVENTORY_SOURCE_GROUP`."]
pub struct InventorySourceGroupAssignedTargetingOptionDetails {
    #[serde(rename = "inventorySourceGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the inventory source group. Should refer to the inventory_source_group_id field of an InventorySourceGroup resource."]
    pub inventory_source_group_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status related settings of the inventory source."]
pub struct InventorySourceStatus {
    #[serde(rename = "configStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The configuration status of the inventory source. Only applicable for guaranteed inventory sources. Acceptable values are `INVENTORY_SOURCE_CONFIG_STATUS_PENDING` and `INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED`. An inventory source must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve."]
    pub config_status: ::std::option::Option<InventorySourceStatusConfigStatusEnum>,
    #[serde(rename = "entityPauseReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-provided reason for pausing this inventory source. Must not exceed 100 characters. Only applicable when entity_status is set to `ENTITY_STATUS_PAUSED`."]
    pub entity_pause_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the inventory source is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`."]
    pub entity_status: ::std::option::Option<InventorySourceStatusEntityStatusEnum>,
    #[serde(rename = "sellerPauseReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The seller-provided reason for pausing this inventory source. Only applicable for inventory sources synced directly from the publishers and when seller_status is set to `ENTITY_STATUS_PAUSED`."]
    pub seller_pause_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellerStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The status set by the seller for the inventory source. Only applicable for inventory sources synced directly from the publishers. Acceptable values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_PAUSED`."]
    pub seller_status: ::std::option::Option<InventorySourceStatusSellerStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The configuration status of the inventory source. Only applicable for guaranteed inventory sources. Acceptable values are `INVENTORY_SOURCE_CONFIG_STATUS_PENDING` and `INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED`. An inventory source must be configured (fill in the required fields, choose creatives, and select a default campaign) before it can serve."]
pub enum InventorySourceStatusConfigStatusEnum {
    #[serde(rename = "INVENTORY_SOURCE_CONFIG_STATUS_UNSPECIFIED")]
    #[doc = "The approval status is not specified or is unknown in this version."]
    InventorySourceConfigStatusUnspecified,
    #[serde(rename = "INVENTORY_SOURCE_CONFIG_STATUS_PENDING")]
    #[doc = "The beginning state of a guaranteed inventory source. The inventory source in this state needs to be configured."]
    InventorySourceConfigStatusPending,
    #[serde(rename = "INVENTORY_SOURCE_CONFIG_STATUS_COMPLETED")]
    #[doc = "The state after the buyer configures a guaranteed inventory source."]
    InventorySourceConfigStatusCompleted,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether or not the inventory source is servable. Acceptable values are `ENTITY_STATUS_ACTIVE`, `ENTITY_STATUS_ARCHIVED`, and `ENTITY_STATUS_PAUSED`. Default value is `ENTITY_STATUS_ACTIVE`."]
pub enum InventorySourceStatusEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The status set by the seller for the inventory source. Only applicable for inventory sources synced directly from the publishers. Acceptable values are `ENTITY_STATUS_ACTIVE` and `ENTITY_STATUS_PAUSED`."]
pub enum InventorySourceStatusSellerStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for video creatives."]
pub struct InventorySourceVideoCreativeConfig {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration requirements for the video creatives that can be assigned to the inventory source."]
    pub duration: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned keyword targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_KEYWORD`."]
pub struct KeywordAssignedTargetingOptionDetails {
    #[serde(rename = "keyword")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The keyword, for example `car insurance`. Positive keyword cannot be offensive word. Must be UTF-8 encoded with a maximum size of 255 bytes. Maximum number of characters is 80. Maximum number of words is 10."]
    pub keyword: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned language targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_LANGUAGE`."]
pub struct LanguageAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the language (e.g., \"French\")."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted. All assigned language targeting options on the same line item must have the same value for this field."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_LANGUAGE`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable language. This will be populated in the language_details field when targeting_type is `TARGETING_TYPE_LANGUAGE`."]
pub struct LanguageTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the language (e.g., \"French\")."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single line item."]
pub struct LineItem {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser the line item belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bidStrategy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The bidding strategy of the line item."]
    pub bid_strategy: ::std::option::Option<::std::boxed::Box<BiddingStrategy>>,
    #[serde(rename = "budget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The budget allocation setting of the line item."]
    pub budget: ::std::option::Option<::std::boxed::Box<LineItemBudget>>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the campaign that the line item belongs to."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conversionCounting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversion tracking setting of the line item."]
    pub conversion_counting: ::std::option::Option<::std::boxed::Box<ConversionCountingConfig>>,
    #[serde(rename = "creativeIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the creatives associated with the line item."]
    pub creative_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the line item. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Controls whether or not the line item can spend its budget and bid on inventory. * For CreateLineItem method, only `ENTITY_STATUS_DRAFT` is allowed. To activate a line item, use UpdateLineItem method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * A line item cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * If the line item's parent insertion order is not active, the line item can't spend its budget even if its own status is `ENTITY_STATUS_ACTIVE`."]
    pub entity_status: ::std::option::Option<LineItemEntityStatusEnum>,
    #[serde(rename = "flight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The start and end time of the line item's flight."]
    pub flight: ::std::option::Option<::std::boxed::Box<LineItemFlight>>,
    #[serde(rename = "frequencyCap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The frequency capping setting of the line item."]
    pub frequency_cap: ::std::option::Option<::std::boxed::Box<FrequencyCap>>,
    #[serde(rename = "insertionOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The unique ID of the insertion order that the line item belongs to."]
    pub insertion_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "integrationDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integration details of the line item."]
    pub integration_details: ::std::option::Option<::std::boxed::Box<IntegrationDetails>>,
    #[serde(rename = "inventorySourceIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the private inventory sources assigned to the line item."]
    pub inventory_source_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the line item. Assigned by the system."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItemType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The type of the line item."]
    pub line_item_type: ::std::option::Option<LineItemLineItemTypeEnum>,
    #[serde(rename = "mobileApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mobile app promoted by the line item. This is applicable only when line_item_type is either `LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL` or `LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL`."]
    pub mobile_app: ::std::option::Option<::std::boxed::Box<MobileApp>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the line item."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pacing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The budget spending speed setting of the line item."]
    pub pacing: ::std::option::Option<::std::boxed::Box<Pacing>>,
    #[serde(rename = "partnerCosts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partner costs associated with the line item. If absent or empty in CreateLineItem method, the newly created line item will inherit partner costs from its parent insertion order."]
    pub partner_costs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartnerCost>>>,
    #[serde(rename = "partnerRevenueModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The partner revenue model setting of the line item."]
    pub partner_revenue_model: ::std::option::Option<::std::boxed::Box<PartnerRevenueModel>>,
    #[serde(rename = "targetingExpansion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [targeting expansion](https://support.google.com/displayvideo/answer/10191558) settings of the line item. This config is only applicable when eligible audience list targeting is assigned to the line item."]
    pub targeting_expansion: ::std::option::Option<::std::boxed::Box<TargetingExpansionConfig>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the line item was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warningMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The warning messages generated by the line item. These warnings do not block saving the line item, but some may block the line item from running."]
    pub warning_messages: ::std::option::Option<::std::vec::Vec<LineItemWarningMessagesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Controls whether or not the line item can spend its budget and bid on inventory. * For CreateLineItem method, only `ENTITY_STATUS_DRAFT` is allowed. To activate a line item, use UpdateLineItem method and update the status to `ENTITY_STATUS_ACTIVE` after creation. * A line item cannot be changed back to `ENTITY_STATUS_DRAFT` status from any other status. * If the line item's parent insertion order is not active, the line item can't spend its budget even if its own status is `ENTITY_STATUS_ACTIVE`."]
pub enum LineItemEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The type of the line item."]
pub enum LineItemLineItemTypeEnum {
    #[serde(rename = "LINE_ITEM_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    LineItemTypeUnspecified,
    #[serde(rename = "LINE_ITEM_TYPE_DISPLAY_DEFAULT")]
    #[doc = "Image, HTML5, native, or rich media ads."]
    LineItemTypeDisplayDefault,
    #[serde(rename = "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INSTALL")]
    #[doc = "Display ads that drive installs of an app."]
    LineItemTypeDisplayMobileAppInstall,
    #[serde(rename = "LINE_ITEM_TYPE_VIDEO_DEFAULT")]
    #[doc = "Video ads sold on a CPM basis for a variety of environments."]
    LineItemTypeVideoDefault,
    #[serde(rename = "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INSTALL")]
    #[doc = "Video ads that drive installs of an app."]
    LineItemTypeVideoMobileAppInstall,
    #[serde(rename = "LINE_ITEM_TYPE_DISPLAY_MOBILE_APP_INVENTORY")]
    #[doc = "Display ads served on mobile app inventory."]
    LineItemTypeDisplayMobileAppInventory,
    #[serde(rename = "LINE_ITEM_TYPE_VIDEO_MOBILE_APP_INVENTORY")]
    #[doc = "Video ads served on mobile app inventory."]
    LineItemTypeVideoMobileAppInventory,
    #[serde(rename = "LINE_ITEM_TYPE_AUDIO_DEFAULT")]
    #[doc = "RTB Audio ads sold for a variety of environments."]
    LineItemTypeAudioDefault,
    #[serde(rename = "LINE_ITEM_TYPE_VIDEO_OVER_THE_TOP")]
    #[doc = "Over-the-top ads present in OTT insertion orders. This type is only applicable to line items with an insertion order of insertion_order_type `OVER_THE_TOP`."]
    LineItemTypeVideoOverTheTop,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum LineItemWarningMessagesEnum {
    #[serde(rename = "LINE_ITEM_WARNING_MESSAGE_UNSPECIFIED")]
    #[doc = "Not specified or is unknown."]
    LineItemWarningMessageUnspecified,
    #[serde(rename = "INVALID_FLIGHT_DATES")]
    #[doc = "This line item has invalid flight dates. The line item will not run."]
    InvalidFlightDates,
    #[serde(rename = "EXPIRED")]
    #[doc = "This line item's end date is in the past."]
    Expired,
    #[serde(rename = "PENDING_FLIGHT")]
    #[doc = "This line item will begin running in the future."]
    PendingFlight,
    #[serde(rename = "ALL_PARTNER_ENABLED_EXCHANGES_NEGATIVELY_TARGETED")]
    #[doc = "All partner enabled exchanges are negatively targeted. The line item will not run."]
    AllPartnerEnabledExchangesNegativelyTargeted,
    #[serde(rename = "INVALID_INVENTORY_SOURCE")]
    #[doc = "No active inventory sources are being targeted. The line item will not run."]
    InvalidInventorySource,
    #[serde(rename = "APP_INVENTORY_INVALID_SITE_TARGETING")]
    #[doc = "This line item's Apps & URLs targeting doesn't include any mobile apps. This line item's type requires you to include mobile apps in your channel, sitelist, or apps targeting. The line item will not run."]
    AppInventoryInvalidSiteTargeting,
    #[serde(rename = "APP_INVENTORY_INVALID_AUDIENCE_LISTS")]
    #[doc = "This line item isn't targeting any mobile users. This line item's type requires you to target a user list with mobile users. The line item will not run."]
    AppInventoryInvalidAudienceLists,
    #[serde(rename = "NO_VALID_CREATIVE")]
    #[doc = "This line item does not contain any valid creative. The line item will not run."]
    NoValidCreative,
    #[serde(rename = "PARENT_INSERTION_ORDER_PAUSED")]
    #[doc = "The insertion order of this line item is paused. The line item will not run."]
    ParentInsertionOrderPaused,
    #[serde(rename = "PARENT_INSERTION_ORDER_EXPIRED")]
    #[doc = "The insertion order of this line item has its end date set in the past. The line item will not run."]
    ParentInsertionOrderExpired,
    #[serde(rename = "NO_POSITIVE_AUDIENCE_LIST_TARGETED")]
    #[doc = "This line item does not target any audience lists, which may result in spending your budget too quickly."]
    NoPositiveAudienceListTargeted,
    #[serde(rename = "APP_INSTALL_NO_CONVERSION_PIXEL")]
    #[doc = "This app install line item does not have any conversion pixel set up."]
    AppInstallNoConversionPixel,
    #[serde(rename = "TARGETING_REVOKED_OR_CLOSED_USER_LIST")]
    #[doc = "This line item targets one or more user lists that are no longer available. In the future, this will prevent the line item from serving, so consider removing these lists from your targeting."]
    TargetingRevokedOrClosedUserList,
    #[serde(rename = "APP_INSTALL_NO_OPTIMAL_BIDDING_STRATEGY")]
    #[doc = "This app install line item does not have an optimal bidding strategy."]
    AppInstallNoOptimalBiddingStrategy,
    #[serde(rename = "CREATIVE_SIZE_NOT_IN_USE_FOR_TARGETED_DEALS")]
    #[doc = "Deals targeted by this line item accept creative sizes which are not in use. This may limit the line item's delivery or performance."]
    CreativeSizeNotInUseForTargetedDeals,
    #[serde(rename = "NO_CREATIVE_FOR_TARGETED_DEALS")]
    #[doc = "This line item does not contain any creative for the targeted deals."]
    NoCreativeForTargetedDeals,
    #[serde(rename = "TARGETING_DEPRECATED_GEO_TARGET")]
    #[doc = "This line item targets a geo target that is deprecated."]
    TargetingDeprecatedGeoTarget,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how budget is allocated."]
pub struct LineItemBudget {
    #[serde(rename = "budgetAllocationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the budget allocation. `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC` is only applicable when automatic budget allocation is enabled for the parent insertion order."]
    pub budget_allocation_type: ::std::option::Option<LineItemBudgetBudgetAllocationTypeEnum>,
    #[serde(rename = "budgetUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The budget unit specifies whether the budget is currency based or impression based. This value is inherited from the parent insertion order."]
    pub budget_unit: ::std::option::Option<LineItemBudgetBudgetUnitEnum>,
    #[serde(rename = "maxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum budget amount the line item will spend. Must be greater than 0. When budget_allocation_type is: * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC`, this field is immutable and is set by the system. * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED`, if budget_unit is: - `BUDGET_UNIT_CURRENCY`, this field represents maximum budget amount to spend, in micros of the advertiser's currency. For example, 1500000 represents 1.5 standard units of the currency. - `BUDGET_UNIT_IMPRESSIONS`, this field represents the maximum number of impressions to serve. * `LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED`, this field is not applicable and will be ignored by the system."]
    pub max_amount: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the budget allocation. `LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC` is only applicable when automatic budget allocation is enabled for the parent insertion order."]
pub enum LineItemBudgetBudgetAllocationTypeEnum {
    #[serde(rename = "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    LineItemBudgetAllocationTypeUnspecified,
    #[serde(rename = "LINE_ITEM_BUDGET_ALLOCATION_TYPE_AUTOMATIC")]
    #[doc = "Automatic budget allocation is enabled for the line item."]
    LineItemBudgetAllocationTypeAutomatic,
    #[serde(rename = "LINE_ITEM_BUDGET_ALLOCATION_TYPE_FIXED")]
    #[doc = "A fixed max budget amount is allocated for the line item."]
    LineItemBudgetAllocationTypeFixed,
    #[serde(rename = "LINE_ITEM_BUDGET_ALLOCATION_TYPE_UNLIMITED")]
    #[doc = "No budget limit is applied to the line item."]
    LineItemBudgetAllocationTypeUnlimited,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The budget unit specifies whether the budget is currency based or impression based. This value is inherited from the parent insertion order."]
pub enum LineItemBudgetBudgetUnitEnum {
    #[serde(rename = "BUDGET_UNIT_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    BudgetUnitUnspecified,
    #[serde(rename = "BUDGET_UNIT_CURRENCY")]
    #[doc = "Budgeting in currency amounts."]
    BudgetUnitCurrency,
    #[serde(rename = "BUDGET_UNIT_IMPRESSIONS")]
    #[doc = "Budgeting in impression amounts."]
    BudgetUnitImpressions,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the active duration of a line item."]
pub struct LineItemFlight {
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The flight start and end dates of the line item. They are resolved relative to the parent advertiser's time zone. * Required when flight_date_type is `LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM`. Output only otherwise. * When creating a new flight, both `start_date` and `end_date` must be in the future. * An existing flight with a `start_date` in the past has a mutable `end_date` but an immutable `start_date`. * `end_date` must be the `start_date` or later, both before the year 2037."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "flightDateType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the line item's flight dates."]
    pub flight_date_type: ::std::option::Option<LineItemFlightFlightDateTypeEnum>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the manual trigger associated with the line item. * Required when flight_date_type is `LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER`. Must not be set otherwise. * When set, the line item's flight dates are inherited from its parent insertion order. * Active line items will spend when the selected trigger is activated within the parent insertion order's flight dates."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the line item's flight dates."]
pub enum LineItemFlightFlightDateTypeEnum {
    #[serde(rename = "LINE_ITEM_FLIGHT_DATE_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    LineItemFlightDateTypeUnspecified,
    #[serde(rename = "LINE_ITEM_FLIGHT_DATE_TYPE_INHERITED")]
    #[doc = "The line item's flight dates are inherited from its parent insertion order."]
    LineItemFlightDateTypeInherited,
    #[serde(rename = "LINE_ITEM_FLIGHT_DATE_TYPE_CUSTOM")]
    #[doc = "The line item uses its own custom flight dates."]
    LineItemFlightDateTypeCustom,
    #[serde(rename = "LINE_ITEM_FLIGHT_DATE_TYPE_TRIGGER")]
    #[doc = "The line item uses a trigger."]
    LineItemFlightDateTypeTrigger,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListAdvertiserAssignedTargetingOptions."]
pub struct ListAdvertiserAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListAdvertiserAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListAdvertisersResponse {
    #[serde(rename = "advertisers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of advertisers. This list will be absent if empty."]
    pub advertisers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Advertiser>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAdvertisers` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for AssignedInventorySourceService.ListAssignedInventorySources."]
pub struct ListAssignedInventorySourcesResponse {
    #[serde(rename = "assignedInventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned inventory sources. This list will be absent if empty."]
    pub assigned_inventory_sources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedInventorySource>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAssignedInventorySources` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for AssignedLocationService.ListAssignedLocations."]
pub struct ListAssignedLocationsResponse {
    #[serde(rename = "assignedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned locations. This list will be absent if empty."]
    pub assigned_locations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedLocation>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListAssignedLocations` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCampaignsResponse {
    #[serde(rename = "campaigns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of campaigns. This list will be absent if empty."]
    pub campaigns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Campaign>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCampaigns` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListChannelsResponse {
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of channels. This list will be absent if empty."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListChannels` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCombinedAudiencesResponse {
    #[serde(rename = "combinedAudiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of combined audiences. This list will be absent if empty."]
    pub combined_audiences:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CombinedAudience>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCombinedAudiences` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCreativesResponse {
    #[serde(rename = "creatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of creatives. This list will be absent if empty."]
    pub creatives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Creative>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCreativesRequest` method to retrieve the next page of results. If this field is null, it means this is the last page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCustomBiddingAlgorithmsResponse {
    #[serde(rename = "customBiddingAlgorithms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom bidding algorithms. This list will be absent if empty."]
    pub custom_bidding_algorithms:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomBiddingAlgorithm>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCustomBiddingAlgorithmsRequest` method to retrieve the next page of results. If this field is null, it means this is the last page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCustomListsResponse {
    #[serde(rename = "customLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom lists. This list will be absent if empty."]
    pub custom_lists: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomList>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListCustomLists` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFirstAndThirdPartyAudiencesResponse {
    #[serde(rename = "firstAndThirdPartyAudiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of first and third party audiences. Audience size properties will not be included. This list will be absent if empty."]
    pub first_and_third_party_audiences:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirstAndThirdPartyAudience>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListFirstAndThirdPartyAudiences` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListGoogleAudiencesResponse {
    #[serde(rename = "googleAudiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Google audiences. This list will be absent if empty."]
    pub google_audiences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAudience>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListGoogleAudiences` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListInsertionOrderAssignedTargetingOptions."]
pub struct ListInsertionOrderAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListInsertionOrderAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListInsertionOrdersResponse {
    #[serde(rename = "insertionOrders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of insertion orders. This list will be absent if empty."]
    pub insertion_orders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InsertionOrder>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInsertionOrders` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for InventorySourceGroupService.ListInventorySourceGroups."]
pub struct ListInventorySourceGroupsResponse {
    #[serde(rename = "inventorySourceGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of inventory source groups. This list will be absent if empty."]
    pub inventory_source_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InventorySourceGroup>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInventorySourceGroups` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListInventorySourcesResponse {
    #[serde(rename = "inventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of inventory sources. This list will be absent if empty."]
    pub inventory_sources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InventorySource>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListInventorySources` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListLineItemAssignedTargetingOptions."]
pub struct ListLineItemAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListLineItemAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListLineItemsResponse {
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of line items. This list will be absent if empty."]
    pub line_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LineItem>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListLineItems` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListLocationListsResponse {
    #[serde(rename = "locationLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of location lists. This list will be absent if empty."]
    pub location_lists: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocationList>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListLocationLists` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListManualTriggersResponse {
    #[serde(rename = "manualTriggers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of manual triggers. This list will be absent if empty."]
    pub manual_triggers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManualTrigger>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListManualTriggers` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for NegativeKeywordListService.ListNegativeKeywordLists."]
pub struct ListNegativeKeywordListsResponse {
    #[serde(rename = "negativeKeywordLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of negative keyword lists. This list will be absent if empty."]
    pub negative_keyword_lists:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NegativeKeywordList>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListNegativeKeywordLists` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for NegativeKeywordService.ListNegativeKeywords."]
pub struct ListNegativeKeywordsResponse {
    #[serde(rename = "negativeKeywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of negative keywords. This list will be absent if empty."]
    pub negative_keywords:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NegativeKeyword>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListNegativeKeywords` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListPartnerAssignedTargetingOptionsResponse {
    #[serde(rename = "assignedTargetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of assigned targeting options. This list will be absent if empty."]
    pub assigned_targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedTargetingOption>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying the next page of results. This value should be specified as the pageToken in a subsequent ListPartnerAssignedTargetingOptionsRequest to fetch the next page of results. This token will be absent if there are no more assigned_targeting_options to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListPartnersResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListPartners` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of partners. This list will be absent if empty."]
    pub partners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Partner>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SiteService.ListSites."]
pub struct ListSitesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListSites` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of sites. This list will be absent if empty."]
    pub sites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Site>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListTargetingOptions."]
pub struct ListTargetingOptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListTargetingOptions` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of targeting options. This list will be absent if empty."]
    pub targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingOption>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListUsersResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `ListUsers` method to retrieve the next page of results. This token will be absent if there are no more results to return."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of users. This list will be absent if empty."]
    pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of locations used for targeting."]
pub struct LocationList {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The unique ID of the advertiser the location list belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the location list. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the location list. Assigned by the system."]
    pub location_list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The type of location. All locations in the list will share this type."]
    pub location_type: ::std::option::Option<LocationListLocationTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the location list."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The type of location. All locations in the list will share this type."]
pub enum LocationListLocationTypeEnum {
    #[serde(rename = "TARGETING_LOCATION_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown."]
    TargetingLocationTypeUnspecified,
    #[serde(rename = "TARGETING_LOCATION_TYPE_PROXIMITY")]
    #[doc = "The type for proximity geo location."]
    TargetingLocationTypeProximity,
    #[serde(rename = "TARGETING_LOCATION_TYPE_REGIONAL")]
    #[doc = "The type for regional geo location."]
    TargetingLocationTypeRegional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies how many days into the past to look when determining whether to record a conversion."]
pub struct LookbackWindow {
    #[serde(rename = "clickDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window, in days, from the last time a given user clicked on one of your ads."]
    pub click_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "impressionDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window, in days, from the last time a given user viewed one of your ads."]
    pub impression_days: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single manual trigger in Display & Video 360."]
pub struct ManualTrigger {
    #[serde(rename = "activationDurationMinutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The maximum duration of each activation in minutes. Must be between 1 and 360 inclusive. After this duration, the trigger will be automatically deactivated."]
    pub activation_duration_minutes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the manual trigger. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestActivationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp of the trigger's latest activation."]
    pub latest_activation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the manual trigger."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of the manual trigger. Will be set to the `INACTIVE` state upon creation."]
    pub state: ::std::option::Option<ManualTriggerStateEnum>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the manual trigger."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of the manual trigger. Will be set to the `INACTIVE` state upon creation."]
pub enum ManualTriggerStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Default value when state is not specified or is unknown in this version."]
    StateUnspecified,
    #[serde(rename = "INACTIVE")]
    #[doc = "The trigger is currently inactive and ready to be activated."]
    Inactive,
    #[serde(rename = "ACTIVE")]
    #[doc = "The trigger is currently active (activated)."]
    Active,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A strategy that automatically adjusts the bid to optimize a specified performance goal while spending the full budget."]
pub struct MaximizeSpendBidStrategy {
    #[serde(rename = "customBiddingAlgorithmId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Custom Bidding Algorithm used by this strategy. Only applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`."]
    pub custom_bidding_algorithm_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxAverageCpmBidAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum average CPM that may be bid, in micros of the advertiser's currency. Must be greater than or equal to a billable unit of the given currency. For example, 1500000 represents 1.5 standard units of the currency."]
    pub max_average_cpm_bid_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the performance goal that the bidding strategy tries to minimize while spending the full budget. `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` is not supported for this strategy."]
    pub performance_goal_type:
        ::std::option::Option<MaximizeSpendBidStrategyPerformanceGoalTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the performance goal that the bidding strategy tries to minimize while spending the full budget. `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` is not supported for this strategy."]
pub enum MaximizeSpendBidStrategyPerformanceGoalTypeEnum {
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    BiddingStrategyPerformanceGoalTypeUnspecified,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA")]
    #[doc = "Cost per action."]
    BiddingStrategyPerformanceGoalTypeCpa,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC")]
    #[doc = "Cost per click."]
    BiddingStrategyPerformanceGoalTypeCpc,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM")]
    #[doc = "Viewable CPM."]
    BiddingStrategyPerformanceGoalTypeViewableCpm,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO")]
    #[doc = "Custom bidding algorithm."]
    BiddingStrategyPerformanceGoalTypeCustomAlgo,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA")]
    #[doc = "Completed inview and audible views."]
    BiddingStrategyPerformanceGoalTypeCiva,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN")]
    #[doc = "Inview time over 10 secs views."]
    BiddingStrategyPerformanceGoalTypeIvoTen,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED")]
    #[doc = "Viewable impressions."]
    BiddingStrategyPerformanceGoalTypeAvViewed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Measurement settings of a partner."]
pub struct MeasurementConfig {
    #[serde(rename = "dv360ToCmCostReportingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to report DV360 cost to CM360."]
    pub dv360_to_cm_cost_reporting_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dv360ToCmDataSharingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to include DV360 data in CM360 data transfer reports."]
    pub dv360_to_cm_data_sharing_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mobile app promoted by a mobile app install line item."]
pub struct MobileApp {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the app provided by the platform store. Android apps are identified by the bundle ID used by Android's Play store, such as `com.google.android.gm`. iOS apps are identified by a nine-digit app ID used by Apple's App store, such as `422689480`."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The app name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The app platform."]
    pub platform: ::std::option::Option<MobileAppPlatformEnum>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The app publisher."]
    pub publisher: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The app platform."]
pub enum MobileAppPlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Platform is not specified."]
    PlatformUnspecified,
    #[serde(rename = "IOS")]
    #[doc = "iOS platform."]
    Ios,
    #[serde(rename = "ANDROID")]
    #[doc = "Android platform."]
    Android,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an amount of money with its currency type."]
pub struct Money {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The three-letter currency code defined in ISO 4217."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
    pub units: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A negatively targeted keyword that belongs to a negative keyword list."]
pub struct NegativeKeyword {
    #[serde(rename = "keywordValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The negatively targeted keyword, for example `car insurance`. Must be UTF-8 encoded with a maximum size of 255 bytes. Maximum number of characters is 80. Maximum number of words is 10. Valid characters are restricted to ASCII characters only. The only URL-escaping permitted is for representing whitespace between words. Leading or trailing whitespace is ignored."]
    pub keyword_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the negative keyword."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of negative keywords used for targeting."]
pub struct NegativeKeywordList {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the advertiser the negative keyword list belongs to."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the negative keyword list. Must be UTF-8 encoded with a maximum size of 255 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the negative keyword list."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negativeKeywordListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the negative keyword list. Assigned by the system."]
    pub negative_keyword_list_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for negative keyword list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_NEGATIVE_KEYWORD_LIST`."]
pub struct NegativeKeywordListAssignedTargetingOptionDetails {
    #[serde(rename = "negativeKeywordListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the negative keyword list. Should refer to the negative_keyword_list_id field of a NegativeKeywordList resource."]
    pub negative_keyword_list_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OBA Icon for a Creative"]
pub struct ObaIcon {
    #[serde(rename = "clickTrackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The click tracking URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com"]
    pub click_tracking_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimensions of the OBA icon."]
    pub dimensions: ::std::option::Option<::std::boxed::Box<Dimensions>>,
    #[serde(rename = "landingPageUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The landing page URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com"]
    pub landing_page_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The position of the OBA icon on the creative."]
    pub position: ::std::option::Option<ObaIconPositionEnum>,
    #[serde(rename = "program")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The program of the OBA icon. For example: “AdChoices”."]
    pub program: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceMimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the OBA icon resource."]
    pub resource_mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the OBA icon resource."]
    pub resource_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewTrackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The view tracking URL of the OBA icon. Only URLs of the following domains are allowed: * https://info.evidon.com * https://l.betrad.com"]
    pub view_tracking_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The position of the OBA icon on the creative."]
pub enum ObaIconPositionEnum {
    #[serde(rename = "OBA_ICON_POSITION_UNSPECIFIED")]
    #[doc = "The OBA icon position is not specified."]
    ObaIconPositionUnspecified,
    #[serde(rename = "OBA_ICON_POSITION_UPPER_RIGHT")]
    #[doc = "At the upper right side of the creative."]
    ObaIconPositionUpperRight,
    #[serde(rename = "OBA_ICON_POSITION_UPPER_LEFT")]
    #[doc = "At the upper left side of the creative."]
    ObaIconPositionUpperLeft,
    #[serde(rename = "OBA_ICON_POSITION_LOWER_RIGHT")]
    #[doc = "At the lower right side of the creative."]
    ObaIconPositionLowerRight,
    #[serde(rename = "OBA_ICON_POSITION_LOWER_LEFT")]
    #[doc = "At the lower left side of the creative."]
    ObaIconPositionLowerLeft,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "On screen position targeting option details. This will be populated in the on_screen_position_details field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`."]
pub struct OnScreenPositionAssignedTargetingOptionDetails {
    #[serde(rename = "adType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`."]
    pub ad_type: ::std::option::Option<OnScreenPositionAssignedTargetingOptionDetailsAdTypeEnum>,
    #[serde(rename = "onScreenPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The on screen position."]
    pub on_screen_position:
        ::std::option::Option<OnScreenPositionAssignedTargetingOptionDetailsOnScreenPositionEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The ad type to target. Only applicable to insertion order targeting and new line items supporting the specified ad type will inherit this targeting option by default. Possible values are: * `AD_TYPE_DISPLAY`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_DISPLAY_DEFAULT`. * `AD_TYPE_VIDEO`, the setting will be inherited by new line item when line_item_type is `LINE_ITEM_TYPE_VIDEO_DEFAULT`."]
pub enum OnScreenPositionAssignedTargetingOptionDetailsAdTypeEnum {
    #[serde(rename = "AD_TYPE_UNSPECIFIED")]
    #[doc = "Ad type is not specified or is unknown in this version."]
    AdTypeUnspecified,
    #[serde(rename = "AD_TYPE_DISPLAY")]
    #[doc = "Display creatives, e.g. image and HTML5."]
    AdTypeDisplay,
    #[serde(rename = "AD_TYPE_VIDEO")]
    #[doc = "Video creatives, e.g. video ads that play during streaming content in video players."]
    AdTypeVideo,
    #[serde(rename = "AD_TYPE_AUDIO")]
    #[doc = "Audio creatives, e.g. audio ads that play during audio content."]
    AdTypeAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The on screen position."]
pub enum OnScreenPositionAssignedTargetingOptionDetailsOnScreenPositionEnum {
    #[serde(rename = "ON_SCREEN_POSITION_UNSPECIFIED")]
    #[doc = "On screen position is not specified in this version. This enum is a place holder for a default value and does not represent a real on screen position."]
    OnScreenPositionUnspecified,
    #[serde(rename = "ON_SCREEN_POSITION_UNKNOWN")]
    #[doc = "The ad position is unknown on the screen."]
    OnScreenPositionUnknown,
    #[serde(rename = "ON_SCREEN_POSITION_ABOVE_THE_FOLD")]
    #[doc = "The ad is located above the fold."]
    OnScreenPositionAboveTheFold,
    #[serde(rename = "ON_SCREEN_POSITION_BELOW_THE_FOLD")]
    #[doc = "The ad is located below the fold."]
    OnScreenPositionBelowTheFold,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable on screen position, which could be used by display and video ads. This will be populated in the on_screen_position_details field when targeting_type is `TARGETING_TYPE_ON_SCREEN_POSITION`."]
pub struct OnScreenPositionTargetingOptionDetails {
    #[serde(rename = "onScreenPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The on screen position."]
    pub on_screen_position:
        ::std::option::Option<OnScreenPositionTargetingOptionDetailsOnScreenPositionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The on screen position."]
pub enum OnScreenPositionTargetingOptionDetailsOnScreenPositionEnum {
    #[serde(rename = "ON_SCREEN_POSITION_UNSPECIFIED")]
    #[doc = "On screen position is not specified in this version. This enum is a place holder for a default value and does not represent a real on screen position."]
    OnScreenPositionUnspecified,
    #[serde(rename = "ON_SCREEN_POSITION_UNKNOWN")]
    #[doc = "The ad position is unknown on the screen."]
    OnScreenPositionUnknown,
    #[serde(rename = "ON_SCREEN_POSITION_ABOVE_THE_FOLD")]
    #[doc = "The ad is located above the fold."]
    OnScreenPositionAboveTheFold,
    #[serde(rename = "ON_SCREEN_POSITION_BELOW_THE_FOLD")]
    #[doc = "The ad is located below the fold."]
    OnScreenPositionBelowTheFold,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned operating system targeting option details. This will be populated in the operating_system_details field when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`."]
pub struct OperatingSystemAssignedTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the operating system."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting option ID populated in targeting_option_id field when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable operating system. This will be populated in the operating_system_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_OPERATING_SYSTEM`."]
pub struct OperatingSystemTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the operating system."]
    pub display_name: ::std::option::Option<::std::string::String>,
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
#[doc = "Settings that control the rate at which a budget is spent."]
pub struct Pacing {
    #[serde(rename = "dailyMaxImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of impressions to serve every day. Applicable when the budget is impression based. Must be greater than 0."]
    pub daily_max_impressions: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dailyMaxMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum currency amount to spend every day in micros of advertiser's currency. Applicable when the budget is currency based. Must be greater than 0. For example, for 1.5 standard unit of the currency, set this field to 1500000. The value assigned will be rounded to whole billable units for the relevant currency by the following rules: any positive value less than a single billable unit will be rounded up to one billable unit and any value larger than a single billable unit will be rounded down to the nearest billable value. For example, if the currency's billable unit is 0.01, and this field is set to 10257770, it will round down to 10250000, a value of 10.25. If set to 505, it will round up to 10000, a value of 0.01."]
    pub daily_max_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pacingPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The time period in which the pacing budget will be spent. When automatic budget allocation is enabled at the insertion order via auto_budget_allocation, this field is output only and defaults to `PACING_PERIOD_FLIGHT`."]
    pub pacing_period: ::std::option::Option<PacingPacingPeriodEnum>,
    #[serde(rename = "pacingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of pacing that defines how the budget amount will be spent across the pacing_period."]
    pub pacing_type: ::std::option::Option<PacingPacingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The time period in which the pacing budget will be spent. When automatic budget allocation is enabled at the insertion order via auto_budget_allocation, this field is output only and defaults to `PACING_PERIOD_FLIGHT`."]
pub enum PacingPacingPeriodEnum {
    #[serde(rename = "PACING_PERIOD_UNSPECIFIED")]
    #[doc = "Period value is not specified or is unknown in this version."]
    PacingPeriodUnspecified,
    #[serde(rename = "PACING_PERIOD_DAILY")]
    #[doc = "The pacing setting will be applied on daily basis."]
    PacingPeriodDaily,
    #[serde(rename = "PACING_PERIOD_FLIGHT")]
    #[doc = "The pacing setting will be applied to the whole flight duration."]
    PacingPeriodFlight,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of pacing that defines how the budget amount will be spent across the pacing_period."]
pub enum PacingPacingTypeEnum {
    #[serde(rename = "PACING_TYPE_UNSPECIFIED")]
    #[doc = "Pacing mode value is not specified or is unknown in this version."]
    PacingTypeUnspecified,
    #[serde(rename = "PACING_TYPE_AHEAD")]
    #[doc = "Only applicable to `PACING_PERIOD_FLIGHT` pacing period. Ahead pacing attempts to spend faster than evenly, to make sure the entire budget is spent by the end of the flight."]
    PacingTypeAhead,
    #[serde(rename = "PACING_TYPE_ASAP")]
    #[doc = "Spend all of pacing budget amount as quick as possible."]
    PacingTypeAsap,
    #[serde(rename = "PACING_TYPE_EVEN")]
    #[doc = "Spend a consistent budget amount every period of time."]
    PacingTypeEven,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filtering option that filters on selected file types belonging to a chosen set of filter entities."]
pub struct ParentEntityFilter {
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. File types that will be returned."]
    pub file_type: ::std::option::Option<::std::vec::Vec<ParentEntityFilterFileTypeEnum>>,
    #[serde(rename = "filterIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the specified filter type. This is used to filter entities to fetch. If filter type is not `FILTER_TYPE_NONE`, at least one ID must be specified."]
    pub filter_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "filterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Filter type used to filter fetched entities."]
    pub filter_type: ::std::option::Option<ParentEntityFilterFilterTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ParentEntityFilterFileTypeEnum {
    #[serde(rename = "FILE_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is unspecified or is unknown in this version."]
    FileTypeUnspecified,
    #[serde(rename = "FILE_TYPE_CAMPAIGN")]
    #[doc = "Campaign."]
    FileTypeCampaign,
    #[serde(rename = "FILE_TYPE_MEDIA_PRODUCT")]
    #[doc = "Media Product."]
    FileTypeMediaProduct,
    #[serde(rename = "FILE_TYPE_INSERTION_ORDER")]
    #[doc = "Insertion Order."]
    FileTypeInsertionOrder,
    #[serde(rename = "FILE_TYPE_LINE_ITEM")]
    #[doc = "Line Item."]
    FileTypeLineItem,
    #[serde(rename = "FILE_TYPE_AD_GROUP")]
    #[doc = "YouTube Ad Group."]
    FileTypeAdGroup,
    #[serde(rename = "FILE_TYPE_AD")]
    #[doc = "YouTube Ad."]
    FileTypeAd,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Filter type used to filter fetched entities."]
pub enum ParentEntityFilterFilterTypeEnum {
    #[serde(rename = "FILTER_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is unspecified or is unknown in this version."]
    FilterTypeUnspecified,
    #[serde(rename = "FILTER_TYPE_NONE")]
    #[doc = "If selected, no filter will be applied to the download. Can only be used if an Advertiser is specified in CreateSdfDownloadTaskRequest."]
    FilterTypeNone,
    #[serde(rename = "FILTER_TYPE_ADVERTISER_ID")]
    #[doc = "Advertiser ID. If selected, all filter IDs must be Advertiser IDs that belong to the Partner specified in CreateSdfDownloadTaskRequest."]
    FilterTypeAdvertiserId,
    #[serde(rename = "FILTER_TYPE_CAMPAIGN_ID")]
    #[doc = "Campaign ID. If selected, all filter IDs must be Campaign IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest."]
    FilterTypeCampaignId,
    #[serde(rename = "FILTER_TYPE_MEDIA_PRODUCT_ID")]
    #[doc = "Media Product ID. If selected, all filter IDs must be Media Product IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_MEDIA_PRODUCT`."]
    FilterTypeMediaProductId,
    #[serde(rename = "FILTER_TYPE_INSERTION_ORDER_ID")]
    #[doc = "Insertion Order ID. If selected, all filter IDs must be Insertion Order IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_INSERTION_ORDER`, `FILE_TYPE_LINE_ITEM`, `FILE_TYPE_AD_GROUP`, and `FILE_TYPE_AD`."]
    FilterTypeInsertionOrderId,
    #[serde(rename = "FILTER_TYPE_LINE_ITEM_ID")]
    #[doc = "Line Item ID. If selected, all filter IDs must be Line Item IDs that belong to the Advertiser or Partner specified in CreateSdfDownloadTaskRequest. Can only be used for downloading `FILE_TYPE_LINE_ITEM`, `FILE_TYPE_AD_GROUP`, and `FILE_TYPE_AD`."]
    FilterTypeLineItemId,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned parental status targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARTGETING_TYPE_PARENTAL_STATUS`."]
pub struct ParentalStatusAssignedTargetingOptionDetails {
    #[serde(rename = "parentalStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The parental status of the audience."]
    pub parental_status:
        ::std::option::Option<ParentalStatusAssignedTargetingOptionDetailsParentalStatusEnum>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_PARENTAL_STATUS`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The parental status of the audience."]
pub enum ParentalStatusAssignedTargetingOptionDetailsParentalStatusEnum {
    #[serde(rename = "PARENTAL_STATUS_UNSPECIFIED")]
    #[doc = "Default value when parental status is not specified in this version. This enum is a place holder for default value and does not represent a real parental status option."]
    ParentalStatusUnspecified,
    #[serde(rename = "PARENTAL_STATUS_PARENT")]
    #[doc = "The audience is a parent."]
    ParentalStatusParent,
    #[serde(rename = "PARENTAL_STATUS_NOT_A_PARENT")]
    #[doc = "The audience is not a parent."]
    ParentalStatusNotAParent,
    #[serde(rename = "PARENTAL_STATUS_UNKNOWN")]
    #[doc = "The parental status of the audience is unknown."]
    ParentalStatusUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable parental status. This will be populated in the parental_status_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_PARENTAL_STATUS`."]
pub struct ParentalStatusTargetingOptionDetails {
    #[serde(rename = "parentalStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The parental status of an audience."]
    pub parental_status:
        ::std::option::Option<ParentalStatusTargetingOptionDetailsParentalStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The parental status of an audience."]
pub enum ParentalStatusTargetingOptionDetailsParentalStatusEnum {
    #[serde(rename = "PARENTAL_STATUS_UNSPECIFIED")]
    #[doc = "Default value when parental status is not specified in this version. This enum is a place holder for default value and does not represent a real parental status option."]
    ParentalStatusUnspecified,
    #[serde(rename = "PARENTAL_STATUS_PARENT")]
    #[doc = "The audience is a parent."]
    ParentalStatusParent,
    #[serde(rename = "PARENTAL_STATUS_NOT_A_PARENT")]
    #[doc = "The audience is not a parent."]
    ParentalStatusNotAParent,
    #[serde(rename = "PARENTAL_STATUS_UNKNOWN")]
    #[doc = "The parental status of the audience is unknown."]
    ParentalStatusUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single partner in Display & Video 360 (DV360)."]
pub struct Partner {
    #[serde(rename = "adServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad server related settings of the partner."]
    pub ad_server_config: ::std::option::Option<::std::boxed::Box<PartnerAdServerConfig>>,
    #[serde(rename = "dataAccessConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings that control how partner data may be accessed."]
    pub data_access_config: ::std::option::Option<::std::boxed::Box<PartnerDataAccessConfig>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the partner. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The status of the partner."]
    pub entity_status: ::std::option::Option<PartnerEntityStatusEnum>,
    #[serde(rename = "exchangeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings that control which exchanges are enabled for the partner."]
    pub exchange_config: ::std::option::Option<::std::boxed::Box<ExchangeConfig>>,
    #[serde(rename = "generalConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General settings of the partner."]
    pub general_config: ::std::option::Option<::std::boxed::Box<PartnerGeneralConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the partner."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the partner. Assigned by the system."]
    pub partner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the partner was last updated. Assigned by the system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The status of the partner."]
pub enum PartnerEntityStatusEnum {
    #[serde(rename = "ENTITY_STATUS_UNSPECIFIED")]
    #[doc = "Default value when status is not specified or is unknown in this version."]
    EntityStatusUnspecified,
    #[serde(rename = "ENTITY_STATUS_ACTIVE")]
    #[doc = "The entity is enabled to bid and spend budget."]
    EntityStatusActive,
    #[serde(rename = "ENTITY_STATUS_ARCHIVED")]
    #[doc = "The entity is archived. Bidding and budget spending are disabled. An entity can be deleted after archived. Deleted entities cannot be retrieved."]
    EntityStatusArchived,
    #[serde(rename = "ENTITY_STATUS_DRAFT")]
    #[doc = "The entity is under draft. Bidding and budget spending are disabled."]
    EntityStatusDraft,
    #[serde(rename = "ENTITY_STATUS_PAUSED")]
    #[doc = "Bidding and budget spending are paused for the entity."]
    EntityStatusPaused,
    #[serde(rename = "ENTITY_STATUS_SCHEDULED_FOR_DELETION")]
    #[doc = "The entity is scheduled for deletion."]
    EntityStatusScheduledForDeletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ad server related settings of a partner."]
pub struct PartnerAdServerConfig {
    #[serde(rename = "measurementConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Measurement settings of a partner."]
    pub measurement_config: ::std::option::Option<::std::boxed::Box<MeasurementConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control a partner cost. A partner cost is any type of expense involved in running a campaign, other than the costs of purchasing impressions (which is called the media cost) and using third-party audience segment data (data fee). Some examples of partner costs include the fees for using DV360, a third-party ad server, or a third-party ad serving verification service."]
pub struct PartnerCost {
    #[serde(rename = "costType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the partner cost."]
    pub cost_type: ::std::option::Option<PartnerCostCostTypeEnum>,
    #[serde(rename = "feeAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CPM fee amount in micros of advertiser's currency. Applicable when the fee_type is `PARTNER_FEE_TYPE_CPM_FEE`. Must be greater than or equal to 0. For example, for 1.5 standard unit of the advertiser's currency, set this field to 1500000."]
    pub fee_amount: ::std::option::Option<::std::string::String>,
    #[serde(rename = "feePercentageMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The media fee percentage in millis (1/1000 of a percent). Applicable when the fee_type is `PARTNER_FEE_TYPE_MEDIA_FEE`. Must be greater than or equal to 0. For example: 100 represents 0.1%."]
    pub fee_percentage_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "feeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fee type for this partner cost."]
    pub fee_type: ::std::option::Option<PartnerCostFeeTypeEnum>,
    #[serde(rename = "invoiceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The invoice type for this partner cost. * Required when cost_type is one of: - `PARTNER_COST_TYPE_ADLOOX` - `PARTNER_COST_TYPE_DOUBLE_VERIFY` - `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE`. * Output only for other types."]
    pub invoice_type: ::std::option::Option<PartnerCostInvoiceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the partner cost."]
pub enum PartnerCostCostTypeEnum {
    #[serde(rename = "PARTNER_COST_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    PartnerCostTypeUnspecified,
    #[serde(rename = "PARTNER_COST_TYPE_ADLOOX")]
    #[doc = "The cost is charged for using Adloox."]
    PartnerCostTypeAdloox,
    #[serde(rename = "PARTNER_COST_TYPE_ADLOOX_PREBID")]
    #[doc = "The cost is charged for using Adloox Pre-Bid."]
    PartnerCostTypeAdlooxPrebid,
    #[serde(rename = "PARTNER_COST_TYPE_ADSAFE")]
    #[doc = "The cost is charged for using AdSafe."]
    PartnerCostTypeAdsafe,
    #[serde(rename = "PARTNER_COST_TYPE_ADXPOSE")]
    #[doc = "The cost is charged for using AdExpose."]
    PartnerCostTypeAdxpose,
    #[serde(rename = "PARTNER_COST_TYPE_AGGREGATE_KNOWLEDGE")]
    #[doc = "The cost is charged for using Aggregate Knowledge."]
    PartnerCostTypeAggregateKnowledge,
    #[serde(rename = "PARTNER_COST_TYPE_AGENCY_TRADING_DESK")]
    #[doc = "The cost is charged for using an Agency Trading Desk."]
    PartnerCostTypeAgencyTradingDesk,
    #[serde(rename = "PARTNER_COST_TYPE_DV360_FEE")]
    #[doc = "The cost is charged for using DV360."]
    PartnerCostTypeDv360Fee,
    #[serde(rename = "PARTNER_COST_TYPE_COMSCORE_VCE")]
    #[doc = "The cost is charged for using comScore vCE."]
    PartnerCostTypeComscoreVce,
    #[serde(rename = "PARTNER_COST_TYPE_DATA_MANAGEMENT_PLATFORM")]
    #[doc = "The cost is charged for using a Data Management Platform."]
    PartnerCostTypeDataManagementPlatform,
    #[serde(rename = "PARTNER_COST_TYPE_DEFAULT")]
    #[doc = "The default cost type."]
    PartnerCostTypeDefault,
    #[serde(rename = "PARTNER_COST_TYPE_DOUBLE_VERIFY")]
    #[doc = "The cost is charged for using DoubleVerify."]
    PartnerCostTypeDoubleVerify,
    #[serde(rename = "PARTNER_COST_TYPE_DOUBLE_VERIFY_PREBID")]
    #[doc = "The cost is charged for using DoubleVerify Pre-Bid."]
    PartnerCostTypeDoubleVerifyPrebid,
    #[serde(rename = "PARTNER_COST_TYPE_EVIDON")]
    #[doc = "The cost is charged for using Evidon."]
    PartnerCostTypeEvidon,
    #[serde(rename = "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO")]
    #[doc = "The cost is charged for using Integral Ad Science Video."]
    PartnerCostTypeIntegralAdScienceVideo,
    #[serde(rename = "PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_PREBID")]
    #[doc = "The cost is charged for using Integral Ad Science Pre-Bid."]
    PartnerCostTypeIntegralAdSciencePrebid,
    #[serde(rename = "PARTNER_COST_TYPE_MEDIA_COST_DATA")]
    #[doc = "The cost is charged for using media cost data."]
    PartnerCostTypeMediaCostData,
    #[serde(rename = "PARTNER_COST_TYPE_MOAT_VIDEO")]
    #[doc = "The cost is charged for using MOAT Video."]
    PartnerCostTypeMoatVideo,
    #[serde(rename = "PARTNER_COST_TYPE_NIELSEN_DAR")]
    #[doc = "The cost is charged for using Nielsen Digital Ad Ratings."]
    PartnerCostTypeNielsenDar,
    #[serde(rename = "PARTNER_COST_TYPE_SHOP_LOCAL")]
    #[doc = "The cost is charged for using ShopLocal."]
    PartnerCostTypeShopLocal,
    #[serde(rename = "PARTNER_COST_TYPE_TERACENT")]
    #[doc = "The cost is charged for using Teracent."]
    PartnerCostTypeTeracent,
    #[serde(rename = "PARTNER_COST_TYPE_THIRD_PARTY_AD_SERVER")]
    #[doc = "The cost is charged for using a third-party ad server."]
    PartnerCostTypeThirdPartyAdServer,
    #[serde(rename = "PARTNER_COST_TYPE_TRUST_METRICS")]
    #[doc = "The cost is charged for using TrustMetrics."]
    PartnerCostTypeTrustMetrics,
    #[serde(rename = "PARTNER_COST_TYPE_VIZU")]
    #[doc = "The cost is charged for using Vizu."]
    PartnerCostTypeVizu,
    #[serde(rename = "PARTNER_COST_TYPE_ADLINGO_FEE")]
    #[doc = "The cost is charged for using AdLingo."]
    PartnerCostTypeAdlingoFee,
    #[serde(rename = "PARTNER_COST_TYPE_CUSTOM_FEE_1")]
    #[doc = "The cost is charged as custom fee 1."]
    PartnerCostTypeCustomFee1,
    #[serde(rename = "PARTNER_COST_TYPE_CUSTOM_FEE_2")]
    #[doc = "The cost is charged as custom fee 2."]
    PartnerCostTypeCustomFee2,
    #[serde(rename = "PARTNER_COST_TYPE_CUSTOM_FEE_3")]
    #[doc = "The cost is charged as custom fee 3."]
    PartnerCostTypeCustomFee3,
    #[serde(rename = "PARTNER_COST_TYPE_CUSTOM_FEE_4")]
    #[doc = "The cost is charged as custom fee 4."]
    PartnerCostTypeCustomFee4,
    #[serde(rename = "PARTNER_COST_TYPE_CUSTOM_FEE_5")]
    #[doc = "The cost is charged as custom fee 5."]
    PartnerCostTypeCustomFee5,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The fee type for this partner cost."]
pub enum PartnerCostFeeTypeEnum {
    #[serde(rename = "PARTNER_COST_FEE_TYPE_UNSPECIFIED")]
    #[doc = "Value is not specified or is unknown in this version."]
    PartnerCostFeeTypeUnspecified,
    #[serde(rename = "PARTNER_COST_FEE_TYPE_CPM_FEE")]
    #[doc = "The partner cost is a fixed CPM fee. Not applicable when the partner cost cost_type is one of: * `PARTNER_COST_TYPE_MEDIA_COST_DATA` * `PARTNER_COST_TYPE_DV360_FEE`."]
    PartnerCostFeeTypeCpmFee,
    #[serde(rename = "PARTNER_COST_FEE_TYPE_MEDIA_FEE")]
    #[doc = "The partner cost is a percentage surcharge based on the media cost. Not applicable when the partner cost_type is one of: * `PARTNER_COST_TYPE_SHOP_LOCAL` * `PARTNER_COST_TYPE_TRUST_METRICS` * `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE_VIDEO` * `PARTNER_COST_TYPE_MOAT_VIDEO`."]
    PartnerCostFeeTypeMediaFee,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The invoice type for this partner cost. * Required when cost_type is one of: - `PARTNER_COST_TYPE_ADLOOX` - `PARTNER_COST_TYPE_DOUBLE_VERIFY` - `PARTNER_COST_TYPE_INTEGRAL_AD_SCIENCE`. * Output only for other types."]
pub enum PartnerCostInvoiceTypeEnum {
    #[serde(rename = "PARTNER_COST_INVOICE_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    PartnerCostInvoiceTypeUnspecified,
    #[serde(rename = "PARTNER_COST_INVOICE_TYPE_DV360")]
    #[doc = "Partner cost is billed through DV360."]
    PartnerCostInvoiceTypeDv360,
    #[serde(rename = "PARTNER_COST_INVOICE_TYPE_PARTNER")]
    #[doc = "Partner cost is billed by the partner."]
    PartnerCostInvoiceTypePartner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how partner related data may be accessed."]
pub struct PartnerDataAccessConfig {
    #[serde(rename = "sdfConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structured Data Files (SDF) settings for the partner. The SDF configuration for the partner."]
    pub sdf_config: ::std::option::Option<::std::boxed::Box<SdfConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "General settings of a partner."]
pub struct PartnerGeneralConfig {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Partner's currency in ISO 4217 format."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The standard TZ database name of the partner's time zone. For example, `America/New_York`. See more at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones"]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control how partner revenue is calculated."]
pub struct PartnerRevenueModel {
    #[serde(rename = "markupAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The markup amount of the partner revenue model. Must be greater than or equal to 0. * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM`, this field represents the CPM markup in micros of advertiser's currency. For example, 1500000 represents 1.5 standard units of the currency. * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP`, this field represents the media cost percent markup in millis. For example, 100 represents 0.1% (decimal 0.001). * When the markup_type is set to be `PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP`, this field represents the total media cost percent markup in millis. For example, 100 represents 0.1% (decimal 0.001)."]
    pub markup_amount: ::std::option::Option<::std::string::String>,
    #[serde(rename = "markupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The markup type of the partner revenue model."]
    pub markup_type: ::std::option::Option<PartnerRevenueModelMarkupTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The markup type of the partner revenue model."]
pub enum PartnerRevenueModelMarkupTypeEnum {
    #[serde(rename = "PARTNER_REVENUE_MODEL_MARKUP_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    PartnerRevenueModelMarkupTypeUnspecified,
    #[serde(rename = "PARTNER_REVENUE_MODEL_MARKUP_TYPE_CPM")]
    #[doc = "Calculate the partner revenue based on a fixed CPM."]
    PartnerRevenueModelMarkupTypeCpm,
    #[serde(rename = "PARTNER_REVENUE_MODEL_MARKUP_TYPE_MEDIA_COST_MARKUP")]
    #[doc = "Calculate the partner revenue based on a percentage surcharge of its media cost."]
    PartnerRevenueModelMarkupTypeMediaCostMarkup,
    #[serde(rename = "PARTNER_REVENUE_MODEL_MARKUP_TYPE_TOTAL_MEDIA_COST_MARKUP")]
    #[doc = "Calculate the partner revenue based on a percentage surcharge of its total media cost, which includes all partner costs and data costs."]
    PartnerRevenueModelMarkupTypeTotalMediaCostMarkup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the performance goal of a campaign or insertion order."]
pub struct PerformanceGoal {
    #[serde(rename = "performanceGoalAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The goal amount, in micros of the advertiser's currency. Applicable when performance_goal_type is one of: * `PERFORMANCE_GOAL_TYPE_CPM` * `PERFORMANCE_GOAL_TYPE_CPC` * `PERFORMANCE_GOAL_TYPE_CPA` * `PERFORMANCE_GOAL_TYPE_CPIAVC` * `PERFORMANCE_GOAL_TYPE_VCPM` For example 1500000 represents 1.5 standard units of the currency."]
    pub performance_goal_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalPercentageMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The decimal representation of the goal percentage in micros. Applicable when performance_goal_type is one of: * `PERFORMANCE_GOAL_TYPE_CTR` * `PERFORMANCE_GOAL_TYPE_VIEWABILITY` * `PERFORMANCE_GOAL_TYPE_CLICK_CVR` * `PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR` * `PERFORMANCE_GOAL_TYPE_VTR` * `PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE` * `PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE` For example, 70000 represents 7% (decimal 0.07)."]
    pub performance_goal_percentage_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A key performance indicator (KPI) string, which can be empty. Must be UTF-8 encoded with a length of no more than 100 characters. Applicable when performance_goal_type is set to `PERFORMANCE_GOAL_TYPE_OTHER`."]
    pub performance_goal_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the performance goal."]
    pub performance_goal_type: ::std::option::Option<PerformanceGoalPerformanceGoalTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the performance goal."]
pub enum PerformanceGoalPerformanceGoalTypeEnum {
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Performance goal type is not specified or is unknown in this version."]
    PerformanceGoalTypeUnspecified,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CPM")]
    #[doc = "The performance goal is set in CPM (cost per mille)."]
    PerformanceGoalTypeCpm,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CPC")]
    #[doc = "The performance goal is set in CPC (cost per click)."]
    PerformanceGoalTypeCpc,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CPA")]
    #[doc = "The performance goal is set in CPA (cost per action)."]
    PerformanceGoalTypeCpa,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CTR")]
    #[doc = "The performance goal is set in CTR (click-through rate) percentage."]
    PerformanceGoalTypeCtr,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_VIEWABILITY")]
    #[doc = "The performance goal is set in Viewability percentage."]
    PerformanceGoalTypeViewability,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CPIAVC")]
    #[doc = "The performance goal is set as CPIAVC (cost per impression audible and visible at completion)."]
    PerformanceGoalTypeCpiavc,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CPE")]
    #[doc = "The performance goal is set in CPE (cost per engagement)."]
    PerformanceGoalTypeCpe,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_CLICK_CVR")]
    #[doc = "The performance goal is set in click conversion rate (conversions per click) percentage."]
    PerformanceGoalTypeClickCvr,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_IMPRESSION_CVR")]
    #[doc = "The performance goal is set in impression conversion rate (conversions per impression) percentage."]
    PerformanceGoalTypeImpressionCvr,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_VCPM")]
    #[doc = "The performance goal is set in VCPM (cost per thousand viewable impressions)."]
    PerformanceGoalTypeVcpm,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_VTR")]
    #[doc = "The performance goal is set in YouTube view rate (YouTube views per impression) percentage."]
    PerformanceGoalTypeVtr,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_AUDIO_COMPLETION_RATE")]
    #[doc = "The performance goal is set in audio completion rate (complete audio listens per impression) percentage."]
    PerformanceGoalTypeAudioCompletionRate,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_VIDEO_COMPLETION_RATE")]
    #[doc = "The performance goal is set in video completion rate (complete video views per impression) percentage."]
    PerformanceGoalTypeVideoCompletionRate,
    #[serde(rename = "PERFORMANCE_GOAL_TYPE_OTHER")]
    #[doc = "The performance goal is set to Other."]
    PerformanceGoalTypeOther,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A strategy that automatically adjusts the bid to meet or beat a specified performance goal."]
pub struct PerformanceGoalBidStrategy {
    #[serde(rename = "customBiddingAlgorithmId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Custom Bidding Algorithm used by this strategy. Only applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`."]
    pub custom_bidding_algorithm_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxAverageCpmBidAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum average CPM that may be bid, in micros of the advertiser's currency. Must be greater than or equal to a billable unit of the given currency. Not applicable when performance_goal_type is set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`. For example, 1500000 represents 1.5 standard units of the currency."]
    pub max_average_cpm_bid_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalAmountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The performance goal the bidding strategy will attempt to meet or beat, in micros of the advertiser's currency or in micro of the ROAS (Return On Advertising Spend) value which is also based on advertiser's currency. Must be greater than or equal to a billable unit of the given currency and smaller or equal to upper bounds. Each performance_goal_type has its upper bound: * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA`, upper bound is 10000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC`, upper bound is 1000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`, upper bound is 1000.00 USD. * when performance_goal_type is `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`, upper bound is 1000.00 and lower bound is 0.01. Example: If set to `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM`, the bid price will be based on the probability that each available impression will be viewable. For example, if viewable CPM target is $2 and an impression is 40% likely to be viewable, the bid price will be $0.80 CPM (40% of $2). For example, 1500000 represents 1.5 standard units of the currency or ROAS value."]
    pub performance_goal_amount_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "performanceGoalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the performance goal that the bidding strategy will try to meet or beat. For line item level usage, the value must be one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`."]
    pub performance_goal_type:
        ::std::option::Option<PerformanceGoalBidStrategyPerformanceGoalTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the performance goal that the bidding strategy will try to meet or beat. For line item level usage, the value must be one of: * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM` * `BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO`."]
pub enum PerformanceGoalBidStrategyPerformanceGoalTypeEnum {
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    BiddingStrategyPerformanceGoalTypeUnspecified,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPA")]
    #[doc = "Cost per action."]
    BiddingStrategyPerformanceGoalTypeCpa,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CPC")]
    #[doc = "Cost per click."]
    BiddingStrategyPerformanceGoalTypeCpc,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_VIEWABLE_CPM")]
    #[doc = "Viewable CPM."]
    BiddingStrategyPerformanceGoalTypeViewableCpm,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CUSTOM_ALGO")]
    #[doc = "Custom bidding algorithm."]
    BiddingStrategyPerformanceGoalTypeCustomAlgo,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_CIVA")]
    #[doc = "Completed inview and audible views."]
    BiddingStrategyPerformanceGoalTypeCiva,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_IVO_TEN")]
    #[doc = "Inview time over 10 secs views."]
    BiddingStrategyPerformanceGoalTypeIvoTen,
    #[serde(rename = "BIDDING_STRATEGY_PERFORMANCE_GOAL_TYPE_AV_VIEWED")]
    #[doc = "Viewable impressions."]
    BiddingStrategyPerformanceGoalTypeAvViewed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for proximity location list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_PROXIMITY_LOCATION_LIST`."]
pub struct ProximityLocationListAssignedTargetingOptionDetails {
    #[serde(rename = "proximityLocationListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the proximity location list. Should refer to the location_list_id field of a LocationList resource whose type is `TARGETING_LOCATION_TYPE_PROXIMITY`."]
    pub proximity_location_list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proximityRadiusRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Radius range for proximity location list. This represents the size of the area around a chosen location that will be targeted. `All` proximity location targeting under a single line item must have the same radius range value. Set this value to match any existing targeting. If updated, this field will change the radius range for all proximity targeting under the line item."]
    pub proximity_radius_range: ::std::option::Option<
        ProximityLocationListAssignedTargetingOptionDetailsProximityRadiusRangeEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Radius range for proximity location list. This represents the size of the area around a chosen location that will be targeted. `All` proximity location targeting under a single line item must have the same radius range value. Set this value to match any existing targeting. If updated, this field will change the radius range for all proximity targeting under the line item."]
pub enum ProximityLocationListAssignedTargetingOptionDetailsProximityRadiusRangeEnum {
    #[serde(rename = "PROXIMITY_RADIUS_RANGE_UNSPECIFIED")]
    #[doc = "The targeted radius range is not specified or is unknown. Default value when radius range is not specified in this version. This enum is a placeholder for default value and does not represent a real radius range option."]
    ProximityRadiusRangeUnspecified,
    #[serde(rename = "PROXIMITY_RADIUS_RANGE_SMALL")]
    #[doc = "The targeted radius range is small."]
    ProximityRadiusRangeSmall,
    #[serde(rename = "PROXIMITY_RADIUS_RANGE_MEDIUM")]
    #[doc = "The targeted radius range is medium."]
    ProximityRadiusRangeMedium,
    #[serde(rename = "PROXIMITY_RADIUS_RANGE_LARGE")]
    #[doc = "The targeted radius range is large."]
    ProximityRadiusRangeLarge,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publisher review status for the creative."]
pub struct PublisherReviewStatus {
    #[serde(rename = "publisherName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The publisher reviewing the creative."]
    pub publisher_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the publisher review."]
    pub status: ::std::option::Option<PublisherReviewStatusStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the publisher review."]
pub enum PublisherReviewStatusStatusEnum {
    #[serde(rename = "REVIEW_STATUS_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    ReviewStatusUnspecified,
    #[serde(rename = "REVIEW_STATUS_APPROVED")]
    #[doc = "The creative is approved."]
    ReviewStatusApproved,
    #[serde(rename = "REVIEW_STATUS_REJECTED")]
    #[doc = "The creative is rejected."]
    ReviewStatusRejected,
    #[serde(rename = "REVIEW_STATUS_PENDING")]
    #[doc = "The creative is pending review."]
    ReviewStatusPending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The rate related settings of the inventory source."]
pub struct RateDetails {
    #[serde(rename = "inventorySourceRateType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rate type. Acceptable values are `INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED`, `INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR`, and `INVENTORY_SOURCE_RATE_TYPE_CPD`."]
    pub inventory_source_rate_type: ::std::option::Option<RateDetailsInventorySourceRateTypeEnum>,
    #[serde(rename = "minimumSpend")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The amount that the buyer has committed to spending on the inventory source up front. Only applicable for guaranteed inventory sources."]
    pub minimum_spend: ::std::option::Option<::std::boxed::Box<Money>>,
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rate for the inventory source."]
    pub rate: ::std::option::Option<::std::boxed::Box<Money>>,
    #[serde(rename = "unitsPurchased")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for guaranteed inventory sources. The number of impressions guaranteed by the seller."]
    pub units_purchased: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The rate type. Acceptable values are `INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED`, `INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR`, and `INVENTORY_SOURCE_RATE_TYPE_CPD`."]
pub enum RateDetailsInventorySourceRateTypeEnum {
    #[serde(rename = "INVENTORY_SOURCE_RATE_TYPE_UNSPECIFIED")]
    #[doc = "The rate type is not specified or is unknown in this version."]
    InventorySourceRateTypeUnspecified,
    #[serde(rename = "INVENTORY_SOURCE_RATE_TYPE_CPM_FIXED")]
    #[doc = "The rate type is CPM (Fixed)."]
    InventorySourceRateTypeCpmFixed,
    #[serde(rename = "INVENTORY_SOURCE_RATE_TYPE_CPM_FLOOR")]
    #[doc = "The rate type is CPM (Floor)."]
    InventorySourceRateTypeCpmFloor,
    #[serde(rename = "INVENTORY_SOURCE_RATE_TYPE_CPD")]
    #[doc = "The rate type is Cost per Day."]
    InventorySourceRateTypeCpd,
    #[serde(rename = "INVENTORY_SOURCE_RATE_TYPE_FLAT")]
    #[doc = "The rate type is Flat."]
    InventorySourceRateTypeFlat,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for regional location list. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_REGIONAL_LOCATION_LIST`."]
pub struct RegionalLocationListAssignedTargetingOptionDetails {
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "regionalLocationListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the regional location list. Should refer to the location_list_id field of a LocationList resource whose type is `TARGETING_LOCATION_TYPE_REGIONAL`."]
    pub regional_location_list_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Review statuses for the creative."]
pub struct ReviewStatusInfo {
    #[serde(rename = "approvalStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents the basic approval needed for a creative to begin serving. Summary of creative_and_landing_page_review_status and content_and_policy_review_status."]
    pub approval_status: ::std::option::Option<ReviewStatusInfoApprovalStatusEnum>,
    #[serde(rename = "contentAndPolicyReviewStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content and policy review status for the creative."]
    pub content_and_policy_review_status:
        ::std::option::Option<ReviewStatusInfoContentAndPolicyReviewStatusEnum>,
    #[serde(rename = "creativeAndLandingPageReviewStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative and landing page review status for the creative."]
    pub creative_and_landing_page_review_status:
        ::std::option::Option<ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum>,
    #[serde(rename = "exchangeReviewStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exchange review statuses for the creative."]
    pub exchange_review_statuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExchangeReviewStatus>>>,
    #[serde(rename = "publisherReviewStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher review statuses for the creative."]
    pub publisher_review_statuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublisherReviewStatus>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Represents the basic approval needed for a creative to begin serving. Summary of creative_and_landing_page_review_status and content_and_policy_review_status."]
pub enum ReviewStatusInfoApprovalStatusEnum {
    #[serde(rename = "APPROVAL_STATUS_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    ApprovalStatusUnspecified,
    #[serde(rename = "APPROVAL_STATUS_PENDING_NOT_SERVABLE")]
    #[doc = "The creative is still under review and not servable."]
    ApprovalStatusPendingNotServable,
    #[serde(rename = "APPROVAL_STATUS_PENDING_SERVABLE")]
    #[doc = "The creative has passed creative & landing page review and is servable, but is awaiting additional content & policy review."]
    ApprovalStatusPendingServable,
    #[serde(rename = "APPROVAL_STATUS_APPROVED_SERVABLE")]
    #[doc = "Both creative & landing page review and content & policy review are approved. The creative is servable."]
    ApprovalStatusApprovedServable,
    #[serde(rename = "APPROVAL_STATUS_REJECTED_NOT_SERVABLE")]
    #[doc = "There is an issue with the creative that must be fixed before it can serve."]
    ApprovalStatusRejectedNotServable,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Content and policy review status for the creative."]
pub enum ReviewStatusInfoContentAndPolicyReviewStatusEnum {
    #[serde(rename = "REVIEW_STATUS_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    ReviewStatusUnspecified,
    #[serde(rename = "REVIEW_STATUS_APPROVED")]
    #[doc = "The creative is approved."]
    ReviewStatusApproved,
    #[serde(rename = "REVIEW_STATUS_REJECTED")]
    #[doc = "The creative is rejected."]
    ReviewStatusRejected,
    #[serde(rename = "REVIEW_STATUS_PENDING")]
    #[doc = "The creative is pending review."]
    ReviewStatusPending,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Creative and landing page review status for the creative."]
pub enum ReviewStatusInfoCreativeAndLandingPageReviewStatusEnum {
    #[serde(rename = "REVIEW_STATUS_UNSPECIFIED")]
    #[doc = "Type value is not specified or is unknown in this version."]
    ReviewStatusUnspecified,
    #[serde(rename = "REVIEW_STATUS_APPROVED")]
    #[doc = "The creative is approved."]
    ReviewStatusApproved,
    #[serde(rename = "REVIEW_STATUS_REJECTED")]
    #[doc = "The creative is rejected."]
    ReviewStatusRejected,
    #[serde(rename = "REVIEW_STATUS_PENDING")]
    #[doc = "The creative is pending review."]
    ReviewStatusPending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structured Data File (SDF) related settings."]
pub struct SdfConfig {
    #[serde(rename = "adminEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An administrator email address to which the SDF processing status reports will be sent."]
    pub admin_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of SDF being used."]
    pub version: ::std::option::Option<SdfConfigVersionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The version of SDF being used."]
pub enum SdfConfigVersionEnum {
    #[serde(rename = "SDF_VERSION_UNSPECIFIED")]
    #[doc = "SDF version value is not specified or is unknown in this version."]
    SdfVersionUnspecified,
    #[serde(rename = "SDF_VERSION_3_1")]
    #[doc = "SDF version 3.1"]
    SdfVersion31,
    #[serde(rename = "SDF_VERSION_4")]
    #[doc = "SDF version 4"]
    SdfVersion4,
    #[serde(rename = "SDF_VERSION_4_1")]
    #[doc = "SDF version 4.1"]
    SdfVersion41,
    #[serde(rename = "SDF_VERSION_4_2")]
    #[doc = "SDF version 4.2"]
    SdfVersion42,
    #[serde(rename = "SDF_VERSION_5")]
    #[doc = "SDF version 5."]
    SdfVersion5,
    #[serde(rename = "SDF_VERSION_5_1")]
    #[doc = "SDF version 5.1"]
    SdfVersion51,
    #[serde(rename = "SDF_VERSION_5_2")]
    #[doc = "SDF version 5.2"]
    SdfVersion52,
    #[serde(rename = "SDF_VERSION_5_3")]
    #[doc = "SDF version 5.3"]
    SdfVersion53,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Type for the response returned by [SdfDownloadTaskService.CreateSdfDownloadTask]."]
pub struct SdfDownloadTask {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A resource name to be used in media.download to Download the prepared files. Resource names have the format `download/sdfdownloadtasks/media/{media_id}`. `media_id` will be made available by the long running operation service once the task status is done."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Type for the metadata returned by [SdfDownloadTaskService.CreateSdfDownloadTask]."]
pub struct SdfDownloadTaskMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when execution was completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SDF version used to execute this download task."]
    pub version: ::std::option::Option<SdfDownloadTaskMetadataVersionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The SDF version used to execute this download task."]
pub enum SdfDownloadTaskMetadataVersionEnum {
    #[serde(rename = "SDF_VERSION_UNSPECIFIED")]
    #[doc = "SDF version value is not specified or is unknown in this version."]
    SdfVersionUnspecified,
    #[serde(rename = "SDF_VERSION_3_1")]
    #[doc = "SDF version 3.1"]
    SdfVersion31,
    #[serde(rename = "SDF_VERSION_4")]
    #[doc = "SDF version 4"]
    SdfVersion4,
    #[serde(rename = "SDF_VERSION_4_1")]
    #[doc = "SDF version 4.1"]
    SdfVersion41,
    #[serde(rename = "SDF_VERSION_4_2")]
    #[doc = "SDF version 4.2"]
    SdfVersion42,
    #[serde(rename = "SDF_VERSION_5")]
    #[doc = "SDF version 5."]
    SdfVersion5,
    #[serde(rename = "SDF_VERSION_5_1")]
    #[doc = "SDF version 5.1"]
    SdfVersion51,
    #[serde(rename = "SDF_VERSION_5_2")]
    #[doc = "SDF version 5.2"]
    SdfVersion52,
    #[serde(rename = "SDF_VERSION_5_3")]
    #[doc = "SDF version 5.3"]
    SdfVersion53,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SearchTargetingOptions."]
pub struct SearchTargetingOptionsRequest {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Advertiser this request is being made in the context of."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoRegionSearchTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Search terms for geo region targeting options. Can only be used when targeting_type is `TARGETING_TYPE_GEO_REGION`."]
    pub geo_region_search_terms: ::std::option::Option<::std::boxed::Box<GeoRegionSearchTerms>>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requested page size. Must be between `1` and `100`. If unspecified will default to `100`. Returns error code `INVALID_ARGUMENT` if an invalid value is specified."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of next_page_token returned from the previous call to `SearchTargetingOptions` method. If not specified, the first page of results will be returned."]
    pub page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SearchTargetingOptionsResponse."]
pub struct SearchTargetingOptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. Pass this value in the page_token field in the subsequent call to `SearchTargetingOptions` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of targeting options that match the search criteria. This list will be absent if empty."]
    pub targeting_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingOption>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting details for sensitive category. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`."]
pub struct SensitiveCategoryAssignedTargetingOptionDetails {
    #[serde(rename = "excludedTargetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the sensitive category to be EXCLUDED."]
    pub excluded_targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sensitiveCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An enum for the DV360 Sensitive category content classifier."]
    pub sensitive_category:
        ::std::option::Option<SensitiveCategoryAssignedTargetingOptionDetailsSensitiveCategoryEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. An enum for the DV360 Sensitive category content classifier."]
pub enum SensitiveCategoryAssignedTargetingOptionDetailsSensitiveCategoryEnum {
    #[serde(rename = "SENSITIVE_CATEGORY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and doesn't specify a DV360 sensitive category."]
    SensitiveCategoryUnspecified,
    #[serde(rename = "SENSITIVE_CATEGORY_ADULT")]
    #[doc = "Adult or pornographic text, image, or video content."]
    SensitiveCategoryAdult,
    #[serde(rename = "SENSITIVE_CATEGORY_DEROGATORY")]
    #[doc = "Content that may be construed as biased against individuals, groups, or organizations based on criteria such as race, religion, disability, sex, age, veteran status, sexual orientation, gender identity, or political affiliation. May also indicate discussion of such content, for instance, in an academic or journalistic context."]
    SensitiveCategoryDerogatory,
    #[serde(rename = "SENSITIVE_CATEGORY_DOWNLOADS_SHARING")]
    #[doc = "Content related to audio, video, or software downloads."]
    SensitiveCategoryDownloadsSharing,
    #[serde(rename = "SENSITIVE_CATEGORY_WEAPONS")]
    #[doc = "Contains content related to personal weapons, including knives, guns, small firearms, and ammunition. Selecting either \"weapons\" or \"sensitive social issues\" will result in selecting both."]
    SensitiveCategoryWeapons,
    #[serde(rename = "SENSITIVE_CATEGORY_GAMBLING")]
    #[doc = "Contains content related to betting or wagering in a real-world or online setting."]
    SensitiveCategoryGambling,
    #[serde(rename = "SENSITIVE_CATEGORY_VIOLENCE")]
    #[doc = "Content which may be considered graphically violent, gory, gruesome, or shocking, such as street fighting videos, accident photos, descriptions of torture, etc."]
    SensitiveCategoryViolence,
    #[serde(rename = "SENSITIVE_CATEGORY_SUGGESTIVE")]
    #[doc = "Adult content, as well as suggestive content that's not explicitly pornographic. This category includes all pages categorized as adult."]
    SensitiveCategorySuggestive,
    #[serde(rename = "SENSITIVE_CATEGORY_PROFANITY")]
    #[doc = "Prominent use of words considered indecent, such as curse words and sexual slang. Pages with only very occasional usage, such as news sites that might include such words in a quotation, are not included."]
    SensitiveCategoryProfanity,
    #[serde(rename = "SENSITIVE_CATEGORY_ALCOHOL")]
    #[doc = "Contains content related to alcoholic beverages, alcohol brands, recipes, etc."]
    SensitiveCategoryAlcohol,
    #[serde(rename = "SENSITIVE_CATEGORY_DRUGS")]
    #[doc = "Contains content related to the recreational use of legal or illegal drugs, as well as to drug paraphernalia or cultivation."]
    SensitiveCategoryDrugs,
    #[serde(rename = "SENSITIVE_CATEGORY_TOBACCO")]
    #[doc = "Contains content related to tobacco and tobacco accessories, including lighters, humidors, ashtrays, etc."]
    SensitiveCategoryTobacco,
    #[serde(rename = "SENSITIVE_CATEGORY_POLITICS")]
    #[doc = "Political news and media, including discussions of social, governmental, and public policy."]
    SensitiveCategoryPolitics,
    #[serde(rename = "SENSITIVE_CATEGORY_RELIGION")]
    #[doc = "Content related to religious thought or beliefs."]
    SensitiveCategoryReligion,
    #[serde(rename = "SENSITIVE_CATEGORY_TRAGEDY")]
    #[doc = "Content related to death, disasters, accidents, war, etc."]
    SensitiveCategoryTragedy,
    #[serde(rename = "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS")]
    #[doc = "Content related to motor vehicle, aviation or other transportation accidents."]
    SensitiveCategoryTransportationAccidents,
    #[serde(rename = "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES")]
    #[doc = "Issues that evoke strong, opposing views and spark debate. These include issues that are controversial in most countries and markets (such as abortion), as well as those that are controversial in specific countries and markets (such as immigration reform in the United States)."]
    SensitiveCategorySensitiveSocialIssues,
    #[serde(rename = "SENSITIVE_CATEGORY_SHOCKING")]
    #[doc = "Content which may be considered shocking or disturbing, such as violent news stories, stunts, or toilet humor."]
    SensitiveCategoryShocking,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable sensitive category. This will be populated in the sensitive_category_details field of the TargetingOption when targeting_type is `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`."]
pub struct SensitiveCategoryTargetingOptionDetails {
    #[serde(rename = "sensitiveCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An enum for the DV360 Sensitive category content classifier."]
    pub sensitive_category:
        ::std::option::Option<SensitiveCategoryTargetingOptionDetailsSensitiveCategoryEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. An enum for the DV360 Sensitive category content classifier."]
pub enum SensitiveCategoryTargetingOptionDetailsSensitiveCategoryEnum {
    #[serde(rename = "SENSITIVE_CATEGORY_UNSPECIFIED")]
    #[doc = "This enum is only a placeholder and doesn't specify a DV360 sensitive category."]
    SensitiveCategoryUnspecified,
    #[serde(rename = "SENSITIVE_CATEGORY_ADULT")]
    #[doc = "Adult or pornographic text, image, or video content."]
    SensitiveCategoryAdult,
    #[serde(rename = "SENSITIVE_CATEGORY_DEROGATORY")]
    #[doc = "Content that may be construed as biased against individuals, groups, or organizations based on criteria such as race, religion, disability, sex, age, veteran status, sexual orientation, gender identity, or political affiliation. May also indicate discussion of such content, for instance, in an academic or journalistic context."]
    SensitiveCategoryDerogatory,
    #[serde(rename = "SENSITIVE_CATEGORY_DOWNLOADS_SHARING")]
    #[doc = "Content related to audio, video, or software downloads."]
    SensitiveCategoryDownloadsSharing,
    #[serde(rename = "SENSITIVE_CATEGORY_WEAPONS")]
    #[doc = "Contains content related to personal weapons, including knives, guns, small firearms, and ammunition. Selecting either \"weapons\" or \"sensitive social issues\" will result in selecting both."]
    SensitiveCategoryWeapons,
    #[serde(rename = "SENSITIVE_CATEGORY_GAMBLING")]
    #[doc = "Contains content related to betting or wagering in a real-world or online setting."]
    SensitiveCategoryGambling,
    #[serde(rename = "SENSITIVE_CATEGORY_VIOLENCE")]
    #[doc = "Content which may be considered graphically violent, gory, gruesome, or shocking, such as street fighting videos, accident photos, descriptions of torture, etc."]
    SensitiveCategoryViolence,
    #[serde(rename = "SENSITIVE_CATEGORY_SUGGESTIVE")]
    #[doc = "Adult content, as well as suggestive content that's not explicitly pornographic. This category includes all pages categorized as adult."]
    SensitiveCategorySuggestive,
    #[serde(rename = "SENSITIVE_CATEGORY_PROFANITY")]
    #[doc = "Prominent use of words considered indecent, such as curse words and sexual slang. Pages with only very occasional usage, such as news sites that might include such words in a quotation, are not included."]
    SensitiveCategoryProfanity,
    #[serde(rename = "SENSITIVE_CATEGORY_ALCOHOL")]
    #[doc = "Contains content related to alcoholic beverages, alcohol brands, recipes, etc."]
    SensitiveCategoryAlcohol,
    #[serde(rename = "SENSITIVE_CATEGORY_DRUGS")]
    #[doc = "Contains content related to the recreational use of legal or illegal drugs, as well as to drug paraphernalia or cultivation."]
    SensitiveCategoryDrugs,
    #[serde(rename = "SENSITIVE_CATEGORY_TOBACCO")]
    #[doc = "Contains content related to tobacco and tobacco accessories, including lighters, humidors, ashtrays, etc."]
    SensitiveCategoryTobacco,
    #[serde(rename = "SENSITIVE_CATEGORY_POLITICS")]
    #[doc = "Political news and media, including discussions of social, governmental, and public policy."]
    SensitiveCategoryPolitics,
    #[serde(rename = "SENSITIVE_CATEGORY_RELIGION")]
    #[doc = "Content related to religious thought or beliefs."]
    SensitiveCategoryReligion,
    #[serde(rename = "SENSITIVE_CATEGORY_TRAGEDY")]
    #[doc = "Content related to death, disasters, accidents, war, etc."]
    SensitiveCategoryTragedy,
    #[serde(rename = "SENSITIVE_CATEGORY_TRANSPORTATION_ACCIDENTS")]
    #[doc = "Content related to motor vehicle, aviation or other transportation accidents."]
    SensitiveCategoryTransportationAccidents,
    #[serde(rename = "SENSITIVE_CATEGORY_SENSITIVE_SOCIAL_ISSUES")]
    #[doc = "Issues that evoke strong, opposing views and spark debate. These include issues that are controversial in most countries and markets (such as abortion), as well as those that are controversial in specific countries and markets (such as immigration reform in the United States)."]
    SensitiveCategorySensitiveSocialIssues,
    #[serde(rename = "SENSITIVE_CATEGORY_SHOCKING")]
    #[doc = "Content which may be considered shocking or disturbing, such as violent news stories, stunts, or toilet humor."]
    SensitiveCategoryShocking,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single site. Sites are apps or websites belonging to a channel."]
pub struct Site {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the site."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlOrAppId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URL or app ID of the site. Must be UTF-8 encoded with a maximum length of 240 bytes."]
    pub url_or_app_id: ::std::option::Option<::std::string::String>,
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
#[doc = "Details for assigned sub-exchange targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_SUB_EXCHANGE`."]
pub struct SubExchangeAssignedTargetingOptionDetails {
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_SUB_EXCHANGE`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable sub-exchange. This will be populated in the sub_exchange_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_SUB_EXCHANGE`."]
pub struct SubExchangeTargetingOptionDetails {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the sub-exchange."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the targeting expansion of the line item. Targeting expansion allows the line item to reach a larger audience based on the original audience list and the targeting expansion level."]
pub struct TargetingExpansionConfig {
    #[serde(rename = "excludeFirstPartyAudience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Whether to exclude first party audiences from targeting. Similar audiences of the excluded first party lists will not be excluded. Only applicable when a first-party audience is positively targeted (directly or included in a combined audience), otherwise this selection will be ignored."]
    pub exclude_first_party_audience: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "targetingExpansionLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Magnitude of expansion for applicable targeting under this line item."]
    pub targeting_expansion_level:
        ::std::option::Option<TargetingExpansionConfigTargetingExpansionLevelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Magnitude of expansion for applicable targeting under this line item."]
pub enum TargetingExpansionConfigTargetingExpansionLevelEnum {
    #[serde(rename = "TARGETING_EXPANSION_LEVEL_UNSPECIFIED")]
    #[doc = "Targeting expansion level is not specified or is unknown in this version."]
    TargetingExpansionLevelUnspecified,
    #[serde(rename = "NO_EXPANSION")]
    #[doc = "Targeting expansion off."]
    NoExpansion,
    #[serde(rename = "LEAST_EXPANSION")]
    #[doc = "Conservative targeting expansion, lowest reach."]
    LeastExpansion,
    #[serde(rename = "SOME_EXPANSION")]
    #[doc = "Moderately conservative targeting expansion, lower reach."]
    SomeExpansion,
    #[serde(rename = "BALANCED_EXPANSION")]
    #[doc = "Moderate targeting expansion, medium reach."]
    BalancedExpansion,
    #[serde(rename = "MORE_EXPANSION")]
    #[doc = "Moderately aggressive targeting expansion, higher reach."]
    MoreExpansion,
    #[serde(rename = "MOST_EXPANSION")]
    #[doc = "Aggressive targeting expansion, highest reach."]
    MostExpansion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single targeting option, which is a targetable concept in DV360."]
pub struct TargetingOption {
    #[serde(rename = "ageRangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Age range details."]
    pub age_range_details: ::std::option::Option<::std::boxed::Box<AgeRangeTargetingOptionDetails>>,
    #[serde(rename = "appCategoryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "App category details."]
    pub app_category_details:
        ::std::option::Option<::std::boxed::Box<AppCategoryTargetingOptionDetails>>,
    #[serde(rename = "authorizedSellerStatusDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authorized seller status resource details."]
    pub authorized_seller_status_details:
        ::std::option::Option<::std::boxed::Box<AuthorizedSellerStatusTargetingOptionDetails>>,
    #[serde(rename = "browserDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browser details."]
    pub browser_details: ::std::option::Option<::std::boxed::Box<BrowserTargetingOptionDetails>>,
    #[serde(rename = "carrierAndIspDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Carrier and ISP details."]
    pub carrier_and_isp_details:
        ::std::option::Option<::std::boxed::Box<CarrierAndIspTargetingOptionDetails>>,
    #[serde(rename = "categoryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Category resource details."]
    pub category_details: ::std::option::Option<::std::boxed::Box<CategoryTargetingOptionDetails>>,
    #[serde(rename = "contentInstreamPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content instream position details."]
    pub content_instream_position_details:
        ::std::option::Option<::std::boxed::Box<ContentInstreamPositionTargetingOptionDetails>>,
    #[serde(rename = "contentOutstreamPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content outstream position details."]
    pub content_outstream_position_details:
        ::std::option::Option<::std::boxed::Box<ContentOutstreamPositionTargetingOptionDetails>>,
    #[serde(rename = "deviceMakeModelDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device make and model resource details."]
    pub device_make_model_details:
        ::std::option::Option<::std::boxed::Box<DeviceMakeModelTargetingOptionDetails>>,
    #[serde(rename = "deviceTypeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device type details."]
    pub device_type_details:
        ::std::option::Option<::std::boxed::Box<DeviceTypeTargetingOptionDetails>>,
    #[serde(rename = "digitalContentLabelDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Digital content label details."]
    pub digital_content_label_details:
        ::std::option::Option<::std::boxed::Box<DigitalContentLabelTargetingOptionDetails>>,
    #[serde(rename = "environmentDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment details."]
    pub environment_details:
        ::std::option::Option<::std::boxed::Box<EnvironmentTargetingOptionDetails>>,
    #[serde(rename = "exchangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exchange details."]
    pub exchange_details: ::std::option::Option<::std::boxed::Box<ExchangeTargetingOptionDetails>>,
    #[serde(rename = "genderDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Gender details."]
    pub gender_details: ::std::option::Option<::std::boxed::Box<GenderTargetingOptionDetails>>,
    #[serde(rename = "geoRegionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic region resource details."]
    pub geo_region_details:
        ::std::option::Option<::std::boxed::Box<GeoRegionTargetingOptionDetails>>,
    #[serde(rename = "householdIncomeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Household income details."]
    pub household_income_details:
        ::std::option::Option<::std::boxed::Box<HouseholdIncomeTargetingOptionDetails>>,
    #[serde(rename = "languageDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language resource details."]
    pub language_details: ::std::option::Option<::std::boxed::Box<LanguageTargetingOptionDetails>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name for this targeting option."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onScreenPositionDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "On screen position details."]
    pub on_screen_position_details:
        ::std::option::Option<::std::boxed::Box<OnScreenPositionTargetingOptionDetails>>,
    #[serde(rename = "operatingSystemDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system resources details."]
    pub operating_system_details:
        ::std::option::Option<::std::boxed::Box<OperatingSystemTargetingOptionDetails>>,
    #[serde(rename = "parentalStatusDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parental status details."]
    pub parental_status_details:
        ::std::option::Option<::std::boxed::Box<ParentalStatusTargetingOptionDetails>>,
    #[serde(rename = "sensitiveCategoryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sensitive Category details."]
    pub sensitive_category_details:
        ::std::option::Option<::std::boxed::Box<SensitiveCategoryTargetingOptionDetails>>,
    #[serde(rename = "subExchangeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub-exchange details."]
    pub sub_exchange_details:
        ::std::option::Option<::std::boxed::Box<SubExchangeTargetingOptionDetails>>,
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A unique identifier for this targeting option. The tuple {`targeting_type`, `targeting_option_id`} will be unique."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of this targeting option."]
    pub targeting_type: ::std::option::Option<TargetingOptionTargetingTypeEnum>,
    #[serde(rename = "userRewardedContentDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User rewarded content details."]
    pub user_rewarded_content_details:
        ::std::option::Option<::std::boxed::Box<UserRewardedContentTargetingOptionDetails>>,
    #[serde(rename = "videoPlayerSizeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video player size details."]
    pub video_player_size_details:
        ::std::option::Option<::std::boxed::Box<VideoPlayerSizeTargetingOptionDetails>>,
    #[serde(rename = "viewabilityDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Viewability resource details."]
    pub viewability_details:
        ::std::option::Option<::std::boxed::Box<ViewabilityTargetingOptionDetails>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of this targeting option."]
pub enum TargetingOptionTargetingTypeEnum {
    #[serde(rename = "TARGETING_TYPE_UNSPECIFIED")]
    #[doc = "Default value when type is not specified or is unknown in this version."]
    TargetingTypeUnspecified,
    #[serde(rename = "TARGETING_TYPE_CHANNEL")]
    #[doc = "Target a channel (a custom group of related websites or apps)."]
    TargetingTypeChannel,
    #[serde(rename = "TARGETING_TYPE_APP_CATEGORY")]
    #[doc = "Target an app category (for example, education or puzzle games)."]
    TargetingTypeAppCategory,
    #[serde(rename = "TARGETING_TYPE_APP")]
    #[doc = "Target a specific app (for example, Angry Birds)."]
    TargetingTypeApp,
    #[serde(rename = "TARGETING_TYPE_URL")]
    #[doc = "Target a specific url (for example, quora.com)."]
    TargetingTypeUrl,
    #[serde(rename = "TARGETING_TYPE_DAY_AND_TIME")]
    #[doc = "Target ads during a chosen time period on a specific day."]
    TargetingTypeDayAndTime,
    #[serde(rename = "TARGETING_TYPE_AGE_RANGE")]
    #[doc = "Target ads to a specific age range (for example, 18-24)."]
    TargetingTypeAgeRange,
    #[serde(rename = "TARGETING_TYPE_REGIONAL_LOCATION_LIST")]
    #[doc = "Target ads to the specified regions on a regional location list."]
    TargetingTypeRegionalLocationList,
    #[serde(rename = "TARGETING_TYPE_PROXIMITY_LOCATION_LIST")]
    #[doc = "Target ads to the specified points of interest on a proximity location list."]
    TargetingTypeProximityLocationList,
    #[serde(rename = "TARGETING_TYPE_GENDER")]
    #[doc = "Target ads to a specific gender (for example, female or male)."]
    TargetingTypeGender,
    #[serde(rename = "TARGETING_TYPE_VIDEO_PLAYER_SIZE")]
    #[doc = "Target a specific video player size for video ads."]
    TargetingTypeVideoPlayerSize,
    #[serde(rename = "TARGETING_TYPE_USER_REWARDED_CONTENT")]
    #[doc = "Target user rewarded content for video ads."]
    TargetingTypeUserRewardedContent,
    #[serde(rename = "TARGETING_TYPE_PARENTAL_STATUS")]
    #[doc = "Target ads to a specific parental status (for example, parent or not a parent)."]
    TargetingTypeParentalStatus,
    #[serde(rename = "TARGETING_TYPE_CONTENT_INSTREAM_POSITION")]
    #[doc = "Target video or audio ads in a specific content instream position (for example, pre-roll, mid-roll, or post-roll)."]
    TargetingTypeContentInstreamPosition,
    #[serde(rename = "TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION")]
    #[doc = "Target ads in a specific content outstream position."]
    TargetingTypeContentOutstreamPosition,
    #[serde(rename = "TARGETING_TYPE_DEVICE_TYPE")]
    #[doc = "Target ads to a specific device type (for example, tablet or connected TV)."]
    TargetingTypeDeviceType,
    #[serde(rename = "TARGETING_TYPE_AUDIENCE_GROUP")]
    #[doc = "Target ads to an audience or groups of audiences. Singleton field, at most one can exist on a single Lineitem at a time."]
    TargetingTypeAudienceGroup,
    #[serde(rename = "TARGETING_TYPE_BROWSER")]
    #[doc = "Target ads to specific web browsers (for example, Chrome)."]
    TargetingTypeBrowser,
    #[serde(rename = "TARGETING_TYPE_HOUSEHOLD_INCOME")]
    #[doc = "Target ads to a specific household income range (for example, top 10%)."]
    TargetingTypeHouseholdIncome,
    #[serde(rename = "TARGETING_TYPE_ON_SCREEN_POSITION")]
    #[doc = "Target ads in a specific on screen position."]
    TargetingTypeOnScreenPosition,
    #[serde(rename = "TARGETING_TYPE_THIRD_PARTY_VERIFIER")]
    #[doc = "Filter web sites through third party verification (for example, IAS or DoubleVerify)."]
    TargetingTypeThirdPartyVerifier,
    #[serde(rename = "TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION")]
    #[doc = "Filter web sites by specific digital content label ratings (for example, DL-MA: suitable only for mature audiences)."]
    TargetingTypeDigitalContentLabelExclusion,
    #[serde(rename = "TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION")]
    #[doc = "Filter website content by sensitive categories (for example, adult)."]
    TargetingTypeSensitiveCategoryExclusion,
    #[serde(rename = "TARGETING_TYPE_ENVIRONMENT")]
    #[doc = "Target ads to a specific environment (for example, web or app)."]
    TargetingTypeEnvironment,
    #[serde(rename = "TARGETING_TYPE_CARRIER_AND_ISP")]
    #[doc = "Target ads to a specific network carrier or internet service provider (ISP) (for example, Comcast or Orange)."]
    TargetingTypeCarrierAndIsp,
    #[serde(rename = "TARGETING_TYPE_OPERATING_SYSTEM")]
    #[doc = "Target ads to a specific operating system (for example, macOS)."]
    TargetingTypeOperatingSystem,
    #[serde(rename = "TARGETING_TYPE_DEVICE_MAKE_MODEL")]
    #[doc = "Target ads to a specific device make or model (for example, Roku or Samsung)."]
    TargetingTypeDeviceMakeModel,
    #[serde(rename = "TARGETING_TYPE_KEYWORD")]
    #[doc = "Target ads to a specific keyword (for example, dog or retriever)."]
    TargetingTypeKeyword,
    #[serde(rename = "TARGETING_TYPE_NEGATIVE_KEYWORD_LIST")]
    #[doc = "Target ads to a specific negative keyword list."]
    TargetingTypeNegativeKeywordList,
    #[serde(rename = "TARGETING_TYPE_VIEWABILITY")]
    #[doc = "Target ads to a specific viewability (for example, 80% viewable)."]
    TargetingTypeViewability,
    #[serde(rename = "TARGETING_TYPE_CATEGORY")]
    #[doc = "Target ads to a specific content category (for example, arts & entertainment)."]
    TargetingTypeCategory,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE")]
    #[doc = "Purchase impressions from specific deals and auction packages."]
    TargetingTypeInventorySource,
    #[serde(rename = "TARGETING_TYPE_LANGUAGE")]
    #[doc = "Target ads to a specific language (for example, English or Japanese)."]
    TargetingTypeLanguage,
    #[serde(rename = "TARGETING_TYPE_AUTHORIZED_SELLER_STATUS")]
    #[doc = "Target ads to ads.txt authorized sellers."]
    TargetingTypeAuthorizedSellerStatus,
    #[serde(rename = "TARGETING_TYPE_GEO_REGION")]
    #[doc = "Target ads to a specific regional location (for example, a city or state)."]
    TargetingTypeGeoRegion,
    #[serde(rename = "TARGETING_TYPE_INVENTORY_SOURCE_GROUP")]
    #[doc = "Purchase impressions from a group of deals and auction packages."]
    TargetingTypeInventorySourceGroup,
    #[serde(rename = "TARGETING_TYPE_EXCHANGE")]
    #[doc = "Purchase impressions from specific exchanges."]
    TargetingTypeExchange,
    #[serde(rename = "TARGETING_TYPE_SUB_EXCHANGE")]
    #[doc = "Purchase impressions from specific sub-exchanges."]
    TargetingTypeSubExchange,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for advertisers that use third-party ad servers only."]
pub struct ThirdPartyOnlyConfig {
    #[serde(rename = "pixelOrderIdReportingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not order ID reporting for pixels is enabled. This value cannot be changed once set to `true`."]
    pub pixel_order_id_reporting_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tracking URLs from third parties to track interactions with an audio or a video creative."]
pub struct ThirdPartyUrl {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of interaction needs to be tracked by the tracking URL"]
    pub _type: ::std::option::Option<ThirdPartyUrlTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tracking URL used to track the interaction. Provide a URL with optional path or query string, beginning with `https:`. For example, https://www.example.com/path"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of interaction needs to be tracked by the tracking URL"]
pub enum ThirdPartyUrlTypeEnum {
    #[serde(rename = "THIRD_PARTY_URL_TYPE_UNSPECIFIED")]
    #[doc = "The type of third-party URL is unspecified or is unknown in this version."]
    ThirdPartyUrlTypeUnspecified,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_IMPRESSION")]
    #[doc = "Used to count impressions of the creative after the audio or video buffering is complete."]
    ThirdPartyUrlTypeImpression,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_CLICK_TRACKING")]
    #[doc = "Used to track user clicks on the audio or video."]
    ThirdPartyUrlTypeClickTracking,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_START")]
    #[doc = "Used to track the number of times a user starts the audio or video."]
    ThirdPartyUrlTypeAudioVideoStart,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FIRST_QUARTILE")]
    #[doc = "Used to track the number of times the audio or video plays to 25% of its length."]
    ThirdPartyUrlTypeAudioVideoFirstQuartile,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MIDPOINT")]
    #[doc = "Used to track the number of times the audio or video plays to 50% of its length."]
    ThirdPartyUrlTypeAudioVideoMidpoint,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_THIRD_QUARTILE")]
    #[doc = "Used to track the number of times the audio or video plays to 75% of its length."]
    ThirdPartyUrlTypeAudioVideoThirdQuartile,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_COMPLETE")]
    #[doc = "Used to track the number of times the audio or video plays to the end."]
    ThirdPartyUrlTypeAudioVideoComplete,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_MUTE")]
    #[doc = "Used to track the number of times a user mutes the audio or video."]
    ThirdPartyUrlTypeAudioVideoMute,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PAUSE")]
    #[doc = "Used to track the number of times a user pauses the audio or video."]
    ThirdPartyUrlTypeAudioVideoPause,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_REWIND")]
    #[doc = "Used to track the number of times a user replays the audio or video."]
    ThirdPartyUrlTypeAudioVideoRewind,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_FULLSCREEN")]
    #[doc = "Used to track the number of times a user expands the player to full-screen size."]
    ThirdPartyUrlTypeAudioVideoFullscreen,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_STOP")]
    #[doc = "Used to track the number of times a user stops the audio or video."]
    ThirdPartyUrlTypeAudioVideoStop,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_CUSTOM")]
    #[doc = "Used to track the number of times a user performs a custom click, such as clicking on a video hot spot."]
    ThirdPartyUrlTypeAudioVideoCustom,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_SKIP")]
    #[doc = "Used to track the number of times the audio or video was skipped."]
    ThirdPartyUrlTypeAudioVideoSkip,
    #[serde(rename = "THIRD_PARTY_URL_TYPE_AUDIO_VIDEO_PROGRESS")]
    #[doc = "Used to track the number of times the audio or video plays to an offset determined by the progress_offset."]
    ThirdPartyUrlTypeAudioVideoProgress,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned third party verifier targeting option details. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_THIRD_PARTY_VERIFIER`."]
pub struct ThirdPartyVerifierAssignedTargetingOptionDetails {
    #[serde(rename = "adloox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third party brand verifier -- Adloox."]
    pub adloox: ::std::option::Option<::std::boxed::Box<Adloox>>,
    #[serde(rename = "doubleVerify")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third party brand verifier -- DoubleVerify."]
    pub double_verify: ::std::option::Option<::std::boxed::Box<DoubleVerify>>,
    #[serde(rename = "integralAdScience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third party brand verifier -- Integral Ad Science."]
    pub integral_ad_science: ::std::option::Option<::std::boxed::Box<IntegralAdScience>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A time range."]
pub struct TimeRange {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The upper bound of a time range, inclusive."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The lower bound of a time range, inclusive."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Timer event of the creative."]
pub struct TimerEvent {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the timer event."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportingName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name used to identify this timer event in reports."]
    pub reporting_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings that control the behavior of a single Floodlight activity config."]
pub struct TrackingFloodlightActivityConfig {
    #[serde(rename = "floodlightActivityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the Floodlight activity."]
    pub floodlight_activity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postClickLookbackWindowDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The number of days after an ad has been clicked in which a conversion may be counted. Must be between 0 and 90 inclusive."]
    pub post_click_lookback_window_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "postViewLookbackWindowDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The number of days after an ad has been viewed in which a conversion may be counted. Must be between 0 and 90 inclusive."]
    pub post_view_lookback_window_days: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents information about the transcoded audio or video file."]
pub struct Transcode {
    #[serde(rename = "audioBitRateKbps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The bit rate for the audio stream of the transcoded video, or the bit rate for the transcoded audio, in kilobits per second."]
    pub audio_bit_rate_kbps: ::std::option::Option<::std::string::String>,
    #[serde(rename = "audioSampleRateHz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sample rate for the audio stream of the transcoded video, or the sample rate for the transcoded audio, in hertz."]
    pub audio_sample_rate_hz: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bitRateKbps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transcoding bit rate of the transcoded video, in kilobits per second."]
    pub bit_rate_kbps: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimensions of the transcoded video."]
    pub dimensions: ::std::option::Option<::std::boxed::Box<Dimensions>>,
    #[serde(rename = "fileSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the transcoded file, in bytes."]
    pub file_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frameRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The frame rate of the transcoded video, in frames per second."]
    pub frame_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of the transcoded file."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the transcoded file."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transcoded")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if the transcoding was successful."]
    pub transcoded: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A creative identifier provided by a registry that is unique across all platforms. This is part of the VAST 4.0 standard."]
pub struct UniversalAdId {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique creative identifier."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "registry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The registry provides unique creative identifiers."]
    pub registry: ::std::option::Option<UniversalAdIdRegistryEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The registry provides unique creative identifiers."]
pub enum UniversalAdIdRegistryEnum {
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_UNSPECIFIED")]
    #[doc = "The Universal Ad registry is unspecified or is unknown in this version."]
    UniversalAdRegistryUnspecified,
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_OTHER")]
    #[doc = "Use a custom provider to provide the Universal Ad ID."]
    UniversalAdRegistryOther,
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_AD_ID")]
    #[doc = "Use Ad-ID to provide the Universal Ad ID."]
    UniversalAdRegistryAdId,
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_CLEARCAST")]
    #[doc = "Use clearcast.co.uk to provide the Universal Ad ID."]
    UniversalAdRegistryClearcast,
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_DV360")]
    #[doc = "Use Display & Video 360 to provide the Universal Ad ID."]
    UniversalAdRegistryDv360,
    #[serde(rename = "UNIVERSAL_AD_REGISTRY_CM")]
    #[doc = "Use Campaign Manager 360 to provide the Universal Ad ID."]
    UniversalAdRegistryCm,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details for assigned URL targeting option. This will be populated in the details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_URL`."]
pub struct UrlAssignedTargetingOptionDetails {
    #[serde(rename = "negative")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this option is being negatively targeted."]
    pub negative: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URL, for example `example.com`. DV360 supports two levels of subdirectory targeting, for example `www.example.com/one-subdirectory-level/second-level`, and five levels of subdomain targeting, for example `five.four.three.two.one.example.com`."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single user in Display & Video 360."]
pub struct User {
    #[serde(rename = "assignedUserRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The assigned user roles. Required in CreateUser. Output only in UpdateUser. Can only be updated through BulkEditAssignedUserRoles."]
    pub assigned_user_roles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignedUserRole>>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the user. Must be UTF-8 encoded with a maximum size of 240 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The email address used to identify the user."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the user."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique ID of the user. Assigned by the system."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User rewarded content targeting option details. This will be populated in the user_rewarded_content_details field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`."]
pub struct UserRewardedContentAssignedTargetingOptionDetails {
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userRewardedContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User rewarded content status for video ads."]
    pub user_rewarded_content: ::std::option::Option<
        UserRewardedContentAssignedTargetingOptionDetailsUserRewardedContentEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. User rewarded content status for video ads."]
pub enum UserRewardedContentAssignedTargetingOptionDetailsUserRewardedContentEnum {
    #[serde(rename = "USER_REWARDED_CONTENT_UNSPECIFIED")]
    #[doc = "User rewarded content is not specified or is unknown in this version."]
    UserRewardedContentUnspecified,
    #[serde(rename = "USER_REWARDED_CONTENT_USER_REWARDED")]
    #[doc = "Represents ads where the user will see a reward after viewing."]
    UserRewardedContentUserRewarded,
    #[serde(rename = "USER_REWARDED_CONTENT_NOT_USER_REWARDED")]
    #[doc = "Represents all other ads besides user-rewarded."]
    UserRewardedContentNotUserRewarded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable user rewarded content status for video ads only. This will be populated in the user_rewarded_content_details field when targeting_type is `TARGETING_TYPE_USER_REWARDED_CONTENT`."]
pub struct UserRewardedContentTargetingOptionDetails {
    #[serde(rename = "userRewardedContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User rewarded content status for video ads."]
    pub user_rewarded_content:
        ::std::option::Option<UserRewardedContentTargetingOptionDetailsUserRewardedContentEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. User rewarded content status for video ads."]
pub enum UserRewardedContentTargetingOptionDetailsUserRewardedContentEnum {
    #[serde(rename = "USER_REWARDED_CONTENT_UNSPECIFIED")]
    #[doc = "User rewarded content is not specified or is unknown in this version."]
    UserRewardedContentUnspecified,
    #[serde(rename = "USER_REWARDED_CONTENT_USER_REWARDED")]
    #[doc = "Represents ads where the user will see a reward after viewing."]
    UserRewardedContentUserRewarded,
    #[serde(rename = "USER_REWARDED_CONTENT_NOT_USER_REWARDED")]
    #[doc = "Represents all other ads besides user-rewarded."]
    UserRewardedContentNotUserRewarded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video player size targeting option details. This will be populated in the video_player_size_details field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`. Explicitly targeting all options is not supported. Remove all video player size targeting options to achieve this effect."]
pub struct VideoPlayerSizeAssignedTargetingOptionDetails {
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoPlayerSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The video player size."]
    pub video_player_size:
        ::std::option::Option<VideoPlayerSizeAssignedTargetingOptionDetailsVideoPlayerSizeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The video player size."]
pub enum VideoPlayerSizeAssignedTargetingOptionDetailsVideoPlayerSizeEnum {
    #[serde(rename = "VIDEO_PLAYER_SIZE_UNSPECIFIED")]
    #[doc = "Video player size is not specified in this version. This enum is a place holder for a default value and does not represent a real video player size."]
    VideoPlayerSizeUnspecified,
    #[serde(rename = "VIDEO_PLAYER_SIZE_SMALL")]
    #[doc = "The dimensions of the video player are less than 400×300 (desktop), or up to 20% of screen covered (mobile)."]
    VideoPlayerSizeSmall,
    #[serde(rename = "VIDEO_PLAYER_SIZE_LARGE")]
    #[doc = "The dimensions of the video player are between 400x300 and 1280x720 pixels (desktop), or 20% to 90% of the screen covered (mobile)."]
    VideoPlayerSizeLarge,
    #[serde(rename = "VIDEO_PLAYER_SIZE_HD")]
    #[doc = "The dimensions of the video player are 1280×720 or greater (desktop), or over 90% of the screen covered (mobile)."]
    VideoPlayerSizeHd,
    #[serde(rename = "VIDEO_PLAYER_SIZE_UNKNOWN")]
    #[doc = "The dimensions of the video player are unknown."]
    VideoPlayerSizeUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable video player size. This will be populated in the video_player_size_details field when targeting_type is `TARGETING_TYPE_VIDEO_PLAYER_SIZE`."]
pub struct VideoPlayerSizeTargetingOptionDetails {
    #[serde(rename = "videoPlayerSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The video player size."]
    pub video_player_size:
        ::std::option::Option<VideoPlayerSizeTargetingOptionDetailsVideoPlayerSizeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The video player size."]
pub enum VideoPlayerSizeTargetingOptionDetailsVideoPlayerSizeEnum {
    #[serde(rename = "VIDEO_PLAYER_SIZE_UNSPECIFIED")]
    #[doc = "Video player size is not specified in this version. This enum is a place holder for a default value and does not represent a real video player size."]
    VideoPlayerSizeUnspecified,
    #[serde(rename = "VIDEO_PLAYER_SIZE_SMALL")]
    #[doc = "The dimensions of the video player are less than 400×300 (desktop), or up to 20% of screen covered (mobile)."]
    VideoPlayerSizeSmall,
    #[serde(rename = "VIDEO_PLAYER_SIZE_LARGE")]
    #[doc = "The dimensions of the video player are between 400x300 and 1280x720 pixels (desktop), or 20% to 90% of the screen covered (mobile)."]
    VideoPlayerSizeLarge,
    #[serde(rename = "VIDEO_PLAYER_SIZE_HD")]
    #[doc = "The dimensions of the video player are 1280×720 or greater (desktop), or over 90% of the screen covered (mobile)."]
    VideoPlayerSizeHd,
    #[serde(rename = "VIDEO_PLAYER_SIZE_UNKNOWN")]
    #[doc = "The dimensions of the video player are unknown."]
    VideoPlayerSizeUnknown,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Assigned viewability targeting option details. This will be populated in the viewability_details field of an AssignedTargetingOption when targeting_type is `TARGETING_TYPE_VIEWABILITY`."]
pub struct ViewabilityAssignedTargetingOptionDetails {
    #[serde(rename = "targetingOptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The targeting_option_id of a TargetingOption of type `TARGETING_TYPE_VIEWABILITY` (e.g., \"509010\" for targeting the `VIEWABILITY_10_PERCENT_OR_MORE` option)."]
    pub targeting_option_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The predicted viewability percentage."]
    pub viewability:
        ::std::option::Option<ViewabilityAssignedTargetingOptionDetailsViewabilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The predicted viewability percentage."]
pub enum ViewabilityAssignedTargetingOptionDetailsViewabilityEnum {
    #[serde(rename = "VIEWABILITY_UNSPECIFIED")]
    #[doc = "Default value when viewability is not specified in this version. This enum is a placeholder for default value and does not represent a real viewability option."]
    ViewabilityUnspecified,
    #[serde(rename = "VIEWABILITY_10_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 10% likely to be viewable."]
    Viewability10PercentOrMore,
    #[serde(rename = "VIEWABILITY_20_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 20% likely to be viewable."]
    Viewability20PercentOrMore,
    #[serde(rename = "VIEWABILITY_30_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 30% likely to be viewable."]
    Viewability30PercentOrMore,
    #[serde(rename = "VIEWABILITY_40_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 40% likely to be viewable."]
    Viewability40PercentOrMore,
    #[serde(rename = "VIEWABILITY_50_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 50% likely to be viewable."]
    Viewability50PercentOrMore,
    #[serde(rename = "VIEWABILITY_60_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 60% likely to be viewable."]
    Viewability60PercentOrMore,
    #[serde(rename = "VIEWABILITY_70_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 70% likely to be viewable."]
    Viewability70PercentOrMore,
    #[serde(rename = "VIEWABILITY_80_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 80% likely to be viewable."]
    Viewability80PercentOrMore,
    #[serde(rename = "VIEWABILITY_90_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 90% likely to be viewable."]
    Viewability90PercentOrMore,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a targetable viewability. This will be populated in the viewability_details field of a TargetingOption when targeting_type is `TARGETING_TYPE_VIEWABILITY`."]
pub struct ViewabilityTargetingOptionDetails {
    #[serde(rename = "viewability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The predicted viewability percentage."]
    pub viewability: ::std::option::Option<ViewabilityTargetingOptionDetailsViewabilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The predicted viewability percentage."]
pub enum ViewabilityTargetingOptionDetailsViewabilityEnum {
    #[serde(rename = "VIEWABILITY_UNSPECIFIED")]
    #[doc = "Default value when viewability is not specified in this version. This enum is a placeholder for default value and does not represent a real viewability option."]
    ViewabilityUnspecified,
    #[serde(rename = "VIEWABILITY_10_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 10% likely to be viewable."]
    Viewability10PercentOrMore,
    #[serde(rename = "VIEWABILITY_20_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 20% likely to be viewable."]
    Viewability20PercentOrMore,
    #[serde(rename = "VIEWABILITY_30_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 30% likely to be viewable."]
    Viewability30PercentOrMore,
    #[serde(rename = "VIEWABILITY_40_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 40% likely to be viewable."]
    Viewability40PercentOrMore,
    #[serde(rename = "VIEWABILITY_50_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 50% likely to be viewable."]
    Viewability50PercentOrMore,
    #[serde(rename = "VIEWABILITY_60_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 60% likely to be viewable."]
    Viewability60PercentOrMore,
    #[serde(rename = "VIEWABILITY_70_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 70% likely to be viewable."]
    Viewability70PercentOrMore,
    #[serde(rename = "VIEWABILITY_80_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 80% likely to be viewable."]
    Viewability80PercentOrMore,
    #[serde(rename = "VIEWABILITY_90_PERCENT_OR_MORE")]
    #[doc = "Bid only on impressions that are at least 90% likely to be viewable."]
    Viewability90PercentOrMore,
}
