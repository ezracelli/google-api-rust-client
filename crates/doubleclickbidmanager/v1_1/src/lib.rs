#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A channel grouping defines a set of rules that can be used to categorize events in a path report."]
pub struct ChannelGrouping {
    #[serde(rename = "fallbackName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name to apply to an event that does not match any of the rules in the channel grouping."]
    pub fallback_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Channel Grouping name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rules within Channel Grouping. There is a limit of 100 rules that can be set per channel grouping."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Rule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DisjunctiveMatchStatement that OR's all contained filters."]
pub struct DisjunctiveMatchStatement {
    #[serde(rename = "eventFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters. There is a limit of 100 filters that can be set per disjunctive match statement."]
    pub event_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventFilter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to fetch stored line items."]
pub struct DownloadLineItemsRequest {
    #[serde(rename = "fileSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File specification (column names, types, order) in which the line items will be returned. Default to EWF."]
    pub file_spec: ::std::option::Option<DownloadLineItemsRequestFileSpecEnum>,
    #[serde(rename = "filterIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ids of the specified filter type used to filter line items to fetch. If omitted, all the line items will be returned."]
    pub filter_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "filterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter type used to filter line items to fetch."]
    pub filter_type: ::std::option::Option<DownloadLineItemsRequestFilterTypeEnum>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format in which the line items will be returned. Default to CSV."]
    pub format: ::std::option::Option<DownloadLineItemsRequestFormatEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "File specification (column names, types, order) in which the line items will be returned. Default to EWF."]
pub enum DownloadLineItemsRequestFileSpecEnum {
    #[serde(rename = "EWF")]
    #[doc = ""]
    Ewf,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Filter type used to filter line items to fetch."]
pub enum DownloadLineItemsRequestFilterTypeEnum {
    #[serde(rename = "ADVERTISER_ID")]
    #[doc = ""]
    AdvertiserId,
    #[serde(rename = "INSERTION_ORDER_ID")]
    #[doc = ""]
    InsertionOrderId,
    #[serde(rename = "LINE_ITEM_ID")]
    #[doc = ""]
    LineItemId,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Format in which the line items will be returned. Default to CSV."]
pub enum DownloadLineItemsRequestFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Download line items response."]
pub struct DownloadLineItemsResponse {
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved line items in CSV format. For more information about file formats, see Entity Write File Format."]
    pub line_items: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to fetch stored inventory sources, campaigns, insertion orders, line items, YouTube ad groups and ads."]
pub struct DownloadRequest {
    #[serde(rename = "fileTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File types that will be returned. If INVENTORY_SOURCE is requested, no other file types may be requested. Acceptable values are: - \"AD\" - \"AD_GROUP\" - \"CAMPAIGN\" - \"INSERTION_ORDER\" - \"INVENTORY_SOURCE\" - \"LINE_ITEM\" "]
    pub file_types: ::std::option::Option<::std::vec::Vec<DownloadRequestFileTypesEnum>>,
    #[serde(rename = "filterIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of the specified filter type. This is used to filter entities to fetch. At least one ID must be specified."]
    pub filter_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "filterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter type used to filter entities to fetch. PARTNER_ID and INVENTORY_SOURCE_ID may only be used when downloading inventory sources."]
    pub filter_type: ::std::option::Option<DownloadRequestFilterTypeEnum>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "SDF Version (column names, types, order) in which the entities will be returned. Default to 5."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DownloadRequestFileTypesEnum {
    #[serde(rename = "INSERTION_ORDER")]
    #[doc = ""]
    InsertionOrder,
    #[serde(rename = "LINE_ITEM")]
    #[doc = ""]
    LineItem,
    #[serde(rename = "AD_GROUP")]
    #[doc = ""]
    AdGroup,
    #[serde(rename = "AD")]
    #[doc = ""]
    Ad,
    #[serde(rename = "CAMPAIGN")]
    #[doc = ""]
    Campaign,
    #[serde(rename = "INVENTORY_SOURCE")]
    #[doc = ""]
    InventorySource,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Filter type used to filter entities to fetch. PARTNER_ID and INVENTORY_SOURCE_ID may only be used when downloading inventory sources."]
pub enum DownloadRequestFilterTypeEnum {
    #[serde(rename = "ADVERTISER_ID")]
    #[doc = ""]
    AdvertiserId,
    #[serde(rename = "INSERTION_ORDER_ID")]
    #[doc = ""]
    InsertionOrderId,
    #[serde(rename = "LINE_ITEM_ID")]
    #[doc = ""]
    LineItemId,
    #[serde(rename = "CAMPAIGN_ID")]
    #[doc = ""]
    CampaignId,
    #[serde(rename = "INVENTORY_SOURCE_ID")]
    #[doc = ""]
    InventorySourceId,
    #[serde(rename = "PARTNER_ID")]
    #[doc = ""]
    PartnerId,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Download response."]
pub struct DownloadResponse {
    #[serde(rename = "adGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved ad groups in SDF format."]
    pub ad_groups: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ads")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved ads in SDF format."]
    pub ads: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaigns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved campaigns in SDF format."]
    pub campaigns: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertionOrders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved insertion orders in SDF format."]
    pub insertion_orders: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventorySources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub inventory_sources: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved line items in SDF format."]
    pub line_items: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the type of filter to be applied to the path, a DV360 event dimension filter."]
pub struct EventFilter {
    #[serde(rename = "dimensionFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter on a dimension."]
    pub dimension_filter: ::std::option::Option<::std::boxed::Box<PathQueryOptionsFilter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Filter used to match traffic data in your report."]
pub struct FilterPair {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter type."]
    pub _type: ::std::option::Option<FilterPairTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Filter type."]
pub enum FilterPairTypeEnum {
    #[serde(rename = "FILTER_UNKNOWN")]
    #[doc = ""]
    FilterUnknown,
    #[serde(rename = "FILTER_DATE")]
    #[doc = ""]
    FilterDate,
    #[serde(rename = "FILTER_DAY_OF_WEEK")]
    #[doc = ""]
    FilterDayOfWeek,
    #[serde(rename = "FILTER_WEEK")]
    #[doc = ""]
    FilterWeek,
    #[serde(rename = "FILTER_MONTH")]
    #[doc = ""]
    FilterMonth,
    #[serde(rename = "FILTER_YEAR")]
    #[doc = ""]
    FilterYear,
    #[serde(rename = "FILTER_TIME_OF_DAY")]
    #[doc = ""]
    FilterTimeOfDay,
    #[serde(rename = "FILTER_CONVERSION_DELAY")]
    #[doc = ""]
    FilterConversionDelay,
    #[serde(rename = "FILTER_CREATIVE_ID")]
    #[doc = ""]
    FilterCreativeId,
    #[serde(rename = "FILTER_CREATIVE_SIZE")]
    #[doc = ""]
    FilterCreativeSize,
    #[serde(rename = "FILTER_CREATIVE_TYPE")]
    #[doc = ""]
    FilterCreativeType,
    #[serde(rename = "FILTER_EXCHANGE_ID")]
    #[doc = ""]
    FilterExchangeId,
    #[serde(rename = "FILTER_AD_POSITION")]
    #[doc = ""]
    FilterAdPosition,
    #[serde(rename = "FILTER_PUBLIC_INVENTORY")]
    #[doc = ""]
    FilterPublicInventory,
    #[serde(rename = "FILTER_INVENTORY_SOURCE")]
    #[doc = ""]
    FilterInventorySource,
    #[serde(rename = "FILTER_CITY")]
    #[doc = ""]
    FilterCity,
    #[serde(rename = "FILTER_REGION")]
    #[doc = ""]
    FilterRegion,
    #[serde(rename = "FILTER_DMA")]
    #[doc = ""]
    FilterDma,
    #[serde(rename = "FILTER_COUNTRY")]
    #[doc = ""]
    FilterCountry,
    #[serde(rename = "FILTER_SITE_ID")]
    #[doc = ""]
    FilterSiteId,
    #[serde(rename = "FILTER_CHANNEL_ID")]
    #[doc = ""]
    FilterChannelId,
    #[serde(rename = "FILTER_PARTNER")]
    #[doc = ""]
    FilterPartner,
    #[serde(rename = "FILTER_ADVERTISER")]
    #[doc = ""]
    FilterAdvertiser,
    #[serde(rename = "FILTER_INSERTION_ORDER")]
    #[doc = ""]
    FilterInsertionOrder,
    #[serde(rename = "FILTER_LINE_ITEM")]
    #[doc = ""]
    FilterLineItem,
    #[serde(rename = "FILTER_PARTNER_CURRENCY")]
    #[doc = ""]
    FilterPartnerCurrency,
    #[serde(rename = "FILTER_ADVERTISER_CURRENCY")]
    #[doc = ""]
    FilterAdvertiserCurrency,
    #[serde(rename = "FILTER_ADVERTISER_TIMEZONE")]
    #[doc = ""]
    FilterAdvertiserTimezone,
    #[serde(rename = "FILTER_LINE_ITEM_TYPE")]
    #[doc = ""]
    FilterLineItemType,
    #[serde(rename = "FILTER_USER_LIST")]
    #[doc = ""]
    FilterUserList,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY")]
    #[doc = ""]
    FilterUserListFirstParty,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY")]
    #[doc = ""]
    FilterUserListThirdParty,
    #[serde(rename = "FILTER_TARGETED_USER_LIST")]
    #[doc = ""]
    FilterTargetedUserList,
    #[serde(rename = "FILTER_DATA_PROVIDER")]
    #[doc = ""]
    FilterDataProvider,
    #[serde(rename = "FILTER_ORDER_ID")]
    #[doc = ""]
    FilterOrderId,
    #[serde(rename = "FILTER_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVideoPlayerSize,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS")]
    #[doc = ""]
    FilterVideoDurationSeconds,
    #[serde(rename = "FILTER_KEYWORD")]
    #[doc = ""]
    FilterKeyword,
    #[serde(rename = "FILTER_PAGE_CATEGORY")]
    #[doc = ""]
    FilterPageCategory,
    #[serde(rename = "FILTER_CAMPAIGN_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterCampaignDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterLineItemDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_LIFETIME_FREQUENCY")]
    #[doc = ""]
    FilterLineItemLifetimeFrequency,
    #[serde(rename = "FILTER_OS")]
    #[doc = ""]
    FilterOs,
    #[serde(rename = "FILTER_BROWSER")]
    #[doc = ""]
    FilterBrowser,
    #[serde(rename = "FILTER_CARRIER")]
    #[doc = ""]
    FilterCarrier,
    #[serde(rename = "FILTER_SITE_LANGUAGE")]
    #[doc = ""]
    FilterSiteLanguage,
    #[serde(rename = "FILTER_INVENTORY_FORMAT")]
    #[doc = ""]
    FilterInventoryFormat,
    #[serde(rename = "FILTER_ZIP_CODE")]
    #[doc = ""]
    FilterZipCode,
    #[serde(rename = "FILTER_VIDEO_RATING_TIER")]
    #[doc = ""]
    FilterVideoRatingTier,
    #[serde(rename = "FILTER_VIDEO_FORMAT_SUPPORT")]
    #[doc = ""]
    FilterVideoFormatSupport,
    #[serde(rename = "FILTER_VIDEO_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterVideoSkippableSupport,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION")]
    #[doc = ""]
    FilterVideoCreativeDuration,
    #[serde(rename = "FILTER_PAGE_LAYOUT")]
    #[doc = ""]
    FilterPageLayout,
    #[serde(rename = "FILTER_VIDEO_AD_POSITION_IN_STREAM")]
    #[doc = ""]
    FilterVideoAdPositionInStream,
    #[serde(rename = "FILTER_AGE")]
    #[doc = ""]
    FilterAge,
    #[serde(rename = "FILTER_GENDER")]
    #[doc = ""]
    FilterGender,
    #[serde(rename = "FILTER_QUARTER")]
    #[doc = ""]
    FilterQuarter,
    #[serde(rename = "FILTER_TRUEVIEW_CONVERSION_TYPE")]
    #[doc = ""]
    FilterTrueviewConversionType,
    #[serde(rename = "FILTER_MOBILE_GEO")]
    #[doc = ""]
    FilterMobileGeo,
    #[serde(rename = "FILTER_MRAID_SUPPORT")]
    #[doc = ""]
    FilterMraidSupport,
    #[serde(rename = "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY")]
    #[doc = ""]
    FilterActiveViewExpectedViewability,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE")]
    #[doc = ""]
    FilterVideoCreativeDurationSkippable,
    #[serde(rename = "FILTER_NIELSEN_COUNTRY_CODE")]
    #[doc = ""]
    FilterNielsenCountryCode,
    #[serde(rename = "FILTER_NIELSEN_DEVICE_ID")]
    #[doc = ""]
    FilterNielsenDeviceId,
    #[serde(rename = "FILTER_NIELSEN_GENDER")]
    #[doc = ""]
    FilterNielsenGender,
    #[serde(rename = "FILTER_NIELSEN_AGE")]
    #[doc = ""]
    FilterNielsenAge,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_TYPE")]
    #[doc = ""]
    FilterInventorySourceType,
    #[serde(rename = "FILTER_CREATIVE_WIDTH")]
    #[doc = ""]
    FilterCreativeWidth,
    #[serde(rename = "FILTER_CREATIVE_HEIGHT")]
    #[doc = ""]
    FilterCreativeHeight,
    #[serde(rename = "FILTER_DFP_ORDER_ID")]
    #[doc = ""]
    FilterDfpOrderId,
    #[serde(rename = "FILTER_TRUEVIEW_AGE")]
    #[doc = ""]
    FilterTrueviewAge,
    #[serde(rename = "FILTER_TRUEVIEW_GENDER")]
    #[doc = ""]
    FilterTrueviewGender,
    #[serde(rename = "FILTER_TRUEVIEW_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_INTEREST")]
    #[doc = ""]
    FilterTrueviewInterest,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupId,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_AD_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupAdId,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_LANGUAGE")]
    #[doc = ""]
    FilterTrueviewIarLanguage,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_GENDER")]
    #[doc = ""]
    FilterTrueviewIarGender,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_AGE")]
    #[doc = ""]
    FilterTrueviewIarAge,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CATEGORY")]
    #[doc = ""]
    FilterTrueviewIarCategory,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY")]
    #[doc = ""]
    FilterTrueviewIarCountry,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CITY")]
    #[doc = ""]
    FilterTrueviewIarCity,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION")]
    #[doc = ""]
    FilterTrueviewIarRegion,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewIarZipcode,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewIarRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_INTEREST")]
    #[doc = ""]
    FilterTrueviewIarInterest,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewIarParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_TIME_OF_DAY")]
    #[doc = ""]
    FilterTrueviewIarTimeOfDay,
    #[serde(rename = "FILTER_TRUEVIEW_CUSTOM_AFFINITY")]
    #[doc = ""]
    FilterTrueviewCustomAffinity,
    #[serde(rename = "FILTER_TRUEVIEW_CATEGORY")]
    #[doc = ""]
    FilterTrueviewCategory,
    #[serde(rename = "FILTER_TRUEVIEW_KEYWORD")]
    #[doc = ""]
    FilterTrueviewKeyword,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT")]
    #[doc = ""]
    FilterTrueviewPlacement,
    #[serde(rename = "FILTER_TRUEVIEW_URL")]
    #[doc = ""]
    FilterTrueviewUrl,
    #[serde(rename = "FILTER_TRUEVIEW_COUNTRY")]
    #[doc = ""]
    FilterTrueviewCountry,
    #[serde(rename = "FILTER_TRUEVIEW_REGION")]
    #[doc = ""]
    FilterTrueviewRegion,
    #[serde(rename = "FILTER_TRUEVIEW_CITY")]
    #[doc = ""]
    FilterTrueviewCity,
    #[serde(rename = "FILTER_TRUEVIEW_DMA")]
    #[doc = ""]
    FilterTrueviewDma,
    #[serde(rename = "FILTER_TRUEVIEW_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewZipcode,
    #[serde(rename = "FILTER_NOT_SUPPORTED")]
    #[doc = ""]
    FilterNotSupported,
    #[serde(rename = "FILTER_MEDIA_PLAN")]
    #[doc = ""]
    FilterMediaPlan,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterTrueviewIarYoutubeChannel,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterTrueviewIarYoutubeVideo,
    #[serde(rename = "FILTER_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterSkippableSupport,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_ID")]
    #[doc = ""]
    FilterCompanionCreativeId,
    #[serde(rename = "FILTER_BUDGET_SEGMENT_DESCRIPTION")]
    #[doc = ""]
    FilterBudgetSegmentDescription,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY_ID")]
    #[doc = ""]
    FilterFloodlightActivityId,
    #[serde(rename = "FILTER_DEVICE_MODEL")]
    #[doc = ""]
    FilterDeviceModel,
    #[serde(rename = "FILTER_DEVICE_MAKE")]
    #[doc = ""]
    FilterDeviceMake,
    #[serde(rename = "FILTER_DEVICE_TYPE")]
    #[doc = ""]
    FilterDeviceType,
    #[serde(rename = "FILTER_CREATIVE_ATTRIBUTE")]
    #[doc = ""]
    FilterCreativeAttribute,
    #[serde(rename = "FILTER_INVENTORY_COMMITMENT_TYPE")]
    #[doc = ""]
    FilterInventoryCommitmentType,
    #[serde(rename = "FILTER_INVENTORY_RATE_TYPE")]
    #[doc = ""]
    FilterInventoryRateType,
    #[serde(rename = "FILTER_INVENTORY_DELIVERY_METHOD")]
    #[doc = ""]
    FilterInventoryDeliveryMethod,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_EXTERNAL_ID")]
    #[doc = ""]
    FilterInventorySourceExternalId,
    #[serde(rename = "FILTER_AUTHORIZED_SELLER_STATE")]
    #[doc = ""]
    FilterAuthorizedSellerState,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS_RANGE")]
    #[doc = ""]
    FilterVideoDurationSecondsRange,
    #[serde(rename = "FILTER_PARTNER_NAME")]
    #[doc = ""]
    FilterPartnerName,
    #[serde(rename = "FILTER_PARTNER_STATUS")]
    #[doc = ""]
    FilterPartnerStatus,
    #[serde(rename = "FILTER_ADVERTISER_NAME")]
    #[doc = ""]
    FilterAdvertiserName,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterAdvertiserIntegrationCode,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_STATUS")]
    #[doc = ""]
    FilterAdvertiserIntegrationStatus,
    #[serde(rename = "FILTER_CARRIER_NAME")]
    #[doc = ""]
    FilterCarrierName,
    #[serde(rename = "FILTER_CHANNEL_NAME")]
    #[doc = ""]
    FilterChannelName,
    #[serde(rename = "FILTER_CITY_NAME")]
    #[doc = ""]
    FilterCityName,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_NAME")]
    #[doc = ""]
    FilterCompanionCreativeName,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY_NAME")]
    #[doc = ""]
    FilterUserListFirstPartyName,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY_NAME")]
    #[doc = ""]
    FilterUserListThirdPartyName,
    #[serde(rename = "FILTER_NIELSEN_RESTATEMENT_DATE")]
    #[doc = ""]
    FilterNielsenRestatementDate,
    #[serde(rename = "FILTER_NIELSEN_DATE_RANGE")]
    #[doc = ""]
    FilterNielsenDateRange,
    #[serde(rename = "FILTER_INSERTION_ORDER_NAME")]
    #[doc = ""]
    FilterInsertionOrderName,
    #[serde(rename = "FILTER_REGION_NAME")]
    #[doc = ""]
    FilterRegionName,
    #[serde(rename = "FILTER_DMA_NAME")]
    #[doc = ""]
    FilterDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewIarRegionName,
    #[serde(rename = "FILTER_TRUEVIEW_DMA_NAME")]
    #[doc = ""]
    FilterTrueviewDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewRegionName,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID")]
    #[doc = ""]
    FilterActiveViewCustomMetricId,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME")]
    #[doc = ""]
    FilterActiveViewCustomMetricName,
    #[serde(rename = "FILTER_AD_TYPE")]
    #[doc = ""]
    FilterAdType,
    #[serde(rename = "FILTER_ALGORITHM")]
    #[doc = ""]
    FilterAlgorithm,
    #[serde(rename = "FILTER_ALGORITHM_ID")]
    #[doc = ""]
    FilterAlgorithmId,
    #[serde(rename = "FILTER_AMP_PAGE_REQUEST")]
    #[doc = ""]
    FilterAmpPageRequest,
    #[serde(rename = "FILTER_ANONYMOUS_INVENTORY_MODELING")]
    #[doc = ""]
    FilterAnonymousInventoryModeling,
    #[serde(rename = "FILTER_APP_URL")]
    #[doc = ""]
    FilterAppUrl,
    #[serde(rename = "FILTER_APP_URL_EXCLUDED")]
    #[doc = ""]
    FilterAppUrlExcluded,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST")]
    #[doc = ""]
    FilterAttributedUserlist,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_COST")]
    #[doc = ""]
    FilterAttributedUserlistCost,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_TYPE")]
    #[doc = ""]
    FilterAttributedUserlistType,
    #[serde(rename = "FILTER_ATTRIBUTION_MODEL")]
    #[doc = ""]
    FilterAttributionModel,
    #[serde(rename = "FILTER_AUDIENCE_LIST")]
    #[doc = ""]
    FilterAudienceList,
    #[serde(rename = "FILTER_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterAudienceListCost,
    #[serde(rename = "FILTER_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterAudienceListType,
    #[serde(rename = "FILTER_AUDIENCE_NAME")]
    #[doc = ""]
    FilterAudienceName,
    #[serde(rename = "FILTER_AUDIENCE_TYPE")]
    #[doc = ""]
    FilterAudienceType,
    #[serde(rename = "FILTER_BILLABLE_OUTCOME")]
    #[doc = ""]
    FilterBillableOutcome,
    #[serde(rename = "FILTER_BRAND_LIFT_TYPE")]
    #[doc = ""]
    FilterBrandLiftType,
    #[serde(rename = "FILTER_CHANNEL_TYPE")]
    #[doc = ""]
    FilterChannelType,
    #[serde(rename = "FILTER_CM_PLACEMENT_ID")]
    #[doc = ""]
    FilterCmPlacementId,
    #[serde(rename = "FILTER_CONVERSION_SOURCE")]
    #[doc = ""]
    FilterConversionSource,
    #[serde(rename = "FILTER_CONVERSION_SOURCE_ID")]
    #[doc = ""]
    FilterConversionSourceId,
    #[serde(rename = "FILTER_COUNTRY_ID")]
    #[doc = ""]
    FilterCountryId,
    #[serde(rename = "FILTER_CREATIVE")]
    #[doc = ""]
    FilterCreative,
    #[serde(rename = "FILTER_CREATIVE_ASSET")]
    #[doc = ""]
    FilterCreativeAsset,
    #[serde(rename = "FILTER_CREATIVE_INTEGRATION_CODE")]
    #[doc = ""]
    FilterCreativeIntegrationCode,
    #[serde(rename = "FILTER_CREATIVE_RENDERED_IN_AMP")]
    #[doc = ""]
    FilterCreativeRenderedInAmp,
    #[serde(rename = "FILTER_CREATIVE_SOURCE")]
    #[doc = ""]
    FilterCreativeSource,
    #[serde(rename = "FILTER_CREATIVE_STATUS")]
    #[doc = ""]
    FilterCreativeStatus,
    #[serde(rename = "FILTER_DATA_PROVIDER_NAME")]
    #[doc = ""]
    FilterDataProviderName,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterDetailedDemographics,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterDetailedDemographicsId,
    #[serde(rename = "FILTER_DEVICE")]
    #[doc = ""]
    FilterDevice,
    #[serde(rename = "FILTER_GAM_INSERTION_ORDER")]
    #[doc = ""]
    FilterGamInsertionOrder,
    #[serde(rename = "FILTER_GAM_LINE_ITEM")]
    #[doc = ""]
    FilterGamLineItem,
    #[serde(rename = "FILTER_GAM_LINE_ITEM_ID")]
    #[doc = ""]
    FilterGamLineItemId,
    #[serde(rename = "FILTER_DIGITAL_CONTENT_LABEL")]
    #[doc = ""]
    FilterDigitalContentLabel,
    #[serde(rename = "FILTER_DOMAIN")]
    #[doc = ""]
    FilterDomain,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST")]
    #[doc = ""]
    FilterEligibleCookiesOnFirstPartyAudienceList,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST")]
    #[doc = ""]
    FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
    #[serde(rename = "FILTER_EXCHANGE")]
    #[doc = ""]
    FilterExchange,
    #[serde(rename = "FILTER_EXCHANGE_CODE")]
    #[doc = ""]
    FilterExchangeCode,
    #[serde(rename = "FILTER_EXTENSION")]
    #[doc = ""]
    FilterExtension,
    #[serde(rename = "FILTER_EXTENSION_STATUS")]
    #[doc = ""]
    FilterExtensionStatus,
    #[serde(rename = "FILTER_EXTENSION_TYPE")]
    #[doc = ""]
    FilterExtensionType,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterFirstPartyAudienceListCost,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterFirstPartyAudienceListType,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY")]
    #[doc = ""]
    FilterFloodlightActivity,
    #[serde(rename = "FILTER_FORMAT")]
    #[doc = ""]
    FilterFormat,
    #[serde(rename = "FILTER_GMAIL_AGE")]
    #[doc = ""]
    FilterGmailAge,
    #[serde(rename = "FILTER_GMAIL_CITY")]
    #[doc = ""]
    FilterGmailCity,
    #[serde(rename = "FILTER_GMAIL_COUNTRY")]
    #[doc = ""]
    FilterGmailCountry,
    #[serde(rename = "FILTER_GMAIL_COUNTRY_NAME")]
    #[doc = ""]
    FilterGmailCountryName,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE")]
    #[doc = ""]
    FilterGmailDeviceType,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE_NAME")]
    #[doc = ""]
    FilterGmailDeviceTypeName,
    #[serde(rename = "FILTER_GMAIL_GENDER")]
    #[doc = ""]
    FilterGmailGender,
    #[serde(rename = "FILTER_GMAIL_REGION")]
    #[doc = ""]
    FilterGmailRegion,
    #[serde(rename = "FILTER_GMAIL_REMARKETING_LIST")]
    #[doc = ""]
    FilterGmailRemarketingList,
    #[serde(rename = "FILTER_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterHouseholdIncome,
    #[serde(rename = "FILTER_IMPRESSION_COUNTING_METHOD")]
    #[doc = ""]
    FilterImpressionCountingMethod,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedInsertionOrder,
    #[serde(rename = "FILTER_INSERTION_ORDER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterInsertionOrderIntegrationCode,
    #[serde(rename = "FILTER_INSERTION_ORDER_STATUS")]
    #[doc = ""]
    FilterInsertionOrderStatus,
    #[serde(rename = "FILTER_INTEREST")]
    #[doc = ""]
    FilterInterest,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP")]
    #[doc = ""]
    FilterInventorySourceGroup,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP_ID")]
    #[doc = ""]
    FilterInventorySourceGroupId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_ID")]
    #[doc = ""]
    FilterInventorySourceId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_NAME")]
    #[doc = ""]
    FilterInventorySourceName,
    #[serde(rename = "FILTER_LIFE_EVENT")]
    #[doc = ""]
    FilterLifeEvent,
    #[serde(rename = "FILTER_LIFE_EVENTS")]
    #[doc = ""]
    FilterLifeEvents,
    #[serde(rename = "FILTER_LINE_ITEM_INTEGRATION_CODE")]
    #[doc = ""]
    FilterLineItemIntegrationCode,
    #[serde(rename = "FILTER_LINE_ITEM_NAME")]
    #[doc = ""]
    FilterLineItemName,
    #[serde(rename = "FILTER_LINE_ITEM_STATUS")]
    #[doc = ""]
    FilterLineItemStatus,
    #[serde(rename = "FILTER_MATCH_RATIO")]
    #[doc = ""]
    FilterMatchRatio,
    #[serde(rename = "FILTER_MEASUREMENT_SOURCE")]
    #[doc = ""]
    FilterMeasurementSource,
    #[serde(rename = "FILTER_MEDIA_PLAN_NAME")]
    #[doc = ""]
    FilterMediaPlanName,
    #[serde(rename = "FILTER_PARENTAL_STATUS")]
    #[doc = ""]
    FilterParentalStatus,
    #[serde(rename = "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementAllYoutubeChannels,
    #[serde(rename = "FILTER_PLATFORM")]
    #[doc = ""]
    FilterPlatform,
    #[serde(rename = "FILTER_PLAYBACK_METHOD")]
    #[doc = ""]
    FilterPlaybackMethod,
    #[serde(rename = "FILTER_POSITION_IN_CONTENT")]
    #[doc = ""]
    FilterPositionInContent,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY")]
    #[doc = ""]
    FilterPublisherProperty,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_ID")]
    #[doc = ""]
    FilterPublisherPropertyId,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION")]
    #[doc = ""]
    FilterPublisherPropertySection,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION_ID")]
    #[doc = ""]
    FilterPublisherPropertySectionId,
    #[serde(rename = "FILTER_REFUND_REASON")]
    #[doc = ""]
    FilterRefundReason,
    #[serde(rename = "FILTER_REMARKETING_LIST")]
    #[doc = ""]
    FilterRemarketingList,
    #[serde(rename = "FILTER_REWARDED")]
    #[doc = ""]
    FilterRewarded,
    #[serde(rename = "FILTER_SENSITIVE_CATEGORY")]
    #[doc = ""]
    FilterSensitiveCategory,
    #[serde(rename = "FILTER_SERVED_PIXEL_DENSITY")]
    #[doc = ""]
    FilterServedPixelDensity,
    #[serde(rename = "FILTER_TARGETED_DATA_PROVIDERS")]
    #[doc = ""]
    FilterTargetedDataProviders,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterThirdPartyAudienceListCost,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterThirdPartyAudienceListType,
    #[serde(rename = "FILTER_TRUEVIEW_AD")]
    #[doc = ""]
    FilterTrueviewAd,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP")]
    #[doc = ""]
    FilterTrueviewAdGroup,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterTrueviewDetailedDemographics,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterTrueviewDetailedDemographicsId,
    #[serde(rename = "FILTER_TRUEVIEW_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterTrueviewHouseholdIncome,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY_NAME")]
    #[doc = ""]
    FilterTrueviewIarCountryName,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST_NAME")]
    #[doc = ""]
    FilterTrueviewRemarketingListName,
    #[serde(rename = "FILTER_VARIANT_ID")]
    #[doc = ""]
    FilterVariantId,
    #[serde(rename = "FILTER_VARIANT_NAME")]
    #[doc = ""]
    FilterVariantName,
    #[serde(rename = "FILTER_VARIANT_VERSION")]
    #[doc = ""]
    FilterVariantVersion,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVerificationVideoPlayerSize,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_POSITION")]
    #[doc = ""]
    FilterVerificationVideoPosition,
    #[serde(rename = "FILTER_VIDEO_COMPANION_CREATIVE_SIZE")]
    #[doc = ""]
    FilterVideoCompanionCreativeSize,
    #[serde(rename = "FILTER_VIDEO_CONTINUOUS_PLAY")]
    #[doc = ""]
    FilterVideoContinuousPlay,
    #[serde(rename = "FILTER_VIDEO_DURATION")]
    #[doc = ""]
    FilterVideoDuration,
    #[serde(rename = "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST")]
    #[doc = ""]
    FilterYoutubeAdaptedAudienceList,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO")]
    #[doc = ""]
    FilterYoutubeAdVideo,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO_ID")]
    #[doc = ""]
    FilterYoutubeAdVideoId,
    #[serde(rename = "FILTER_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterYoutubeChannel,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedAdvertiser,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedPartner,
    #[serde(rename = "FILTER_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterYoutubeVideo,
    #[serde(rename = "FILTER_ZIP_POSTAL_CODE")]
    #[doc = ""]
    FilterZipPostalCode,
    #[serde(rename = "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementNameAllYoutubeChannels,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT_ID")]
    #[doc = ""]
    FilterTrueviewPlacementId,
    #[serde(rename = "FILTER_PATH_PATTERN_ID")]
    #[doc = ""]
    FilterPathPatternId,
    #[serde(rename = "FILTER_PATH_EVENT_INDEX")]
    #[doc = ""]
    FilterPathEventIndex,
    #[serde(rename = "FILTER_EVENT_TYPE")]
    #[doc = ""]
    FilterEventType,
    #[serde(rename = "FILTER_CHANNEL_GROUPING")]
    #[doc = ""]
    FilterChannelGrouping,
    #[serde(rename = "FILTER_OM_SDK_AVAILABLE")]
    #[doc = ""]
    FilterOmSdkAvailable,
    #[serde(rename = "FILTER_DATA_SOURCE")]
    #[doc = ""]
    FilterDataSource,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List queries response."]
pub struct ListQueriesResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listQueriesResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Next page's pagination token if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved queries."]
    pub queries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Query>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List reports response."]
pub struct ListReportsResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listReportsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Next page's pagination token if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrieved reports."]
    pub reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Report>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional query options."]
pub struct Options {
    #[serde(rename = "includeOnlyTargetedUserLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true and filter your report by `FILTER_INSERTION_ORDER` or `FILTER_LINE_ITEM` to include data for audience lists specifically targeted by those items."]
    pub include_only_targeted_user_lists: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pathQueryOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options that contain Path Filters and Custom Channel Groupings."]
    pub path_query_options: ::std::option::Option<::std::boxed::Box<PathQueryOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters of a query or report."]
pub struct Parameters {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filters used to match traffic data in your report."]
    pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterPair>>>,
    #[serde(rename = "groupBys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data is grouped by the filters listed in this field."]
    pub group_bys: ::std::option::Option<::std::vec::Vec<ParametersGroupBysEnum>>,
    #[serde(rename = "includeInviteData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field is no longer in use."]
    pub include_invite_data: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics to include as columns in your report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<ParametersMetricsEnum>>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional query options."]
    pub options: ::std::option::Option<::std::boxed::Box<Options>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report type."]
    pub _type: ::std::option::Option<ParametersTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ParametersGroupBysEnum {
    #[serde(rename = "FILTER_UNKNOWN")]
    #[doc = ""]
    FilterUnknown,
    #[serde(rename = "FILTER_DATE")]
    #[doc = ""]
    FilterDate,
    #[serde(rename = "FILTER_DAY_OF_WEEK")]
    #[doc = ""]
    FilterDayOfWeek,
    #[serde(rename = "FILTER_WEEK")]
    #[doc = ""]
    FilterWeek,
    #[serde(rename = "FILTER_MONTH")]
    #[doc = ""]
    FilterMonth,
    #[serde(rename = "FILTER_YEAR")]
    #[doc = ""]
    FilterYear,
    #[serde(rename = "FILTER_TIME_OF_DAY")]
    #[doc = ""]
    FilterTimeOfDay,
    #[serde(rename = "FILTER_CONVERSION_DELAY")]
    #[doc = ""]
    FilterConversionDelay,
    #[serde(rename = "FILTER_CREATIVE_ID")]
    #[doc = ""]
    FilterCreativeId,
    #[serde(rename = "FILTER_CREATIVE_SIZE")]
    #[doc = ""]
    FilterCreativeSize,
    #[serde(rename = "FILTER_CREATIVE_TYPE")]
    #[doc = ""]
    FilterCreativeType,
    #[serde(rename = "FILTER_EXCHANGE_ID")]
    #[doc = ""]
    FilterExchangeId,
    #[serde(rename = "FILTER_AD_POSITION")]
    #[doc = ""]
    FilterAdPosition,
    #[serde(rename = "FILTER_PUBLIC_INVENTORY")]
    #[doc = ""]
    FilterPublicInventory,
    #[serde(rename = "FILTER_INVENTORY_SOURCE")]
    #[doc = ""]
    FilterInventorySource,
    #[serde(rename = "FILTER_CITY")]
    #[doc = ""]
    FilterCity,
    #[serde(rename = "FILTER_REGION")]
    #[doc = ""]
    FilterRegion,
    #[serde(rename = "FILTER_DMA")]
    #[doc = ""]
    FilterDma,
    #[serde(rename = "FILTER_COUNTRY")]
    #[doc = ""]
    FilterCountry,
    #[serde(rename = "FILTER_SITE_ID")]
    #[doc = ""]
    FilterSiteId,
    #[serde(rename = "FILTER_CHANNEL_ID")]
    #[doc = ""]
    FilterChannelId,
    #[serde(rename = "FILTER_PARTNER")]
    #[doc = ""]
    FilterPartner,
    #[serde(rename = "FILTER_ADVERTISER")]
    #[doc = ""]
    FilterAdvertiser,
    #[serde(rename = "FILTER_INSERTION_ORDER")]
    #[doc = ""]
    FilterInsertionOrder,
    #[serde(rename = "FILTER_LINE_ITEM")]
    #[doc = ""]
    FilterLineItem,
    #[serde(rename = "FILTER_PARTNER_CURRENCY")]
    #[doc = ""]
    FilterPartnerCurrency,
    #[serde(rename = "FILTER_ADVERTISER_CURRENCY")]
    #[doc = ""]
    FilterAdvertiserCurrency,
    #[serde(rename = "FILTER_ADVERTISER_TIMEZONE")]
    #[doc = ""]
    FilterAdvertiserTimezone,
    #[serde(rename = "FILTER_LINE_ITEM_TYPE")]
    #[doc = ""]
    FilterLineItemType,
    #[serde(rename = "FILTER_USER_LIST")]
    #[doc = ""]
    FilterUserList,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY")]
    #[doc = ""]
    FilterUserListFirstParty,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY")]
    #[doc = ""]
    FilterUserListThirdParty,
    #[serde(rename = "FILTER_TARGETED_USER_LIST")]
    #[doc = ""]
    FilterTargetedUserList,
    #[serde(rename = "FILTER_DATA_PROVIDER")]
    #[doc = ""]
    FilterDataProvider,
    #[serde(rename = "FILTER_ORDER_ID")]
    #[doc = ""]
    FilterOrderId,
    #[serde(rename = "FILTER_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVideoPlayerSize,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS")]
    #[doc = ""]
    FilterVideoDurationSeconds,
    #[serde(rename = "FILTER_KEYWORD")]
    #[doc = ""]
    FilterKeyword,
    #[serde(rename = "FILTER_PAGE_CATEGORY")]
    #[doc = ""]
    FilterPageCategory,
    #[serde(rename = "FILTER_CAMPAIGN_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterCampaignDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterLineItemDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_LIFETIME_FREQUENCY")]
    #[doc = ""]
    FilterLineItemLifetimeFrequency,
    #[serde(rename = "FILTER_OS")]
    #[doc = ""]
    FilterOs,
    #[serde(rename = "FILTER_BROWSER")]
    #[doc = ""]
    FilterBrowser,
    #[serde(rename = "FILTER_CARRIER")]
    #[doc = ""]
    FilterCarrier,
    #[serde(rename = "FILTER_SITE_LANGUAGE")]
    #[doc = ""]
    FilterSiteLanguage,
    #[serde(rename = "FILTER_INVENTORY_FORMAT")]
    #[doc = ""]
    FilterInventoryFormat,
    #[serde(rename = "FILTER_ZIP_CODE")]
    #[doc = ""]
    FilterZipCode,
    #[serde(rename = "FILTER_VIDEO_RATING_TIER")]
    #[doc = ""]
    FilterVideoRatingTier,
    #[serde(rename = "FILTER_VIDEO_FORMAT_SUPPORT")]
    #[doc = ""]
    FilterVideoFormatSupport,
    #[serde(rename = "FILTER_VIDEO_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterVideoSkippableSupport,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION")]
    #[doc = ""]
    FilterVideoCreativeDuration,
    #[serde(rename = "FILTER_PAGE_LAYOUT")]
    #[doc = ""]
    FilterPageLayout,
    #[serde(rename = "FILTER_VIDEO_AD_POSITION_IN_STREAM")]
    #[doc = ""]
    FilterVideoAdPositionInStream,
    #[serde(rename = "FILTER_AGE")]
    #[doc = ""]
    FilterAge,
    #[serde(rename = "FILTER_GENDER")]
    #[doc = ""]
    FilterGender,
    #[serde(rename = "FILTER_QUARTER")]
    #[doc = ""]
    FilterQuarter,
    #[serde(rename = "FILTER_TRUEVIEW_CONVERSION_TYPE")]
    #[doc = ""]
    FilterTrueviewConversionType,
    #[serde(rename = "FILTER_MOBILE_GEO")]
    #[doc = ""]
    FilterMobileGeo,
    #[serde(rename = "FILTER_MRAID_SUPPORT")]
    #[doc = ""]
    FilterMraidSupport,
    #[serde(rename = "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY")]
    #[doc = ""]
    FilterActiveViewExpectedViewability,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE")]
    #[doc = ""]
    FilterVideoCreativeDurationSkippable,
    #[serde(rename = "FILTER_NIELSEN_COUNTRY_CODE")]
    #[doc = ""]
    FilterNielsenCountryCode,
    #[serde(rename = "FILTER_NIELSEN_DEVICE_ID")]
    #[doc = ""]
    FilterNielsenDeviceId,
    #[serde(rename = "FILTER_NIELSEN_GENDER")]
    #[doc = ""]
    FilterNielsenGender,
    #[serde(rename = "FILTER_NIELSEN_AGE")]
    #[doc = ""]
    FilterNielsenAge,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_TYPE")]
    #[doc = ""]
    FilterInventorySourceType,
    #[serde(rename = "FILTER_CREATIVE_WIDTH")]
    #[doc = ""]
    FilterCreativeWidth,
    #[serde(rename = "FILTER_CREATIVE_HEIGHT")]
    #[doc = ""]
    FilterCreativeHeight,
    #[serde(rename = "FILTER_DFP_ORDER_ID")]
    #[doc = ""]
    FilterDfpOrderId,
    #[serde(rename = "FILTER_TRUEVIEW_AGE")]
    #[doc = ""]
    FilterTrueviewAge,
    #[serde(rename = "FILTER_TRUEVIEW_GENDER")]
    #[doc = ""]
    FilterTrueviewGender,
    #[serde(rename = "FILTER_TRUEVIEW_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_INTEREST")]
    #[doc = ""]
    FilterTrueviewInterest,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupId,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_AD_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupAdId,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_LANGUAGE")]
    #[doc = ""]
    FilterTrueviewIarLanguage,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_GENDER")]
    #[doc = ""]
    FilterTrueviewIarGender,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_AGE")]
    #[doc = ""]
    FilterTrueviewIarAge,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CATEGORY")]
    #[doc = ""]
    FilterTrueviewIarCategory,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY")]
    #[doc = ""]
    FilterTrueviewIarCountry,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CITY")]
    #[doc = ""]
    FilterTrueviewIarCity,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION")]
    #[doc = ""]
    FilterTrueviewIarRegion,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewIarZipcode,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewIarRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_INTEREST")]
    #[doc = ""]
    FilterTrueviewIarInterest,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewIarParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_TIME_OF_DAY")]
    #[doc = ""]
    FilterTrueviewIarTimeOfDay,
    #[serde(rename = "FILTER_TRUEVIEW_CUSTOM_AFFINITY")]
    #[doc = ""]
    FilterTrueviewCustomAffinity,
    #[serde(rename = "FILTER_TRUEVIEW_CATEGORY")]
    #[doc = ""]
    FilterTrueviewCategory,
    #[serde(rename = "FILTER_TRUEVIEW_KEYWORD")]
    #[doc = ""]
    FilterTrueviewKeyword,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT")]
    #[doc = ""]
    FilterTrueviewPlacement,
    #[serde(rename = "FILTER_TRUEVIEW_URL")]
    #[doc = ""]
    FilterTrueviewUrl,
    #[serde(rename = "FILTER_TRUEVIEW_COUNTRY")]
    #[doc = ""]
    FilterTrueviewCountry,
    #[serde(rename = "FILTER_TRUEVIEW_REGION")]
    #[doc = ""]
    FilterTrueviewRegion,
    #[serde(rename = "FILTER_TRUEVIEW_CITY")]
    #[doc = ""]
    FilterTrueviewCity,
    #[serde(rename = "FILTER_TRUEVIEW_DMA")]
    #[doc = ""]
    FilterTrueviewDma,
    #[serde(rename = "FILTER_TRUEVIEW_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewZipcode,
    #[serde(rename = "FILTER_NOT_SUPPORTED")]
    #[doc = ""]
    FilterNotSupported,
    #[serde(rename = "FILTER_MEDIA_PLAN")]
    #[doc = ""]
    FilterMediaPlan,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterTrueviewIarYoutubeChannel,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterTrueviewIarYoutubeVideo,
    #[serde(rename = "FILTER_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterSkippableSupport,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_ID")]
    #[doc = ""]
    FilterCompanionCreativeId,
    #[serde(rename = "FILTER_BUDGET_SEGMENT_DESCRIPTION")]
    #[doc = ""]
    FilterBudgetSegmentDescription,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY_ID")]
    #[doc = ""]
    FilterFloodlightActivityId,
    #[serde(rename = "FILTER_DEVICE_MODEL")]
    #[doc = ""]
    FilterDeviceModel,
    #[serde(rename = "FILTER_DEVICE_MAKE")]
    #[doc = ""]
    FilterDeviceMake,
    #[serde(rename = "FILTER_DEVICE_TYPE")]
    #[doc = ""]
    FilterDeviceType,
    #[serde(rename = "FILTER_CREATIVE_ATTRIBUTE")]
    #[doc = ""]
    FilterCreativeAttribute,
    #[serde(rename = "FILTER_INVENTORY_COMMITMENT_TYPE")]
    #[doc = ""]
    FilterInventoryCommitmentType,
    #[serde(rename = "FILTER_INVENTORY_RATE_TYPE")]
    #[doc = ""]
    FilterInventoryRateType,
    #[serde(rename = "FILTER_INVENTORY_DELIVERY_METHOD")]
    #[doc = ""]
    FilterInventoryDeliveryMethod,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_EXTERNAL_ID")]
    #[doc = ""]
    FilterInventorySourceExternalId,
    #[serde(rename = "FILTER_AUTHORIZED_SELLER_STATE")]
    #[doc = ""]
    FilterAuthorizedSellerState,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS_RANGE")]
    #[doc = ""]
    FilterVideoDurationSecondsRange,
    #[serde(rename = "FILTER_PARTNER_NAME")]
    #[doc = ""]
    FilterPartnerName,
    #[serde(rename = "FILTER_PARTNER_STATUS")]
    #[doc = ""]
    FilterPartnerStatus,
    #[serde(rename = "FILTER_ADVERTISER_NAME")]
    #[doc = ""]
    FilterAdvertiserName,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterAdvertiserIntegrationCode,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_STATUS")]
    #[doc = ""]
    FilterAdvertiserIntegrationStatus,
    #[serde(rename = "FILTER_CARRIER_NAME")]
    #[doc = ""]
    FilterCarrierName,
    #[serde(rename = "FILTER_CHANNEL_NAME")]
    #[doc = ""]
    FilterChannelName,
    #[serde(rename = "FILTER_CITY_NAME")]
    #[doc = ""]
    FilterCityName,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_NAME")]
    #[doc = ""]
    FilterCompanionCreativeName,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY_NAME")]
    #[doc = ""]
    FilterUserListFirstPartyName,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY_NAME")]
    #[doc = ""]
    FilterUserListThirdPartyName,
    #[serde(rename = "FILTER_NIELSEN_RESTATEMENT_DATE")]
    #[doc = ""]
    FilterNielsenRestatementDate,
    #[serde(rename = "FILTER_NIELSEN_DATE_RANGE")]
    #[doc = ""]
    FilterNielsenDateRange,
    #[serde(rename = "FILTER_INSERTION_ORDER_NAME")]
    #[doc = ""]
    FilterInsertionOrderName,
    #[serde(rename = "FILTER_REGION_NAME")]
    #[doc = ""]
    FilterRegionName,
    #[serde(rename = "FILTER_DMA_NAME")]
    #[doc = ""]
    FilterDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewIarRegionName,
    #[serde(rename = "FILTER_TRUEVIEW_DMA_NAME")]
    #[doc = ""]
    FilterTrueviewDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewRegionName,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID")]
    #[doc = ""]
    FilterActiveViewCustomMetricId,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME")]
    #[doc = ""]
    FilterActiveViewCustomMetricName,
    #[serde(rename = "FILTER_AD_TYPE")]
    #[doc = ""]
    FilterAdType,
    #[serde(rename = "FILTER_ALGORITHM")]
    #[doc = ""]
    FilterAlgorithm,
    #[serde(rename = "FILTER_ALGORITHM_ID")]
    #[doc = ""]
    FilterAlgorithmId,
    #[serde(rename = "FILTER_AMP_PAGE_REQUEST")]
    #[doc = ""]
    FilterAmpPageRequest,
    #[serde(rename = "FILTER_ANONYMOUS_INVENTORY_MODELING")]
    #[doc = ""]
    FilterAnonymousInventoryModeling,
    #[serde(rename = "FILTER_APP_URL")]
    #[doc = ""]
    FilterAppUrl,
    #[serde(rename = "FILTER_APP_URL_EXCLUDED")]
    #[doc = ""]
    FilterAppUrlExcluded,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST")]
    #[doc = ""]
    FilterAttributedUserlist,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_COST")]
    #[doc = ""]
    FilterAttributedUserlistCost,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_TYPE")]
    #[doc = ""]
    FilterAttributedUserlistType,
    #[serde(rename = "FILTER_ATTRIBUTION_MODEL")]
    #[doc = ""]
    FilterAttributionModel,
    #[serde(rename = "FILTER_AUDIENCE_LIST")]
    #[doc = ""]
    FilterAudienceList,
    #[serde(rename = "FILTER_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterAudienceListCost,
    #[serde(rename = "FILTER_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterAudienceListType,
    #[serde(rename = "FILTER_AUDIENCE_NAME")]
    #[doc = ""]
    FilterAudienceName,
    #[serde(rename = "FILTER_AUDIENCE_TYPE")]
    #[doc = ""]
    FilterAudienceType,
    #[serde(rename = "FILTER_BILLABLE_OUTCOME")]
    #[doc = ""]
    FilterBillableOutcome,
    #[serde(rename = "FILTER_BRAND_LIFT_TYPE")]
    #[doc = ""]
    FilterBrandLiftType,
    #[serde(rename = "FILTER_CHANNEL_TYPE")]
    #[doc = ""]
    FilterChannelType,
    #[serde(rename = "FILTER_CM_PLACEMENT_ID")]
    #[doc = ""]
    FilterCmPlacementId,
    #[serde(rename = "FILTER_CONVERSION_SOURCE")]
    #[doc = ""]
    FilterConversionSource,
    #[serde(rename = "FILTER_CONVERSION_SOURCE_ID")]
    #[doc = ""]
    FilterConversionSourceId,
    #[serde(rename = "FILTER_COUNTRY_ID")]
    #[doc = ""]
    FilterCountryId,
    #[serde(rename = "FILTER_CREATIVE")]
    #[doc = ""]
    FilterCreative,
    #[serde(rename = "FILTER_CREATIVE_ASSET")]
    #[doc = ""]
    FilterCreativeAsset,
    #[serde(rename = "FILTER_CREATIVE_INTEGRATION_CODE")]
    #[doc = ""]
    FilterCreativeIntegrationCode,
    #[serde(rename = "FILTER_CREATIVE_RENDERED_IN_AMP")]
    #[doc = ""]
    FilterCreativeRenderedInAmp,
    #[serde(rename = "FILTER_CREATIVE_SOURCE")]
    #[doc = ""]
    FilterCreativeSource,
    #[serde(rename = "FILTER_CREATIVE_STATUS")]
    #[doc = ""]
    FilterCreativeStatus,
    #[serde(rename = "FILTER_DATA_PROVIDER_NAME")]
    #[doc = ""]
    FilterDataProviderName,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterDetailedDemographics,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterDetailedDemographicsId,
    #[serde(rename = "FILTER_DEVICE")]
    #[doc = ""]
    FilterDevice,
    #[serde(rename = "FILTER_GAM_INSERTION_ORDER")]
    #[doc = ""]
    FilterGamInsertionOrder,
    #[serde(rename = "FILTER_GAM_LINE_ITEM")]
    #[doc = ""]
    FilterGamLineItem,
    #[serde(rename = "FILTER_GAM_LINE_ITEM_ID")]
    #[doc = ""]
    FilterGamLineItemId,
    #[serde(rename = "FILTER_DIGITAL_CONTENT_LABEL")]
    #[doc = ""]
    FilterDigitalContentLabel,
    #[serde(rename = "FILTER_DOMAIN")]
    #[doc = ""]
    FilterDomain,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST")]
    #[doc = ""]
    FilterEligibleCookiesOnFirstPartyAudienceList,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST")]
    #[doc = ""]
    FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
    #[serde(rename = "FILTER_EXCHANGE")]
    #[doc = ""]
    FilterExchange,
    #[serde(rename = "FILTER_EXCHANGE_CODE")]
    #[doc = ""]
    FilterExchangeCode,
    #[serde(rename = "FILTER_EXTENSION")]
    #[doc = ""]
    FilterExtension,
    #[serde(rename = "FILTER_EXTENSION_STATUS")]
    #[doc = ""]
    FilterExtensionStatus,
    #[serde(rename = "FILTER_EXTENSION_TYPE")]
    #[doc = ""]
    FilterExtensionType,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterFirstPartyAudienceListCost,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterFirstPartyAudienceListType,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY")]
    #[doc = ""]
    FilterFloodlightActivity,
    #[serde(rename = "FILTER_FORMAT")]
    #[doc = ""]
    FilterFormat,
    #[serde(rename = "FILTER_GMAIL_AGE")]
    #[doc = ""]
    FilterGmailAge,
    #[serde(rename = "FILTER_GMAIL_CITY")]
    #[doc = ""]
    FilterGmailCity,
    #[serde(rename = "FILTER_GMAIL_COUNTRY")]
    #[doc = ""]
    FilterGmailCountry,
    #[serde(rename = "FILTER_GMAIL_COUNTRY_NAME")]
    #[doc = ""]
    FilterGmailCountryName,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE")]
    #[doc = ""]
    FilterGmailDeviceType,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE_NAME")]
    #[doc = ""]
    FilterGmailDeviceTypeName,
    #[serde(rename = "FILTER_GMAIL_GENDER")]
    #[doc = ""]
    FilterGmailGender,
    #[serde(rename = "FILTER_GMAIL_REGION")]
    #[doc = ""]
    FilterGmailRegion,
    #[serde(rename = "FILTER_GMAIL_REMARKETING_LIST")]
    #[doc = ""]
    FilterGmailRemarketingList,
    #[serde(rename = "FILTER_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterHouseholdIncome,
    #[serde(rename = "FILTER_IMPRESSION_COUNTING_METHOD")]
    #[doc = ""]
    FilterImpressionCountingMethod,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedInsertionOrder,
    #[serde(rename = "FILTER_INSERTION_ORDER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterInsertionOrderIntegrationCode,
    #[serde(rename = "FILTER_INSERTION_ORDER_STATUS")]
    #[doc = ""]
    FilterInsertionOrderStatus,
    #[serde(rename = "FILTER_INTEREST")]
    #[doc = ""]
    FilterInterest,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP")]
    #[doc = ""]
    FilterInventorySourceGroup,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP_ID")]
    #[doc = ""]
    FilterInventorySourceGroupId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_ID")]
    #[doc = ""]
    FilterInventorySourceId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_NAME")]
    #[doc = ""]
    FilterInventorySourceName,
    #[serde(rename = "FILTER_LIFE_EVENT")]
    #[doc = ""]
    FilterLifeEvent,
    #[serde(rename = "FILTER_LIFE_EVENTS")]
    #[doc = ""]
    FilterLifeEvents,
    #[serde(rename = "FILTER_LINE_ITEM_INTEGRATION_CODE")]
    #[doc = ""]
    FilterLineItemIntegrationCode,
    #[serde(rename = "FILTER_LINE_ITEM_NAME")]
    #[doc = ""]
    FilterLineItemName,
    #[serde(rename = "FILTER_LINE_ITEM_STATUS")]
    #[doc = ""]
    FilterLineItemStatus,
    #[serde(rename = "FILTER_MATCH_RATIO")]
    #[doc = ""]
    FilterMatchRatio,
    #[serde(rename = "FILTER_MEASUREMENT_SOURCE")]
    #[doc = ""]
    FilterMeasurementSource,
    #[serde(rename = "FILTER_MEDIA_PLAN_NAME")]
    #[doc = ""]
    FilterMediaPlanName,
    #[serde(rename = "FILTER_PARENTAL_STATUS")]
    #[doc = ""]
    FilterParentalStatus,
    #[serde(rename = "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementAllYoutubeChannels,
    #[serde(rename = "FILTER_PLATFORM")]
    #[doc = ""]
    FilterPlatform,
    #[serde(rename = "FILTER_PLAYBACK_METHOD")]
    #[doc = ""]
    FilterPlaybackMethod,
    #[serde(rename = "FILTER_POSITION_IN_CONTENT")]
    #[doc = ""]
    FilterPositionInContent,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY")]
    #[doc = ""]
    FilterPublisherProperty,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_ID")]
    #[doc = ""]
    FilterPublisherPropertyId,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION")]
    #[doc = ""]
    FilterPublisherPropertySection,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION_ID")]
    #[doc = ""]
    FilterPublisherPropertySectionId,
    #[serde(rename = "FILTER_REFUND_REASON")]
    #[doc = ""]
    FilterRefundReason,
    #[serde(rename = "FILTER_REMARKETING_LIST")]
    #[doc = ""]
    FilterRemarketingList,
    #[serde(rename = "FILTER_REWARDED")]
    #[doc = ""]
    FilterRewarded,
    #[serde(rename = "FILTER_SENSITIVE_CATEGORY")]
    #[doc = ""]
    FilterSensitiveCategory,
    #[serde(rename = "FILTER_SERVED_PIXEL_DENSITY")]
    #[doc = ""]
    FilterServedPixelDensity,
    #[serde(rename = "FILTER_TARGETED_DATA_PROVIDERS")]
    #[doc = ""]
    FilterTargetedDataProviders,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterThirdPartyAudienceListCost,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterThirdPartyAudienceListType,
    #[serde(rename = "FILTER_TRUEVIEW_AD")]
    #[doc = ""]
    FilterTrueviewAd,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP")]
    #[doc = ""]
    FilterTrueviewAdGroup,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterTrueviewDetailedDemographics,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterTrueviewDetailedDemographicsId,
    #[serde(rename = "FILTER_TRUEVIEW_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterTrueviewHouseholdIncome,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY_NAME")]
    #[doc = ""]
    FilterTrueviewIarCountryName,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST_NAME")]
    #[doc = ""]
    FilterTrueviewRemarketingListName,
    #[serde(rename = "FILTER_VARIANT_ID")]
    #[doc = ""]
    FilterVariantId,
    #[serde(rename = "FILTER_VARIANT_NAME")]
    #[doc = ""]
    FilterVariantName,
    #[serde(rename = "FILTER_VARIANT_VERSION")]
    #[doc = ""]
    FilterVariantVersion,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVerificationVideoPlayerSize,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_POSITION")]
    #[doc = ""]
    FilterVerificationVideoPosition,
    #[serde(rename = "FILTER_VIDEO_COMPANION_CREATIVE_SIZE")]
    #[doc = ""]
    FilterVideoCompanionCreativeSize,
    #[serde(rename = "FILTER_VIDEO_CONTINUOUS_PLAY")]
    #[doc = ""]
    FilterVideoContinuousPlay,
    #[serde(rename = "FILTER_VIDEO_DURATION")]
    #[doc = ""]
    FilterVideoDuration,
    #[serde(rename = "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST")]
    #[doc = ""]
    FilterYoutubeAdaptedAudienceList,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO")]
    #[doc = ""]
    FilterYoutubeAdVideo,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO_ID")]
    #[doc = ""]
    FilterYoutubeAdVideoId,
    #[serde(rename = "FILTER_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterYoutubeChannel,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedAdvertiser,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedPartner,
    #[serde(rename = "FILTER_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterYoutubeVideo,
    #[serde(rename = "FILTER_ZIP_POSTAL_CODE")]
    #[doc = ""]
    FilterZipPostalCode,
    #[serde(rename = "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementNameAllYoutubeChannels,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT_ID")]
    #[doc = ""]
    FilterTrueviewPlacementId,
    #[serde(rename = "FILTER_PATH_PATTERN_ID")]
    #[doc = ""]
    FilterPathPatternId,
    #[serde(rename = "FILTER_PATH_EVENT_INDEX")]
    #[doc = ""]
    FilterPathEventIndex,
    #[serde(rename = "FILTER_EVENT_TYPE")]
    #[doc = ""]
    FilterEventType,
    #[serde(rename = "FILTER_CHANNEL_GROUPING")]
    #[doc = ""]
    FilterChannelGrouping,
    #[serde(rename = "FILTER_OM_SDK_AVAILABLE")]
    #[doc = ""]
    FilterOmSdkAvailable,
    #[serde(rename = "FILTER_DATA_SOURCE")]
    #[doc = ""]
    FilterDataSource,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ParametersMetricsEnum {
    #[serde(rename = "METRIC_UNKNOWN")]
    #[doc = ""]
    MetricUnknown,
    #[serde(rename = "METRIC_IMPRESSIONS")]
    #[doc = ""]
    MetricImpressions,
    #[serde(rename = "METRIC_CLICKS")]
    #[doc = ""]
    MetricClicks,
    #[serde(rename = "METRIC_LAST_IMPRESSIONS")]
    #[doc = ""]
    MetricLastImpressions,
    #[serde(rename = "METRIC_LAST_CLICKS")]
    #[doc = ""]
    MetricLastClicks,
    #[serde(rename = "METRIC_TOTAL_CONVERSIONS")]
    #[doc = ""]
    MetricTotalConversions,
    #[serde(rename = "METRIC_MEDIA_COST_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_USD")]
    #[doc = ""]
    MetricMediaCostUsd,
    #[serde(rename = "METRIC_MEDIA_COST_PARTNER")]
    #[doc = ""]
    MetricMediaCostPartner,
    #[serde(rename = "METRIC_DATA_COST_ADVERTISER")]
    #[doc = ""]
    MetricDataCostAdvertiser,
    #[serde(rename = "METRIC_DATA_COST_USD")]
    #[doc = ""]
    MetricDataCostUsd,
    #[serde(rename = "METRIC_DATA_COST_PARTNER")]
    #[doc = ""]
    MetricDataCostPartner,
    #[serde(rename = "METRIC_CPM_FEE1_ADVERTISER")]
    #[doc = ""]
    MetricCpmFee1Advertiser,
    #[serde(rename = "METRIC_CPM_FEE1_USD")]
    #[doc = ""]
    MetricCpmFee1Usd,
    #[serde(rename = "METRIC_CPM_FEE1_PARTNER")]
    #[doc = ""]
    MetricCpmFee1Partner,
    #[serde(rename = "METRIC_CPM_FEE2_ADVERTISER")]
    #[doc = ""]
    MetricCpmFee2Advertiser,
    #[serde(rename = "METRIC_CPM_FEE2_USD")]
    #[doc = ""]
    MetricCpmFee2Usd,
    #[serde(rename = "METRIC_CPM_FEE2_PARTNER")]
    #[doc = ""]
    MetricCpmFee2Partner,
    #[serde(rename = "METRIC_MEDIA_FEE1_ADVERTISER")]
    #[doc = ""]
    MetricMediaFee1Advertiser,
    #[serde(rename = "METRIC_MEDIA_FEE1_USD")]
    #[doc = ""]
    MetricMediaFee1Usd,
    #[serde(rename = "METRIC_MEDIA_FEE1_PARTNER")]
    #[doc = ""]
    MetricMediaFee1Partner,
    #[serde(rename = "METRIC_MEDIA_FEE2_ADVERTISER")]
    #[doc = ""]
    MetricMediaFee2Advertiser,
    #[serde(rename = "METRIC_MEDIA_FEE2_USD")]
    #[doc = ""]
    MetricMediaFee2Usd,
    #[serde(rename = "METRIC_MEDIA_FEE2_PARTNER")]
    #[doc = ""]
    MetricMediaFee2Partner,
    #[serde(rename = "METRIC_REVENUE_ADVERTISER")]
    #[doc = ""]
    MetricRevenueAdvertiser,
    #[serde(rename = "METRIC_REVENUE_USD")]
    #[doc = ""]
    MetricRevenueUsd,
    #[serde(rename = "METRIC_REVENUE_PARTNER")]
    #[doc = ""]
    MetricRevenuePartner,
    #[serde(rename = "METRIC_PROFIT_ADVERTISER")]
    #[doc = ""]
    MetricProfitAdvertiser,
    #[serde(rename = "METRIC_PROFIT_USD")]
    #[doc = ""]
    MetricProfitUsd,
    #[serde(rename = "METRIC_PROFIT_PARTNER")]
    #[doc = ""]
    MetricProfitPartner,
    #[serde(rename = "METRIC_PROFIT_MARGIN")]
    #[doc = ""]
    MetricProfitMargin,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_USD")]
    #[doc = ""]
    MetricTotalMediaCostUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostAdvertiser,
    #[serde(rename = "METRIC_BILLABLE_COST_USD")]
    #[doc = ""]
    MetricBillableCostUsd,
    #[serde(rename = "METRIC_BILLABLE_COST_PARTNER")]
    #[doc = ""]
    MetricBillableCostPartner,
    #[serde(rename = "METRIC_BILLABLE_COST_ADVERTISER")]
    #[doc = ""]
    MetricBillableCostAdvertiser,
    #[serde(rename = "METRIC_PLATFORM_FEE_USD")]
    #[doc = ""]
    MetricPlatformFeeUsd,
    #[serde(rename = "METRIC_PLATFORM_FEE_PARTNER")]
    #[doc = ""]
    MetricPlatformFeePartner,
    #[serde(rename = "METRIC_PLATFORM_FEE_ADVERTISER")]
    #[doc = ""]
    MetricPlatformFeeAdvertiser,
    #[serde(rename = "METRIC_VIDEO_COMPLETION_RATE")]
    #[doc = ""]
    MetricVideoCompletionRate,
    #[serde(rename = "METRIC_PROFIT_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricProfitEcpmAdvertiser,
    #[serde(rename = "METRIC_PROFIT_ECPM_USD")]
    #[doc = ""]
    MetricProfitEcpmUsd,
    #[serde(rename = "METRIC_PROFIT_ECPM_PARTNER")]
    #[doc = ""]
    MetricProfitEcpmPartner,
    #[serde(rename = "METRIC_REVENUE_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpmAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPM_USD")]
    #[doc = ""]
    MetricRevenueEcpmUsd,
    #[serde(rename = "METRIC_REVENUE_ECPM_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpmPartner,
    #[serde(rename = "METRIC_REVENUE_ECPC_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpcAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPC_USD")]
    #[doc = ""]
    MetricRevenueEcpcUsd,
    #[serde(rename = "METRIC_REVENUE_ECPC_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpcPartner,
    #[serde(rename = "METRIC_REVENUE_ECPA_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpaAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPA_USD")]
    #[doc = ""]
    MetricRevenueEcpaUsd,
    #[serde(rename = "METRIC_REVENUE_ECPA_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpaPartner,
    #[serde(rename = "METRIC_REVENUE_ECPAPV_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpapvAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPAPV_USD")]
    #[doc = ""]
    MetricRevenueEcpapvUsd,
    #[serde(rename = "METRIC_REVENUE_ECPAPV_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpapvPartner,
    #[serde(rename = "METRIC_REVENUE_ECPAPC_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpapcAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPAPC_USD")]
    #[doc = ""]
    MetricRevenueEcpapcUsd,
    #[serde(rename = "METRIC_REVENUE_ECPAPC_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpapcPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpmAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPM_USD")]
    #[doc = ""]
    MetricMediaCostEcpmUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPM_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpmPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPC_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpcAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPC_USD")]
    #[doc = ""]
    MetricMediaCostEcpcUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPC_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpcPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPA_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpaAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPA_USD")]
    #[doc = ""]
    MetricMediaCostEcpaUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPA_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpaPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPV_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpapvAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPV_USD")]
    #[doc = ""]
    MetricMediaCostEcpapvUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPV_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpapvPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPC_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpapcAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPC_USD")]
    #[doc = ""]
    MetricMediaCostEcpapcUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPAPC_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpapcPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpmAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPM_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpmUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpmPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpcAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPC_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpcUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpcPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpaAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPA_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpaUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpaPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpapvAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpapvUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpapvPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpapcAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpapcUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpapcPartner,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_PLAYS")]
    #[doc = ""]
    MetricRichMediaVideoPlays,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS")]
    #[doc = ""]
    MetricRichMediaVideoCompletions,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_PAUSES")]
    #[doc = ""]
    MetricRichMediaVideoPauses,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_MUTES")]
    #[doc = ""]
    MetricRichMediaVideoMutes,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS")]
    #[doc = ""]
    MetricRichMediaVideoMidpoints,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS")]
    #[doc = ""]
    MetricRichMediaVideoFullScreens,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES")]
    #[doc = ""]
    MetricRichMediaVideoFirstQuartileCompletes,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES")]
    #[doc = ""]
    MetricRichMediaVideoThirdQuartileCompletes,
    #[serde(rename = "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE")]
    #[doc = ""]
    MetricClickToPostClickConversionRate,
    #[serde(rename = "METRIC_IMPRESSIONS_TO_CONVERSION_RATE")]
    #[doc = ""]
    MetricImpressionsToConversionRate,
    #[serde(rename = "METRIC_CONVERSIONS_PER_MILLE")]
    #[doc = ""]
    MetricConversionsPerMille,
    #[serde(rename = "METRIC_CTR")]
    #[doc = ""]
    MetricCtr,
    #[serde(rename = "METRIC_BID_REQUESTS")]
    #[doc = ""]
    MetricBidRequests,
    #[serde(rename = "METRIC_UNIQUE_VISITORS_COOKIES")]
    #[doc = ""]
    MetricUniqueVisitorsCookies,
    #[serde(rename = "METRIC_REVENUE_ECPCV_ADVERTISER")]
    #[doc = ""]
    MetricRevenueEcpcvAdvertiser,
    #[serde(rename = "METRIC_REVENUE_ECPCV_USD")]
    #[doc = ""]
    MetricRevenueEcpcvUsd,
    #[serde(rename = "METRIC_REVENUE_ECPCV_PARTNER")]
    #[doc = ""]
    MetricRevenueEcpcvPartner,
    #[serde(rename = "METRIC_MEDIA_COST_ECPCV_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostEcpcvAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_ECPCV_USD")]
    #[doc = ""]
    MetricMediaCostEcpcvUsd,
    #[serde(rename = "METRIC_MEDIA_COST_ECPCV_PARTNER")]
    #[doc = ""]
    MetricMediaCostEcpcvPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostEcpcvAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPCV_USD")]
    #[doc = ""]
    MetricTotalMediaCostEcpcvUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostEcpcvPartner,
    #[serde(rename = "METRIC_RICH_MEDIA_VIDEO_SKIPS")]
    #[doc = ""]
    MetricRichMediaVideoSkips,
    #[serde(rename = "METRIC_FEE2_ADVERTISER")]
    #[doc = ""]
    MetricFee2Advertiser,
    #[serde(rename = "METRIC_FEE2_USD")]
    #[doc = ""]
    MetricFee2Usd,
    #[serde(rename = "METRIC_FEE2_PARTNER")]
    #[doc = ""]
    MetricFee2Partner,
    #[serde(rename = "METRIC_FEE3_ADVERTISER")]
    #[doc = ""]
    MetricFee3Advertiser,
    #[serde(rename = "METRIC_FEE3_USD")]
    #[doc = ""]
    MetricFee3Usd,
    #[serde(rename = "METRIC_FEE3_PARTNER")]
    #[doc = ""]
    MetricFee3Partner,
    #[serde(rename = "METRIC_FEE4_ADVERTISER")]
    #[doc = ""]
    MetricFee4Advertiser,
    #[serde(rename = "METRIC_FEE4_USD")]
    #[doc = ""]
    MetricFee4Usd,
    #[serde(rename = "METRIC_FEE4_PARTNER")]
    #[doc = ""]
    MetricFee4Partner,
    #[serde(rename = "METRIC_FEE5_ADVERTISER")]
    #[doc = ""]
    MetricFee5Advertiser,
    #[serde(rename = "METRIC_FEE5_USD")]
    #[doc = ""]
    MetricFee5Usd,
    #[serde(rename = "METRIC_FEE5_PARTNER")]
    #[doc = ""]
    MetricFee5Partner,
    #[serde(rename = "METRIC_FEE6_ADVERTISER")]
    #[doc = ""]
    MetricFee6Advertiser,
    #[serde(rename = "METRIC_FEE6_USD")]
    #[doc = ""]
    MetricFee6Usd,
    #[serde(rename = "METRIC_FEE6_PARTNER")]
    #[doc = ""]
    MetricFee6Partner,
    #[serde(rename = "METRIC_FEE7_ADVERTISER")]
    #[doc = ""]
    MetricFee7Advertiser,
    #[serde(rename = "METRIC_FEE7_USD")]
    #[doc = ""]
    MetricFee7Usd,
    #[serde(rename = "METRIC_FEE7_PARTNER")]
    #[doc = ""]
    MetricFee7Partner,
    #[serde(rename = "METRIC_FEE8_ADVERTISER")]
    #[doc = ""]
    MetricFee8Advertiser,
    #[serde(rename = "METRIC_FEE8_USD")]
    #[doc = ""]
    MetricFee8Usd,
    #[serde(rename = "METRIC_FEE8_PARTNER")]
    #[doc = ""]
    MetricFee8Partner,
    #[serde(rename = "METRIC_FEE9_ADVERTISER")]
    #[doc = ""]
    MetricFee9Advertiser,
    #[serde(rename = "METRIC_FEE9_USD")]
    #[doc = ""]
    MetricFee9Usd,
    #[serde(rename = "METRIC_FEE9_PARTNER")]
    #[doc = ""]
    MetricFee9Partner,
    #[serde(rename = "METRIC_FEE10_ADVERTISER")]
    #[doc = ""]
    MetricFee10Advertiser,
    #[serde(rename = "METRIC_FEE10_USD")]
    #[doc = ""]
    MetricFee10Usd,
    #[serde(rename = "METRIC_FEE10_PARTNER")]
    #[doc = ""]
    MetricFee10Partner,
    #[serde(rename = "METRIC_FEE11_ADVERTISER")]
    #[doc = ""]
    MetricFee11Advertiser,
    #[serde(rename = "METRIC_FEE11_USD")]
    #[doc = ""]
    MetricFee11Usd,
    #[serde(rename = "METRIC_FEE11_PARTNER")]
    #[doc = ""]
    MetricFee11Partner,
    #[serde(rename = "METRIC_FEE12_ADVERTISER")]
    #[doc = ""]
    MetricFee12Advertiser,
    #[serde(rename = "METRIC_FEE12_USD")]
    #[doc = ""]
    MetricFee12Usd,
    #[serde(rename = "METRIC_FEE12_PARTNER")]
    #[doc = ""]
    MetricFee12Partner,
    #[serde(rename = "METRIC_FEE13_ADVERTISER")]
    #[doc = ""]
    MetricFee13Advertiser,
    #[serde(rename = "METRIC_FEE13_USD")]
    #[doc = ""]
    MetricFee13Usd,
    #[serde(rename = "METRIC_FEE13_PARTNER")]
    #[doc = ""]
    MetricFee13Partner,
    #[serde(rename = "METRIC_FEE14_ADVERTISER")]
    #[doc = ""]
    MetricFee14Advertiser,
    #[serde(rename = "METRIC_FEE14_USD")]
    #[doc = ""]
    MetricFee14Usd,
    #[serde(rename = "METRIC_FEE14_PARTNER")]
    #[doc = ""]
    MetricFee14Partner,
    #[serde(rename = "METRIC_FEE15_ADVERTISER")]
    #[doc = ""]
    MetricFee15Advertiser,
    #[serde(rename = "METRIC_FEE15_USD")]
    #[doc = ""]
    MetricFee15Usd,
    #[serde(rename = "METRIC_FEE15_PARTNER")]
    #[doc = ""]
    MetricFee15Partner,
    #[serde(rename = "METRIC_CPM_FEE3_ADVERTISER")]
    #[doc = ""]
    MetricCpmFee3Advertiser,
    #[serde(rename = "METRIC_CPM_FEE3_USD")]
    #[doc = ""]
    MetricCpmFee3Usd,
    #[serde(rename = "METRIC_CPM_FEE3_PARTNER")]
    #[doc = ""]
    MetricCpmFee3Partner,
    #[serde(rename = "METRIC_CPM_FEE4_ADVERTISER")]
    #[doc = ""]
    MetricCpmFee4Advertiser,
    #[serde(rename = "METRIC_CPM_FEE4_USD")]
    #[doc = ""]
    MetricCpmFee4Usd,
    #[serde(rename = "METRIC_CPM_FEE4_PARTNER")]
    #[doc = ""]
    MetricCpmFee4Partner,
    #[serde(rename = "METRIC_CPM_FEE5_ADVERTISER")]
    #[doc = ""]
    MetricCpmFee5Advertiser,
    #[serde(rename = "METRIC_CPM_FEE5_USD")]
    #[doc = ""]
    MetricCpmFee5Usd,
    #[serde(rename = "METRIC_CPM_FEE5_PARTNER")]
    #[doc = ""]
    MetricCpmFee5Partner,
    #[serde(rename = "METRIC_MEDIA_FEE3_ADVERTISER")]
    #[doc = ""]
    MetricMediaFee3Advertiser,
    #[serde(rename = "METRIC_MEDIA_FEE3_USD")]
    #[doc = ""]
    MetricMediaFee3Usd,
    #[serde(rename = "METRIC_MEDIA_FEE3_PARTNER")]
    #[doc = ""]
    MetricMediaFee3Partner,
    #[serde(rename = "METRIC_MEDIA_FEE4_ADVERTISER")]
    #[doc = ""]
    MetricMediaFee4Advertiser,
    #[serde(rename = "METRIC_MEDIA_FEE4_USD")]
    #[doc = ""]
    MetricMediaFee4Usd,
    #[serde(rename = "METRIC_MEDIA_FEE4_PARTNER")]
    #[doc = ""]
    MetricMediaFee4Partner,
    #[serde(rename = "METRIC_MEDIA_FEE5_ADVERTISER")]
    #[doc = ""]
    MetricMediaFee5Advertiser,
    #[serde(rename = "METRIC_MEDIA_FEE5_USD")]
    #[doc = ""]
    MetricMediaFee5Usd,
    #[serde(rename = "METRIC_MEDIA_FEE5_PARTNER")]
    #[doc = ""]
    MetricMediaFee5Partner,
    #[serde(rename = "METRIC_VIDEO_COMPANION_IMPRESSIONS")]
    #[doc = ""]
    MetricVideoCompanionImpressions,
    #[serde(rename = "METRIC_VIDEO_COMPANION_CLICKS")]
    #[doc = ""]
    MetricVideoCompanionClicks,
    #[serde(rename = "METRIC_FEE16_ADVERTISER")]
    #[doc = ""]
    MetricFee16Advertiser,
    #[serde(rename = "METRIC_FEE16_USD")]
    #[doc = ""]
    MetricFee16Usd,
    #[serde(rename = "METRIC_FEE16_PARTNER")]
    #[doc = ""]
    MetricFee16Partner,
    #[serde(rename = "METRIC_FEE17_ADVERTISER")]
    #[doc = ""]
    MetricFee17Advertiser,
    #[serde(rename = "METRIC_FEE17_USD")]
    #[doc = ""]
    MetricFee17Usd,
    #[serde(rename = "METRIC_FEE17_PARTNER")]
    #[doc = ""]
    MetricFee17Partner,
    #[serde(rename = "METRIC_FEE18_ADVERTISER")]
    #[doc = ""]
    MetricFee18Advertiser,
    #[serde(rename = "METRIC_FEE18_USD")]
    #[doc = ""]
    MetricFee18Usd,
    #[serde(rename = "METRIC_FEE18_PARTNER")]
    #[doc = ""]
    MetricFee18Partner,
    #[serde(rename = "METRIC_TRUEVIEW_VIEWS")]
    #[doc = ""]
    MetricTrueviewViews,
    #[serde(rename = "METRIC_TRUEVIEW_UNIQUE_VIEWERS")]
    #[doc = ""]
    MetricTrueviewUniqueViewers,
    #[serde(rename = "METRIC_TRUEVIEW_EARNED_VIEWS")]
    #[doc = ""]
    MetricTrueviewEarnedViews,
    #[serde(rename = "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS")]
    #[doc = ""]
    MetricTrueviewEarnedSubscribers,
    #[serde(rename = "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS")]
    #[doc = ""]
    MetricTrueviewEarnedPlaylistAdditions,
    #[serde(rename = "METRIC_TRUEVIEW_EARNED_LIKES")]
    #[doc = ""]
    MetricTrueviewEarnedLikes,
    #[serde(rename = "METRIC_TRUEVIEW_EARNED_SHARES")]
    #[doc = ""]
    MetricTrueviewEarnedShares,
    #[serde(rename = "METRIC_TRUEVIEW_IMPRESSION_SHARE")]
    #[doc = ""]
    MetricTrueviewImpressionShare,
    #[serde(rename = "METRIC_TRUEVIEW_LOST_IS_BUDGET")]
    #[doc = ""]
    MetricTrueviewLostIsBudget,
    #[serde(rename = "METRIC_TRUEVIEW_LOST_IS_RANK")]
    #[doc = ""]
    MetricTrueviewLostIsRank,
    #[serde(rename = "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION")]
    #[doc = ""]
    MetricTrueviewViewThroughConversion,
    #[serde(rename = "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW")]
    #[doc = ""]
    MetricTrueviewConversionManyPerView,
    #[serde(rename = "METRIC_TRUEVIEW_VIEW_RATE")]
    #[doc = ""]
    MetricTrueviewViewRate,
    #[serde(rename = "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW")]
    #[doc = ""]
    MetricTrueviewConversionRateOnePerView,
    #[serde(rename = "METRIC_TRUEVIEW_CPV_ADVERTISER")]
    #[doc = ""]
    MetricTrueviewCpvAdvertiser,
    #[serde(rename = "METRIC_TRUEVIEW_CPV_USD")]
    #[doc = ""]
    MetricTrueviewCpvUsd,
    #[serde(rename = "METRIC_TRUEVIEW_CPV_PARTNER")]
    #[doc = ""]
    MetricTrueviewCpvPartner,
    #[serde(rename = "METRIC_FEE19_ADVERTISER")]
    #[doc = ""]
    MetricFee19Advertiser,
    #[serde(rename = "METRIC_FEE19_USD")]
    #[doc = ""]
    MetricFee19Usd,
    #[serde(rename = "METRIC_FEE19_PARTNER")]
    #[doc = ""]
    MetricFee19Partner,
    #[serde(rename = "METRIC_TEA_TRUEVIEW_IMPRESSIONS")]
    #[doc = ""]
    MetricTeaTrueviewImpressions,
    #[serde(rename = "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES")]
    #[doc = ""]
    MetricTeaTrueviewUniqueCookies,
    #[serde(rename = "METRIC_FEE20_ADVERTISER")]
    #[doc = ""]
    MetricFee20Advertiser,
    #[serde(rename = "METRIC_FEE20_USD")]
    #[doc = ""]
    MetricFee20Usd,
    #[serde(rename = "METRIC_FEE20_PARTNER")]
    #[doc = ""]
    MetricFee20Partner,
    #[serde(rename = "METRIC_FEE21_ADVERTISER")]
    #[doc = ""]
    MetricFee21Advertiser,
    #[serde(rename = "METRIC_FEE21_USD")]
    #[doc = ""]
    MetricFee21Usd,
    #[serde(rename = "METRIC_FEE21_PARTNER")]
    #[doc = ""]
    MetricFee21Partner,
    #[serde(rename = "METRIC_FEE22_ADVERTISER")]
    #[doc = ""]
    MetricFee22Advertiser,
    #[serde(rename = "METRIC_FEE22_USD")]
    #[doc = ""]
    MetricFee22Usd,
    #[serde(rename = "METRIC_FEE22_PARTNER")]
    #[doc = ""]
    MetricFee22Partner,
    #[serde(rename = "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER")]
    #[doc = ""]
    MetricTrueviewTotalConversionValuesAdvertiser,
    #[serde(rename = "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD")]
    #[doc = ""]
    MetricTrueviewTotalConversionValuesUsd,
    #[serde(rename = "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER")]
    #[doc = ""]
    MetricTrueviewTotalConversionValuesPartner,
    #[serde(rename = "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER")]
    #[doc = ""]
    MetricTrueviewConversionCostManyPerViewAdvertiser,
    #[serde(rename = "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD")]
    #[doc = ""]
    MetricTrueviewConversionCostManyPerViewUsd,
    #[serde(rename = "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER")]
    #[doc = ""]
    MetricTrueviewConversionCostManyPerViewPartner,
    #[serde(rename = "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricProfitViewableEcpmAdvertiser,
    #[serde(rename = "METRIC_PROFIT_VIEWABLE_ECPM_USD")]
    #[doc = ""]
    MetricProfitViewableEcpmUsd,
    #[serde(rename = "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER")]
    #[doc = ""]
    MetricProfitViewableEcpmPartner,
    #[serde(rename = "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricRevenueViewableEcpmAdvertiser,
    #[serde(rename = "METRIC_REVENUE_VIEWABLE_ECPM_USD")]
    #[doc = ""]
    MetricRevenueViewableEcpmUsd,
    #[serde(rename = "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER")]
    #[doc = ""]
    MetricRevenueViewableEcpmPartner,
    #[serde(rename = "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricMediaCostViewableEcpmAdvertiser,
    #[serde(rename = "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD")]
    #[doc = ""]
    MetricMediaCostViewableEcpmUsd,
    #[serde(rename = "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER")]
    #[doc = ""]
    MetricMediaCostViewableEcpmPartner,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER")]
    #[doc = ""]
    MetricTotalMediaCostViewableEcpmAdvertiser,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD")]
    #[doc = ""]
    MetricTotalMediaCostViewableEcpmUsd,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER")]
    #[doc = ""]
    MetricTotalMediaCostViewableEcpmPartner,
    #[serde(rename = "METRIC_TRUEVIEW_ENGAGEMENTS")]
    #[doc = ""]
    MetricTrueviewEngagements,
    #[serde(rename = "METRIC_TRUEVIEW_ENGAGEMENT_RATE")]
    #[doc = ""]
    MetricTrueviewEngagementRate,
    #[serde(rename = "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER")]
    #[doc = ""]
    MetricTrueviewAverageCpeAdvertiser,
    #[serde(rename = "METRIC_TRUEVIEW_AVERAGE_CPE_USD")]
    #[doc = ""]
    MetricTrueviewAverageCpeUsd,
    #[serde(rename = "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER")]
    #[doc = ""]
    MetricTrueviewAverageCpePartner,
    #[serde(rename = "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewViewableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewEligibleImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewMeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewPctMeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewPctViewableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME")]
    #[doc = ""]
    MetricActiveViewAverageViewableTime,
    #[serde(rename = "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewUnmeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewUnviewableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE")]
    #[doc = ""]
    MetricActiveViewDistributionUnmeasurable,
    #[serde(rename = "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE")]
    #[doc = ""]
    MetricActiveViewDistributionUnviewable,
    #[serde(rename = "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE")]
    #[doc = ""]
    MetricActiveViewDistributionViewable,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD")]
    #[doc = ""]
    MetricActiveViewPercentViewableForTimeThreshold,
    #[serde(rename = "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD")]
    #[doc = ""]
    MetricActiveViewViewableForTimeThreshold,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START")]
    #[doc = ""]
    MetricActiveViewPercentVisibleAtStart,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentVisibleFirstQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentVisibleSecondQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentVisibleThirdQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE")]
    #[doc = ""]
    MetricActiveViewPercentVisibleOnComplete,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START")]
    #[doc = ""]
    MetricActiveViewPercentAudibleVisibleAtStart,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentAudibleVisibleFirstQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentAudibleVisibleSecondQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR")]
    #[doc = ""]
    MetricActiveViewPercentAudibleVisibleThirdQuar,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE")]
    #[doc = ""]
    MetricActiveViewPercentAudibleVisibleOnComplete,
    #[serde(rename = "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewAudibleVisibleOnCompleteImpressions,
    #[serde(rename = "METRIC_VIEWABLE_BID_REQUESTS")]
    #[doc = ""]
    MetricViewableBidRequests,
    #[serde(rename = "METRIC_COOKIE_REACH_IMPRESSION_REACH")]
    #[doc = ""]
    MetricCookieReachImpressionReach,
    #[serde(rename = "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY")]
    #[doc = ""]
    MetricCookieReachAverageImpressionFrequency,
    #[serde(rename = "METRIC_DBM_ENGAGEMENT_RATE")]
    #[doc = ""]
    MetricDbmEngagementRate,
    #[serde(rename = "METRIC_RICH_MEDIA_SCROLLS")]
    #[doc = ""]
    MetricRichMediaScrolls,
    #[serde(rename = "METRIC_CM_POST_VIEW_REVENUE")]
    #[doc = ""]
    MetricCmPostViewRevenue,
    #[serde(rename = "METRIC_CM_POST_CLICK_REVENUE")]
    #[doc = ""]
    MetricCmPostClickRevenue,
    #[serde(rename = "METRIC_FLOODLIGHT_IMPRESSIONS")]
    #[doc = ""]
    MetricFloodlightImpressions,
    #[serde(rename = "METRIC_BILLABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricBillableImpressions,
    #[serde(rename = "METRIC_NIELSEN_AVERAGE_FREQUENCY")]
    #[doc = ""]
    MetricNielsenAverageFrequency,
    #[serde(rename = "METRIC_NIELSEN_IMPRESSIONS")]
    #[doc = ""]
    MetricNielsenImpressions,
    #[serde(rename = "METRIC_NIELSEN_UNIQUE_AUDIENCE")]
    #[doc = ""]
    MetricNielsenUniqueAudience,
    #[serde(rename = "METRIC_NIELSEN_GRP")]
    #[doc = ""]
    MetricNielsenGrp,
    #[serde(rename = "METRIC_NIELSEN_IMPRESSION_INDEX")]
    #[doc = ""]
    MetricNielsenImpressionIndex,
    #[serde(rename = "METRIC_NIELSEN_IMPRESSIONS_SHARE")]
    #[doc = ""]
    MetricNielsenImpressionsShare,
    #[serde(rename = "METRIC_NIELSEN_POPULATION")]
    #[doc = ""]
    MetricNielsenPopulation,
    #[serde(rename = "METRIC_NIELSEN_POPULATION_REACH")]
    #[doc = ""]
    MetricNielsenPopulationReach,
    #[serde(rename = "METRIC_NIELSEN_POPULATION_SHARE")]
    #[doc = ""]
    MetricNielsenPopulationShare,
    #[serde(rename = "METRIC_NIELSEN_REACH_INDEX")]
    #[doc = ""]
    MetricNielsenReachIndex,
    #[serde(rename = "METRIC_NIELSEN_REACH_SHARE")]
    #[doc = ""]
    MetricNielsenReachShare,
    #[serde(rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationImpressions,
    #[serde(
        rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_MEASURABLE_IMPRESSIONS"
    )]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationMeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_RATE")]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationRate,
    #[serde(
        rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_IMPRESSIONS"
    )]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewImpressions,
    #[serde(
        rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_MEASURABLE_IMPRESSIONS"
    )]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewMeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_RATE")]
    #[doc = ""]
    MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewRate,
    #[serde(rename = "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_MEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewCustomMetricMeasurableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewCustomMetricViewableImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_RATE")]
    #[doc = ""]
    MetricActiveViewCustomMetricViewableRate,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_IMPRESSIONS")]
    #[doc = ""]
    MetricActiveViewPercentAudibleImpressions,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_FULLY_ON_SCREEN_2_SEC")]
    #[doc = ""]
    MetricActiveViewPercentFullyOnScreen2Sec,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_FULL_SCREEN")]
    #[doc = ""]
    MetricActiveViewPercentFullScreen,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_IN_BACKGROUND")]
    #[doc = ""]
    MetricActiveViewPercentInBackground,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_AD_PLAYED")]
    #[doc = ""]
    MetricActiveViewPercentOfAdPlayed,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_AUDIBLE_AND_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfCompletedImpressionsAudibleAndVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfCompletedImpressionsVisible,
    #[serde(
        rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE"
    )]
    #[doc = ""]
    MetricActiveViewPercentOfFirstQuartileImpressionsAudibleAndVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfFirstQuartileImpressionsVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_AUDIBLE_AND_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfMidpointImpressionsAudibleAndVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfMidpointImpressionsVisible,
    #[serde(
        rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE"
    )]
    #[doc = ""]
    MetricActiveViewPercentOfThirdQuartileImpressionsAudibleAndVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentOfThirdQuartileImpressionsVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE")]
    #[doc = ""]
    MetricActiveViewPercentPlayTimeAudible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE_AND_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentPlayTimeAudibleAndVisible,
    #[serde(rename = "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_VISIBLE")]
    #[doc = ""]
    MetricActiveViewPercentPlayTimeVisible,
    #[serde(rename = "METRIC_ADAPTED_AUDIENCE_FREQUENCY")]
    #[doc = ""]
    MetricAdaptedAudienceFrequency,
    #[serde(rename = "METRIC_ADLINGO_FEE_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricAdlingoFeeAdvertiserCurrency,
    #[serde(rename = "METRIC_AUDIO_CLIENT_COST_ECPCL_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricAudioClientCostEcpclAdvertiserCurrency,
    #[serde(rename = "METRIC_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricAudioMediaCostEcpclAdvertiserCurrency,
    #[serde(rename = "METRIC_AUDIO_MUTES_AUDIO")]
    #[doc = ""]
    MetricAudioMutesAudio,
    #[serde(rename = "METRIC_AUDIO_REVENUE_ECPCL_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricAudioRevenueEcpclAdvertiserCurrency,
    #[serde(rename = "METRIC_AUDIO_UNMUTES_AUDIO")]
    #[doc = ""]
    MetricAudioUnmutesAudio,
    #[serde(rename = "METRIC_AUDIO_UNMUTES_VIDEO")]
    #[doc = ""]
    MetricAudioUnmutesVideo,
    #[serde(rename = "METRIC_AVERAGE_DISPLAY_TIME")]
    #[doc = ""]
    MetricAverageDisplayTime,
    #[serde(rename = "METRIC_AVERAGE_IMPRESSION_FREQUENCY_PER_USER")]
    #[doc = ""]
    MetricAverageImpressionFrequencyPerUser,
    #[serde(rename = "METRIC_AVERAGE_INTERACTION_TIME")]
    #[doc = ""]
    MetricAverageInteractionTime,
    #[serde(rename = "METRIC_AVERAGE_WATCH_TIME_PER_IMPRESSION")]
    #[doc = ""]
    MetricAverageWatchTimePerImpression,
    #[serde(rename = "METRIC_BEGIN_TO_RENDER_ELIGIBLE_IMPRESSIONS")]
    #[doc = ""]
    MetricBeginToRenderEligibleImpressions,
    #[serde(rename = "METRIC_BEGIN_TO_RENDER_IMPRESSIONS")]
    #[doc = ""]
    MetricBeginToRenderImpressions,
    #[serde(rename = "METRIC_BENCHMARK_FREQUENCY")]
    #[doc = ""]
    MetricBenchmarkFrequency,
    #[serde(rename = "METRIC_BRAND_LIFT_ABSOLUTE_BRAND_LIFT")]
    #[doc = ""]
    MetricBrandLiftAbsoluteBrandLift,
    #[serde(rename = "METRIC_BRAND_LIFT_ALL_SURVEY_RESPONSES")]
    #[doc = ""]
    MetricBrandLiftAllSurveyResponses,
    #[serde(rename = "METRIC_BRAND_LIFT_BASELINE_POSITIVE_RESPONSE_RATE")]
    #[doc = ""]
    MetricBrandLiftBaselinePositiveResponseRate,
    #[serde(rename = "METRIC_BRAND_LIFT_BASELINE_SURVEY_RESPONSES")]
    #[doc = ""]
    MetricBrandLiftBaselineSurveyResponses,
    #[serde(rename = "METRIC_BRAND_LIFT_COST_PER_LIFTED_USER")]
    #[doc = ""]
    MetricBrandLiftCostPerLiftedUser,
    #[serde(rename = "METRIC_BRAND_LIFT_EXPOSED_SURVEY_RESPONSES")]
    #[doc = ""]
    MetricBrandLiftExposedSurveyResponses,
    #[serde(rename = "METRIC_BRAND_LIFT_HEADROOM_BRAND_LIFT")]
    #[doc = ""]
    MetricBrandLiftHeadroomBrandLift,
    #[serde(rename = "METRIC_BRAND_LIFT_RELATIVE_BRAND_LIFT")]
    #[doc = ""]
    MetricBrandLiftRelativeBrandLift,
    #[serde(rename = "METRIC_BRAND_LIFT_USERS")]
    #[doc = ""]
    MetricBrandLiftUsers,
    #[serde(rename = "METRIC_CARD_CLICKS")]
    #[doc = ""]
    MetricCardClicks,
    #[serde(rename = "METRIC_CLIENT_COST_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_ECPA_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostEcpaAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_ECPA_PC_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostEcpaPcAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_ECPA_PV_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostEcpaPvAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_ECPC_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostEcpcAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_ECPM_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostEcpmAdvertiserCurrency,
    #[serde(rename = "METRIC_CLIENT_COST_VIEWABLE_ECPM_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricClientCostViewableEcpmAdvertiserCurrency,
    #[serde(rename = "METRIC_CM_POST_CLICK_REVENUE_CROSS_ENVIRONMENT")]
    #[doc = ""]
    MetricCmPostClickRevenueCrossEnvironment,
    #[serde(rename = "METRIC_CM_POST_VIEW_REVENUE_CROSS_ENVIRONMENT")]
    #[doc = ""]
    MetricCmPostViewRevenueCrossEnvironment,
    #[serde(rename = "METRIC_COMPANION_CLICKS_AUDIO")]
    #[doc = ""]
    MetricCompanionClicksAudio,
    #[serde(rename = "METRIC_COMPANION_IMPRESSIONS_AUDIO")]
    #[doc = ""]
    MetricCompanionImpressionsAudio,
    #[serde(rename = "METRIC_COMPLETE_LISTENS_AUDIO")]
    #[doc = ""]
    MetricCompleteListensAudio,
    #[serde(rename = "METRIC_COMPLETION_RATE_AUDIO")]
    #[doc = ""]
    MetricCompletionRateAudio,
    #[serde(rename = "METRIC_COUNTERS")]
    #[doc = ""]
    MetricCounters,
    #[serde(rename = "METRIC_CUSTOM_FEE_1_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricCustomFee1AdvertiserCurrency,
    #[serde(rename = "METRIC_CUSTOM_FEE_2_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricCustomFee2AdvertiserCurrency,
    #[serde(rename = "METRIC_CUSTOM_FEE_3_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricCustomFee3AdvertiserCurrency,
    #[serde(rename = "METRIC_CUSTOM_FEE_4_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricCustomFee4AdvertiserCurrency,
    #[serde(rename = "METRIC_CUSTOM_FEE_5_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricCustomFee5AdvertiserCurrency,
    #[serde(rename = "METRIC_CUSTOM_VALUE_PER_1000_IMPRESSIONS")]
    #[doc = ""]
    MetricCustomValuePer1000Impressions,
    #[serde(rename = "METRIC_ENGAGEMENTS")]
    #[doc = ""]
    MetricEngagements,
    #[serde(rename = "METRIC_ESTIMATED_CPM_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricEstimatedCpmForImpressionsWithCustomValueAdvertiserCurrency,
    #[serde(
        rename = "METRIC_ESTIMATED_TOTAL_COST_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY"
    )]
    #[doc = ""]
    MetricEstimatedTotalCostForImpressionsWithCustomValueAdvertiserCurrency,
    #[serde(rename = "METRIC_EXITS")]
    #[doc = ""]
    MetricExits,
    #[serde(rename = "METRIC_EXPANSIONS")]
    #[doc = ""]
    MetricExpansions,
    #[serde(rename = "METRIC_FIRST_QUARTILE_AUDIO")]
    #[doc = ""]
    MetricFirstQuartileAudio,
    #[serde(rename = "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_IMPRESSIONS")]
    #[doc = ""]
    MetricGeneralInvalidTrafficGivtImpressions,
    #[serde(rename = "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_TRACKED_ADS")]
    #[doc = ""]
    MetricGeneralInvalidTrafficGivtTrackedAds,
    #[serde(rename = "METRIC_GIVT_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS")]
    #[doc = ""]
    MetricGivtActiveViewEligibleImpressions,
    #[serde(rename = "METRIC_GIVT_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricGivtActiveViewMeasurableImpressions,
    #[serde(rename = "METRIC_GIVT_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricGivtActiveViewViewableImpressions,
    #[serde(rename = "METRIC_GIVT_BEGIN_TO_RENDER_IMPRESSIONS")]
    #[doc = ""]
    MetricGivtBeginToRenderImpressions,
    #[serde(rename = "METRIC_GIVT_CLICKS")]
    #[doc = ""]
    MetricGivtClicks,
    #[serde(rename = "METRIC_GMAIL_CONVERSIONS")]
    #[doc = ""]
    MetricGmailConversions,
    #[serde(rename = "METRIC_GMAIL_POST_CLICK_CONVERSIONS")]
    #[doc = ""]
    MetricGmailPostClickConversions,
    #[serde(rename = "METRIC_GMAIL_POST_VIEW_CONVERSIONS")]
    #[doc = ""]
    MetricGmailPostViewConversions,
    #[serde(rename = "METRIC_GMAIL_POTENTIAL_VIEWS")]
    #[doc = ""]
    MetricGmailPotentialViews,
    #[serde(rename = "METRIC_IMPRESSIONS_WITH_CUSTOM_VALUE")]
    #[doc = ""]
    MetricImpressionsWithCustomValue,
    #[serde(rename = "METRIC_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE")]
    #[doc = ""]
    MetricImpressionsWithPositiveCustomValue,
    #[serde(rename = "METRIC_IMPRESSION_CUSTOM_VALUE_COST")]
    #[doc = ""]
    MetricImpressionCustomValueCost,
    #[serde(rename = "METRIC_INTERACTIVE_IMPRESSIONS")]
    #[doc = ""]
    MetricInteractiveImpressions,
    #[serde(rename = "METRIC_INVALID_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS")]
    #[doc = ""]
    MetricInvalidActiveViewEligibleImpressions,
    #[serde(rename = "METRIC_INVALID_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricInvalidActiveViewMeasurableImpressions,
    #[serde(rename = "METRIC_INVALID_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricInvalidActiveViewViewableImpressions,
    #[serde(rename = "METRIC_INVALID_BEGIN_TO_RENDER_IMPRESSIONS")]
    #[doc = ""]
    MetricInvalidBeginToRenderImpressions,
    #[serde(rename = "METRIC_INVALID_CLICKS")]
    #[doc = ""]
    MetricInvalidClicks,
    #[serde(rename = "METRIC_INVALID_IMPRESSIONS")]
    #[doc = ""]
    MetricInvalidImpressions,
    #[serde(rename = "METRIC_INVALID_TRACKED_ADS")]
    #[doc = ""]
    MetricInvalidTrackedAds,
    #[serde(rename = "METRIC_MEDIA_COST_ADVERTISER_CURRENCY_PER_STORE_VISIT_ADX_ONLY")]
    #[doc = ""]
    MetricMediaCostAdvertiserCurrencyPerStoreVisitAdxOnly,
    #[serde(rename = "METRIC_MIDPOINT_AUDIO")]
    #[doc = ""]
    MetricMidpointAudio,
    #[serde(rename = "METRIC_ORIGINAL_AUDIENCE_FREQUENCY")]
    #[doc = ""]
    MetricOriginalAudienceFrequency,
    #[serde(rename = "METRIC_PAUSES_AUDIO")]
    #[doc = ""]
    MetricPausesAudio,
    #[serde(rename = "METRIC_PERCENT_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE")]
    #[doc = ""]
    MetricPercentImpressionsWithPositiveCustomValue,
    #[serde(rename = "METRIC_PLATFORM_FEE_RATE")]
    #[doc = ""]
    MetricPlatformFeeRate,
    #[serde(rename = "METRIC_POST_CLICK_CONVERSIONS_CROSS_ENVIRONMENT")]
    #[doc = ""]
    MetricPostClickConversionsCrossEnvironment,
    #[serde(rename = "METRIC_POST_VIEW_CONVERSIONS_CROSS_ENVIRONMENT")]
    #[doc = ""]
    MetricPostViewConversionsCrossEnvironment,
    #[serde(rename = "METRIC_POTENTIAL_IMPRESSIONS")]
    #[doc = ""]
    MetricPotentialImpressions,
    #[serde(rename = "METRIC_POTENTIAL_VIEWS")]
    #[doc = ""]
    MetricPotentialViews,
    #[serde(rename = "METRIC_PREMIUM_FEE_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricPremiumFeeAdvertiserCurrency,
    #[serde(rename = "METRIC_PROGRAMMATIC_GUARANTEED_IMPRESSIONS_PASSED_DUE_TO_FREQUENCY")]
    #[doc = ""]
    MetricProgrammaticGuaranteedImpressionsPassedDueToFrequency,
    #[serde(
        rename = "METRIC_PROGRAMMATIC_GUARANTEED_SAVINGS_RE_INVESTED_DUE_TO_FREQUENCY_ADVERTISER_CURRENCY"
    )]
    #[doc = ""]
    MetricProgrammaticGuaranteedSavingsReInvestedDueToFrequencyAdvertiserCurrency,
    #[serde(rename = "METRIC_REFUND_BILLABLE_COST_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricRefundBillableCostAdvertiserCurrency,
    #[serde(rename = "METRIC_REFUND_MEDIA_COST_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricRefundMediaCostAdvertiserCurrency,
    #[serde(rename = "METRIC_REFUND_PLATFORM_FEE_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricRefundPlatformFeeAdvertiserCurrency,
    #[serde(rename = "METRIC_REVENUE_ADVERTISER_CURRENCY_PER_STORE_VISIT_ADX_ONLY")]
    #[doc = ""]
    MetricRevenueAdvertiserCurrencyPerStoreVisitAdxOnly,
    #[serde(rename = "METRIC_RICH_MEDIA_ENGAGEMENTS")]
    #[doc = ""]
    MetricRichMediaEngagements,
    #[serde(rename = "METRIC_STARTS_AUDIO")]
    #[doc = ""]
    MetricStartsAudio,
    #[serde(rename = "METRIC_STOPS_AUDIO")]
    #[doc = ""]
    MetricStopsAudio,
    #[serde(rename = "METRIC_STORE_VISITS_ADX_ONLY")]
    #[doc = ""]
    MetricStoreVisitsAdxOnly,
    #[serde(rename = "METRIC_STORE_VISIT_CONVERSIONS")]
    #[doc = ""]
    MetricStoreVisitConversions,
    #[serde(rename = "METRIC_THIRD_QUARTILE_AUDIO")]
    #[doc = ""]
    MetricThirdQuartileAudio,
    #[serde(rename = "METRIC_TIMERS")]
    #[doc = ""]
    MetricTimers,
    #[serde(rename = "METRIC_TOTAL_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricTotalAudioMediaCostEcpclAdvertiserCurrency,
    #[serde(rename = "METRIC_TOTAL_CONVERSIONS_CROSS_ENVIRONMENT")]
    #[doc = ""]
    MetricTotalConversionsCrossEnvironment,
    #[serde(rename = "METRIC_TOTAL_DISPLAY_TIME")]
    #[doc = ""]
    MetricTotalDisplayTime,
    #[serde(rename = "METRIC_TOTAL_IMPRESSION_CUSTOM_VALUE")]
    #[doc = ""]
    MetricTotalImpressionCustomValue,
    #[serde(rename = "METRIC_TOTAL_INTERACTION_TIME")]
    #[doc = ""]
    MetricTotalInteractionTime,
    #[serde(rename = "METRIC_TOTAL_MEDIA_COST_ADVERTISER_CURRENCY_PER_STORE_VISIT_ADX_ONLY")]
    #[doc = ""]
    MetricTotalMediaCostAdvertiserCurrencyPerStoreVisitAdxOnly,
    #[serde(rename = "METRIC_TOTAL_USERS")]
    #[doc = ""]
    MetricTotalUsers,
    #[serde(rename = "METRIC_TRACKED_ADS")]
    #[doc = ""]
    MetricTrackedAds,
    #[serde(rename = "METRIC_TRUEVIEW_GENERAL_INVALID_TRAFFIC_GIVT_VIEWS")]
    #[doc = ""]
    MetricTrueviewGeneralInvalidTrafficGivtViews,
    #[serde(rename = "METRIC_TRUEVIEW_INVALID_VIEWS")]
    #[doc = ""]
    MetricTrueviewInvalidViews,
    #[serde(rename = "METRIC_UNIQUE_COOKIES_WITH_IMPRESSIONS")]
    #[doc = ""]
    MetricUniqueCookiesWithImpressions,
    #[serde(rename = "METRIC_UNIQUE_REACH_AVERAGE_IMPRESSION_FREQUENCY")]
    #[doc = ""]
    MetricUniqueReachAverageImpressionFrequency,
    #[serde(rename = "METRIC_UNIQUE_REACH_CLICK_REACH")]
    #[doc = ""]
    MetricUniqueReachClickReach,
    #[serde(rename = "METRIC_UNIQUE_REACH_IMPRESSION_REACH")]
    #[doc = ""]
    MetricUniqueReachImpressionReach,
    #[serde(rename = "METRIC_UNIQUE_REACH_TOTAL_REACH")]
    #[doc = ""]
    MetricUniqueReachTotalReach,
    #[serde(rename = "METRIC_VERIFIABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricVerifiableImpressions,
    #[serde(rename = "METRIC_VIDEO_CLIENT_COST_ECPCV_ADVERTISER_CURRENCY")]
    #[doc = ""]
    MetricVideoClientCostEcpcvAdvertiserCurrency,
    #[serde(rename = "METRIC_WATCH_TIME")]
    #[doc = ""]
    MetricWatchTime,
    #[serde(rename = "METRIC_LAST_TOUCH_TOTAL_CONVERSIONS")]
    #[doc = ""]
    MetricLastTouchTotalConversions,
    #[serde(rename = "METRIC_LAST_TOUCH_CLICK_THROUGH_CONVERSIONS")]
    #[doc = ""]
    MetricLastTouchClickThroughConversions,
    #[serde(rename = "METRIC_LAST_TOUCH_VIEW_THROUGH_CONVERSIONS")]
    #[doc = ""]
    MetricLastTouchViewThroughConversions,
    #[serde(rename = "METRIC_TOTAL_PATHS")]
    #[doc = ""]
    MetricTotalPaths,
    #[serde(rename = "METRIC_TOTAL_EXPOSURES")]
    #[doc = ""]
    MetricTotalExposures,
    #[serde(rename = "METRIC_PATH_CONVERSION_RATE")]
    #[doc = ""]
    MetricPathConversionRate,
    #[serde(rename = "METRIC_CONVERTING_PATHS")]
    #[doc = ""]
    MetricConvertingPaths,
    #[serde(rename = "METRIC_ACTIVITY_REVENUE")]
    #[doc = ""]
    MetricActivityRevenue,
    #[serde(rename = "METRIC_PERCENT_INVALID_IMPRESSIONS_PREBID")]
    #[doc = ""]
    MetricPercentInvalidImpressionsPrebid,
    #[serde(rename = "METRIC_GRP_CORRECTED_IMPRESSIONS")]
    #[doc = ""]
    MetricGrpCorrectedImpressions,
    #[serde(rename = "METRIC_DEMO_CORRECTED_CLICKS")]
    #[doc = ""]
    MetricDemoCorrectedClicks,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_BY_DEMO")]
    #[doc = ""]
    MetricVirtualPeopleImpressionReachByDemo,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_CLICK_REACH_BY_DEMO")]
    #[doc = ""]
    MetricVirtualPeopleClickReachByDemo,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_AVERAGE_IMPRESSION_FREQUENCY_BY_DEMO")]
    #[doc = ""]
    MetricVirtualPeopleAverageImpressionFrequencyByDemo,
    #[serde(rename = "METRIC_DEMO_COMPOSITION_IMPRESSION")]
    #[doc = ""]
    MetricDemoCompositionImpression,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_SHARE_PERCENT")]
    #[doc = ""]
    MetricVirtualPeopleImpressionReachSharePercent,
    #[serde(rename = "METRIC_DEMO_POPULATION")]
    #[doc = ""]
    MetricDemoPopulation,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_PERCENT")]
    #[doc = ""]
    MetricVirtualPeopleImpressionReachPercent,
    #[serde(rename = "METRIC_TARGET_RATING_POINTS")]
    #[doc = ""]
    MetricTargetRatingPoints,
    #[serde(rename = "METRIC_PROVISIONAL_IMPRESSIONS")]
    #[doc = ""]
    MetricProvisionalImpressions,
    #[serde(rename = "METRIC_VENDOR_BLOCKED_ADS")]
    #[doc = ""]
    MetricVendorBlockedAds,
    #[serde(rename = "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS")]
    #[doc = ""]
    MetricGrpCorrectedViewableImpressions,
    #[serde(rename = "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS_SHARE_PERCENT")]
    #[doc = ""]
    MetricGrpCorrectedViewableImpressionsSharePercent,
    #[serde(rename = "METRIC_VIEWABLE_GROSS_RATING_POINTS")]
    #[doc = ""]
    MetricViewableGrossRatingPoints,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_AVERAGE_VIEWABLE_IMPRESSION_FREQUENCY_BY_DEMO")]
    #[doc = ""]
    MetricVirtualPeopleAverageViewableImpressionFrequencyByDemo,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_BY_DEMO")]
    #[doc = ""]
    MetricVirtualPeopleViewableImpressionReachByDemo,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_PERCENT")]
    #[doc = ""]
    MetricVirtualPeopleViewableImpressionReachPercent,
    #[serde(rename = "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_SHARE_PERCENT")]
    #[doc = ""]
    MetricVirtualPeopleViewableImpressionReachSharePercent,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Report type."]
pub enum ParametersTypeEnum {
    #[serde(rename = "TYPE_GENERAL")]
    #[doc = ""]
    TypeGeneral,
    #[serde(rename = "TYPE_AUDIENCE_PERFORMANCE")]
    #[doc = ""]
    TypeAudiencePerformance,
    #[serde(rename = "TYPE_INVENTORY_AVAILABILITY")]
    #[doc = ""]
    TypeInventoryAvailability,
    #[serde(rename = "TYPE_KEYWORD")]
    #[doc = ""]
    TypeKeyword,
    #[serde(rename = "TYPE_PIXEL_LOAD")]
    #[doc = ""]
    TypePixelLoad,
    #[serde(rename = "TYPE_AUDIENCE_COMPOSITION")]
    #[doc = ""]
    TypeAudienceComposition,
    #[serde(rename = "TYPE_CROSS_PARTNER")]
    #[doc = ""]
    TypeCrossPartner,
    #[serde(rename = "TYPE_PAGE_CATEGORY")]
    #[doc = ""]
    TypePageCategory,
    #[serde(rename = "TYPE_THIRD_PARTY_DATA_PROVIDER")]
    #[doc = ""]
    TypeThirdPartyDataProvider,
    #[serde(rename = "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER")]
    #[doc = ""]
    TypeCrossPartnerThirdPartyDataProvider,
    #[serde(rename = "TYPE_CLIENT_SAFE")]
    #[doc = ""]
    TypeClientSafe,
    #[serde(rename = "TYPE_ORDER_ID")]
    #[doc = ""]
    TypeOrderId,
    #[serde(rename = "TYPE_FEE")]
    #[doc = ""]
    TypeFee,
    #[serde(rename = "TYPE_CROSS_FEE")]
    #[doc = ""]
    TypeCrossFee,
    #[serde(rename = "TYPE_ACTIVE_GRP")]
    #[doc = ""]
    TypeActiveGrp,
    #[serde(rename = "TYPE_YOUTUBE_VERTICAL")]
    #[doc = ""]
    TypeYoutubeVertical,
    #[serde(rename = "TYPE_COMSCORE_VCE")]
    #[doc = ""]
    TypeComscoreVce,
    #[serde(rename = "TYPE_TRUEVIEW")]
    #[doc = ""]
    TypeTrueview,
    #[serde(rename = "TYPE_NIELSEN_AUDIENCE_PROFILE")]
    #[doc = ""]
    TypeNielsenAudienceProfile,
    #[serde(rename = "TYPE_NIELSEN_DAILY_REACH_BUILD")]
    #[doc = ""]
    TypeNielsenDailyReachBuild,
    #[serde(rename = "TYPE_NIELSEN_SITE")]
    #[doc = ""]
    TypeNielsenSite,
    #[serde(rename = "TYPE_REACH_AND_FREQUENCY")]
    #[doc = ""]
    TypeReachAndFrequency,
    #[serde(rename = "TYPE_ESTIMATED_CONVERSION")]
    #[doc = ""]
    TypeEstimatedConversion,
    #[serde(rename = "TYPE_VERIFICATION")]
    #[doc = ""]
    TypeVerification,
    #[serde(rename = "TYPE_TRUEVIEW_IAR")]
    #[doc = ""]
    TypeTrueviewIar,
    #[serde(rename = "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET")]
    #[doc = ""]
    TypeNielsenOnlineGlobalMarket,
    #[serde(rename = "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE")]
    #[doc = ""]
    TypePetraNielsenAudienceProfile,
    #[serde(rename = "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD")]
    #[doc = ""]
    TypePetraNielsenDailyReachBuild,
    #[serde(rename = "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET")]
    #[doc = ""]
    TypePetraNielsenOnlineGlobalMarket,
    #[serde(rename = "TYPE_NOT_SUPPORTED")]
    #[doc = ""]
    TypeNotSupported,
    #[serde(rename = "TYPE_REACH_AUDIENCE")]
    #[doc = ""]
    TypeReachAudience,
    #[serde(rename = "TYPE_LINEAR_TV_SEARCH_LIFT")]
    #[doc = ""]
    TypeLinearTvSearchLift,
    #[serde(rename = "TYPE_PATH")]
    #[doc = ""]
    TypePath,
    #[serde(rename = "TYPE_PATH_ATTRIBUTION")]
    #[doc = ""]
    TypePathAttribution,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Path filters specify which paths to include in a report. A path is the result of combining DV360 events based on User ID to create a workflow of users' actions. When a path filter is set, the resulting report will only include paths that match the specified event at the specified position. All other paths will be excluded."]
pub struct PathFilter {
    #[serde(rename = "eventFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter on an event to be applied to some part of the path."]
    pub event_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventFilter>>>,
    #[serde(rename = "pathMatchPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the position of the path the filter should match to (first, last, or any event in path)."]
    pub path_match_position: ::std::option::Option<PathFilterPathMatchPositionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates the position of the path the filter should match to (first, last, or any event in path)."]
pub enum PathFilterPathMatchPositionEnum {
    #[serde(rename = "ANY")]
    #[doc = ""]
    Any,
    #[serde(rename = "FIRST")]
    #[doc = ""]
    First,
    #[serde(rename = "LAST")]
    #[doc = ""]
    Last,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Path Query Options for Report Options."]
pub struct PathQueryOptions {
    #[serde(rename = "channelGrouping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom Channel Groupings."]
    pub channel_grouping: ::std::option::Option<::std::boxed::Box<ChannelGrouping>>,
    #[serde(rename = "pathFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Path Filters. There is a limit of 100 path filters that can be set per report."]
    pub path_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PathFilter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dimension Filter on path events."]
pub struct PathQueryOptionsFilter {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension the filter is applied to."]
    pub filter: ::std::option::Option<PathQueryOptionsFilterFilterEnum>,
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates how the filter should be matched to the value."]
    pub _match: ::std::option::Option<PathQueryOptionsFilterMatchEnum>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value to filter on."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Dimension the filter is applied to."]
pub enum PathQueryOptionsFilterFilterEnum {
    #[serde(rename = "FILTER_UNKNOWN")]
    #[doc = ""]
    FilterUnknown,
    #[serde(rename = "FILTER_DATE")]
    #[doc = ""]
    FilterDate,
    #[serde(rename = "FILTER_DAY_OF_WEEK")]
    #[doc = ""]
    FilterDayOfWeek,
    #[serde(rename = "FILTER_WEEK")]
    #[doc = ""]
    FilterWeek,
    #[serde(rename = "FILTER_MONTH")]
    #[doc = ""]
    FilterMonth,
    #[serde(rename = "FILTER_YEAR")]
    #[doc = ""]
    FilterYear,
    #[serde(rename = "FILTER_TIME_OF_DAY")]
    #[doc = ""]
    FilterTimeOfDay,
    #[serde(rename = "FILTER_CONVERSION_DELAY")]
    #[doc = ""]
    FilterConversionDelay,
    #[serde(rename = "FILTER_CREATIVE_ID")]
    #[doc = ""]
    FilterCreativeId,
    #[serde(rename = "FILTER_CREATIVE_SIZE")]
    #[doc = ""]
    FilterCreativeSize,
    #[serde(rename = "FILTER_CREATIVE_TYPE")]
    #[doc = ""]
    FilterCreativeType,
    #[serde(rename = "FILTER_EXCHANGE_ID")]
    #[doc = ""]
    FilterExchangeId,
    #[serde(rename = "FILTER_AD_POSITION")]
    #[doc = ""]
    FilterAdPosition,
    #[serde(rename = "FILTER_PUBLIC_INVENTORY")]
    #[doc = ""]
    FilterPublicInventory,
    #[serde(rename = "FILTER_INVENTORY_SOURCE")]
    #[doc = ""]
    FilterInventorySource,
    #[serde(rename = "FILTER_CITY")]
    #[doc = ""]
    FilterCity,
    #[serde(rename = "FILTER_REGION")]
    #[doc = ""]
    FilterRegion,
    #[serde(rename = "FILTER_DMA")]
    #[doc = ""]
    FilterDma,
    #[serde(rename = "FILTER_COUNTRY")]
    #[doc = ""]
    FilterCountry,
    #[serde(rename = "FILTER_SITE_ID")]
    #[doc = ""]
    FilterSiteId,
    #[serde(rename = "FILTER_CHANNEL_ID")]
    #[doc = ""]
    FilterChannelId,
    #[serde(rename = "FILTER_PARTNER")]
    #[doc = ""]
    FilterPartner,
    #[serde(rename = "FILTER_ADVERTISER")]
    #[doc = ""]
    FilterAdvertiser,
    #[serde(rename = "FILTER_INSERTION_ORDER")]
    #[doc = ""]
    FilterInsertionOrder,
    #[serde(rename = "FILTER_LINE_ITEM")]
    #[doc = ""]
    FilterLineItem,
    #[serde(rename = "FILTER_PARTNER_CURRENCY")]
    #[doc = ""]
    FilterPartnerCurrency,
    #[serde(rename = "FILTER_ADVERTISER_CURRENCY")]
    #[doc = ""]
    FilterAdvertiserCurrency,
    #[serde(rename = "FILTER_ADVERTISER_TIMEZONE")]
    #[doc = ""]
    FilterAdvertiserTimezone,
    #[serde(rename = "FILTER_LINE_ITEM_TYPE")]
    #[doc = ""]
    FilterLineItemType,
    #[serde(rename = "FILTER_USER_LIST")]
    #[doc = ""]
    FilterUserList,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY")]
    #[doc = ""]
    FilterUserListFirstParty,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY")]
    #[doc = ""]
    FilterUserListThirdParty,
    #[serde(rename = "FILTER_TARGETED_USER_LIST")]
    #[doc = ""]
    FilterTargetedUserList,
    #[serde(rename = "FILTER_DATA_PROVIDER")]
    #[doc = ""]
    FilterDataProvider,
    #[serde(rename = "FILTER_ORDER_ID")]
    #[doc = ""]
    FilterOrderId,
    #[serde(rename = "FILTER_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVideoPlayerSize,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS")]
    #[doc = ""]
    FilterVideoDurationSeconds,
    #[serde(rename = "FILTER_KEYWORD")]
    #[doc = ""]
    FilterKeyword,
    #[serde(rename = "FILTER_PAGE_CATEGORY")]
    #[doc = ""]
    FilterPageCategory,
    #[serde(rename = "FILTER_CAMPAIGN_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterCampaignDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_DAILY_FREQUENCY")]
    #[doc = ""]
    FilterLineItemDailyFrequency,
    #[serde(rename = "FILTER_LINE_ITEM_LIFETIME_FREQUENCY")]
    #[doc = ""]
    FilterLineItemLifetimeFrequency,
    #[serde(rename = "FILTER_OS")]
    #[doc = ""]
    FilterOs,
    #[serde(rename = "FILTER_BROWSER")]
    #[doc = ""]
    FilterBrowser,
    #[serde(rename = "FILTER_CARRIER")]
    #[doc = ""]
    FilterCarrier,
    #[serde(rename = "FILTER_SITE_LANGUAGE")]
    #[doc = ""]
    FilterSiteLanguage,
    #[serde(rename = "FILTER_INVENTORY_FORMAT")]
    #[doc = ""]
    FilterInventoryFormat,
    #[serde(rename = "FILTER_ZIP_CODE")]
    #[doc = ""]
    FilterZipCode,
    #[serde(rename = "FILTER_VIDEO_RATING_TIER")]
    #[doc = ""]
    FilterVideoRatingTier,
    #[serde(rename = "FILTER_VIDEO_FORMAT_SUPPORT")]
    #[doc = ""]
    FilterVideoFormatSupport,
    #[serde(rename = "FILTER_VIDEO_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterVideoSkippableSupport,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION")]
    #[doc = ""]
    FilterVideoCreativeDuration,
    #[serde(rename = "FILTER_PAGE_LAYOUT")]
    #[doc = ""]
    FilterPageLayout,
    #[serde(rename = "FILTER_VIDEO_AD_POSITION_IN_STREAM")]
    #[doc = ""]
    FilterVideoAdPositionInStream,
    #[serde(rename = "FILTER_AGE")]
    #[doc = ""]
    FilterAge,
    #[serde(rename = "FILTER_GENDER")]
    #[doc = ""]
    FilterGender,
    #[serde(rename = "FILTER_QUARTER")]
    #[doc = ""]
    FilterQuarter,
    #[serde(rename = "FILTER_TRUEVIEW_CONVERSION_TYPE")]
    #[doc = ""]
    FilterTrueviewConversionType,
    #[serde(rename = "FILTER_MOBILE_GEO")]
    #[doc = ""]
    FilterMobileGeo,
    #[serde(rename = "FILTER_MRAID_SUPPORT")]
    #[doc = ""]
    FilterMraidSupport,
    #[serde(rename = "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY")]
    #[doc = ""]
    FilterActiveViewExpectedViewability,
    #[serde(rename = "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE")]
    #[doc = ""]
    FilterVideoCreativeDurationSkippable,
    #[serde(rename = "FILTER_NIELSEN_COUNTRY_CODE")]
    #[doc = ""]
    FilterNielsenCountryCode,
    #[serde(rename = "FILTER_NIELSEN_DEVICE_ID")]
    #[doc = ""]
    FilterNielsenDeviceId,
    #[serde(rename = "FILTER_NIELSEN_GENDER")]
    #[doc = ""]
    FilterNielsenGender,
    #[serde(rename = "FILTER_NIELSEN_AGE")]
    #[doc = ""]
    FilterNielsenAge,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_TYPE")]
    #[doc = ""]
    FilterInventorySourceType,
    #[serde(rename = "FILTER_CREATIVE_WIDTH")]
    #[doc = ""]
    FilterCreativeWidth,
    #[serde(rename = "FILTER_CREATIVE_HEIGHT")]
    #[doc = ""]
    FilterCreativeHeight,
    #[serde(rename = "FILTER_DFP_ORDER_ID")]
    #[doc = ""]
    FilterDfpOrderId,
    #[serde(rename = "FILTER_TRUEVIEW_AGE")]
    #[doc = ""]
    FilterTrueviewAge,
    #[serde(rename = "FILTER_TRUEVIEW_GENDER")]
    #[doc = ""]
    FilterTrueviewGender,
    #[serde(rename = "FILTER_TRUEVIEW_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_INTEREST")]
    #[doc = ""]
    FilterTrueviewInterest,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupId,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP_AD_ID")]
    #[doc = ""]
    FilterTrueviewAdGroupAdId,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_LANGUAGE")]
    #[doc = ""]
    FilterTrueviewIarLanguage,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_GENDER")]
    #[doc = ""]
    FilterTrueviewIarGender,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_AGE")]
    #[doc = ""]
    FilterTrueviewIarAge,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CATEGORY")]
    #[doc = ""]
    FilterTrueviewIarCategory,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY")]
    #[doc = ""]
    FilterTrueviewIarCountry,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_CITY")]
    #[doc = ""]
    FilterTrueviewIarCity,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION")]
    #[doc = ""]
    FilterTrueviewIarRegion,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewIarZipcode,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REMARKETING_LIST")]
    #[doc = ""]
    FilterTrueviewIarRemarketingList,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_INTEREST")]
    #[doc = ""]
    FilterTrueviewIarInterest,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS")]
    #[doc = ""]
    FilterTrueviewIarParentalStatus,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_TIME_OF_DAY")]
    #[doc = ""]
    FilterTrueviewIarTimeOfDay,
    #[serde(rename = "FILTER_TRUEVIEW_CUSTOM_AFFINITY")]
    #[doc = ""]
    FilterTrueviewCustomAffinity,
    #[serde(rename = "FILTER_TRUEVIEW_CATEGORY")]
    #[doc = ""]
    FilterTrueviewCategory,
    #[serde(rename = "FILTER_TRUEVIEW_KEYWORD")]
    #[doc = ""]
    FilterTrueviewKeyword,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT")]
    #[doc = ""]
    FilterTrueviewPlacement,
    #[serde(rename = "FILTER_TRUEVIEW_URL")]
    #[doc = ""]
    FilterTrueviewUrl,
    #[serde(rename = "FILTER_TRUEVIEW_COUNTRY")]
    #[doc = ""]
    FilterTrueviewCountry,
    #[serde(rename = "FILTER_TRUEVIEW_REGION")]
    #[doc = ""]
    FilterTrueviewRegion,
    #[serde(rename = "FILTER_TRUEVIEW_CITY")]
    #[doc = ""]
    FilterTrueviewCity,
    #[serde(rename = "FILTER_TRUEVIEW_DMA")]
    #[doc = ""]
    FilterTrueviewDma,
    #[serde(rename = "FILTER_TRUEVIEW_ZIPCODE")]
    #[doc = ""]
    FilterTrueviewZipcode,
    #[serde(rename = "FILTER_NOT_SUPPORTED")]
    #[doc = ""]
    FilterNotSupported,
    #[serde(rename = "FILTER_MEDIA_PLAN")]
    #[doc = ""]
    FilterMediaPlan,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterTrueviewIarYoutubeChannel,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterTrueviewIarYoutubeVideo,
    #[serde(rename = "FILTER_SKIPPABLE_SUPPORT")]
    #[doc = ""]
    FilterSkippableSupport,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_ID")]
    #[doc = ""]
    FilterCompanionCreativeId,
    #[serde(rename = "FILTER_BUDGET_SEGMENT_DESCRIPTION")]
    #[doc = ""]
    FilterBudgetSegmentDescription,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY_ID")]
    #[doc = ""]
    FilterFloodlightActivityId,
    #[serde(rename = "FILTER_DEVICE_MODEL")]
    #[doc = ""]
    FilterDeviceModel,
    #[serde(rename = "FILTER_DEVICE_MAKE")]
    #[doc = ""]
    FilterDeviceMake,
    #[serde(rename = "FILTER_DEVICE_TYPE")]
    #[doc = ""]
    FilterDeviceType,
    #[serde(rename = "FILTER_CREATIVE_ATTRIBUTE")]
    #[doc = ""]
    FilterCreativeAttribute,
    #[serde(rename = "FILTER_INVENTORY_COMMITMENT_TYPE")]
    #[doc = ""]
    FilterInventoryCommitmentType,
    #[serde(rename = "FILTER_INVENTORY_RATE_TYPE")]
    #[doc = ""]
    FilterInventoryRateType,
    #[serde(rename = "FILTER_INVENTORY_DELIVERY_METHOD")]
    #[doc = ""]
    FilterInventoryDeliveryMethod,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_EXTERNAL_ID")]
    #[doc = ""]
    FilterInventorySourceExternalId,
    #[serde(rename = "FILTER_AUTHORIZED_SELLER_STATE")]
    #[doc = ""]
    FilterAuthorizedSellerState,
    #[serde(rename = "FILTER_VIDEO_DURATION_SECONDS_RANGE")]
    #[doc = ""]
    FilterVideoDurationSecondsRange,
    #[serde(rename = "FILTER_PARTNER_NAME")]
    #[doc = ""]
    FilterPartnerName,
    #[serde(rename = "FILTER_PARTNER_STATUS")]
    #[doc = ""]
    FilterPartnerStatus,
    #[serde(rename = "FILTER_ADVERTISER_NAME")]
    #[doc = ""]
    FilterAdvertiserName,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterAdvertiserIntegrationCode,
    #[serde(rename = "FILTER_ADVERTISER_INTEGRATION_STATUS")]
    #[doc = ""]
    FilterAdvertiserIntegrationStatus,
    #[serde(rename = "FILTER_CARRIER_NAME")]
    #[doc = ""]
    FilterCarrierName,
    #[serde(rename = "FILTER_CHANNEL_NAME")]
    #[doc = ""]
    FilterChannelName,
    #[serde(rename = "FILTER_CITY_NAME")]
    #[doc = ""]
    FilterCityName,
    #[serde(rename = "FILTER_COMPANION_CREATIVE_NAME")]
    #[doc = ""]
    FilterCompanionCreativeName,
    #[serde(rename = "FILTER_USER_LIST_FIRST_PARTY_NAME")]
    #[doc = ""]
    FilterUserListFirstPartyName,
    #[serde(rename = "FILTER_USER_LIST_THIRD_PARTY_NAME")]
    #[doc = ""]
    FilterUserListThirdPartyName,
    #[serde(rename = "FILTER_NIELSEN_RESTATEMENT_DATE")]
    #[doc = ""]
    FilterNielsenRestatementDate,
    #[serde(rename = "FILTER_NIELSEN_DATE_RANGE")]
    #[doc = ""]
    FilterNielsenDateRange,
    #[serde(rename = "FILTER_INSERTION_ORDER_NAME")]
    #[doc = ""]
    FilterInsertionOrderName,
    #[serde(rename = "FILTER_REGION_NAME")]
    #[doc = ""]
    FilterRegionName,
    #[serde(rename = "FILTER_DMA_NAME")]
    #[doc = ""]
    FilterDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewIarRegionName,
    #[serde(rename = "FILTER_TRUEVIEW_DMA_NAME")]
    #[doc = ""]
    FilterTrueviewDmaName,
    #[serde(rename = "FILTER_TRUEVIEW_REGION_NAME")]
    #[doc = ""]
    FilterTrueviewRegionName,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID")]
    #[doc = ""]
    FilterActiveViewCustomMetricId,
    #[serde(rename = "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME")]
    #[doc = ""]
    FilterActiveViewCustomMetricName,
    #[serde(rename = "FILTER_AD_TYPE")]
    #[doc = ""]
    FilterAdType,
    #[serde(rename = "FILTER_ALGORITHM")]
    #[doc = ""]
    FilterAlgorithm,
    #[serde(rename = "FILTER_ALGORITHM_ID")]
    #[doc = ""]
    FilterAlgorithmId,
    #[serde(rename = "FILTER_AMP_PAGE_REQUEST")]
    #[doc = ""]
    FilterAmpPageRequest,
    #[serde(rename = "FILTER_ANONYMOUS_INVENTORY_MODELING")]
    #[doc = ""]
    FilterAnonymousInventoryModeling,
    #[serde(rename = "FILTER_APP_URL")]
    #[doc = ""]
    FilterAppUrl,
    #[serde(rename = "FILTER_APP_URL_EXCLUDED")]
    #[doc = ""]
    FilterAppUrlExcluded,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST")]
    #[doc = ""]
    FilterAttributedUserlist,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_COST")]
    #[doc = ""]
    FilterAttributedUserlistCost,
    #[serde(rename = "FILTER_ATTRIBUTED_USERLIST_TYPE")]
    #[doc = ""]
    FilterAttributedUserlistType,
    #[serde(rename = "FILTER_ATTRIBUTION_MODEL")]
    #[doc = ""]
    FilterAttributionModel,
    #[serde(rename = "FILTER_AUDIENCE_LIST")]
    #[doc = ""]
    FilterAudienceList,
    #[serde(rename = "FILTER_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterAudienceListCost,
    #[serde(rename = "FILTER_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterAudienceListType,
    #[serde(rename = "FILTER_AUDIENCE_NAME")]
    #[doc = ""]
    FilterAudienceName,
    #[serde(rename = "FILTER_AUDIENCE_TYPE")]
    #[doc = ""]
    FilterAudienceType,
    #[serde(rename = "FILTER_BILLABLE_OUTCOME")]
    #[doc = ""]
    FilterBillableOutcome,
    #[serde(rename = "FILTER_BRAND_LIFT_TYPE")]
    #[doc = ""]
    FilterBrandLiftType,
    #[serde(rename = "FILTER_CHANNEL_TYPE")]
    #[doc = ""]
    FilterChannelType,
    #[serde(rename = "FILTER_CM_PLACEMENT_ID")]
    #[doc = ""]
    FilterCmPlacementId,
    #[serde(rename = "FILTER_CONVERSION_SOURCE")]
    #[doc = ""]
    FilterConversionSource,
    #[serde(rename = "FILTER_CONVERSION_SOURCE_ID")]
    #[doc = ""]
    FilterConversionSourceId,
    #[serde(rename = "FILTER_COUNTRY_ID")]
    #[doc = ""]
    FilterCountryId,
    #[serde(rename = "FILTER_CREATIVE")]
    #[doc = ""]
    FilterCreative,
    #[serde(rename = "FILTER_CREATIVE_ASSET")]
    #[doc = ""]
    FilterCreativeAsset,
    #[serde(rename = "FILTER_CREATIVE_INTEGRATION_CODE")]
    #[doc = ""]
    FilterCreativeIntegrationCode,
    #[serde(rename = "FILTER_CREATIVE_RENDERED_IN_AMP")]
    #[doc = ""]
    FilterCreativeRenderedInAmp,
    #[serde(rename = "FILTER_CREATIVE_SOURCE")]
    #[doc = ""]
    FilterCreativeSource,
    #[serde(rename = "FILTER_CREATIVE_STATUS")]
    #[doc = ""]
    FilterCreativeStatus,
    #[serde(rename = "FILTER_DATA_PROVIDER_NAME")]
    #[doc = ""]
    FilterDataProviderName,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterDetailedDemographics,
    #[serde(rename = "FILTER_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterDetailedDemographicsId,
    #[serde(rename = "FILTER_DEVICE")]
    #[doc = ""]
    FilterDevice,
    #[serde(rename = "FILTER_GAM_INSERTION_ORDER")]
    #[doc = ""]
    FilterGamInsertionOrder,
    #[serde(rename = "FILTER_GAM_LINE_ITEM")]
    #[doc = ""]
    FilterGamLineItem,
    #[serde(rename = "FILTER_GAM_LINE_ITEM_ID")]
    #[doc = ""]
    FilterGamLineItemId,
    #[serde(rename = "FILTER_DIGITAL_CONTENT_LABEL")]
    #[doc = ""]
    FilterDigitalContentLabel,
    #[serde(rename = "FILTER_DOMAIN")]
    #[doc = ""]
    FilterDomain,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST")]
    #[doc = ""]
    FilterEligibleCookiesOnFirstPartyAudienceList,
    #[serde(rename = "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST")]
    #[doc = ""]
    FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
    #[serde(rename = "FILTER_EXCHANGE")]
    #[doc = ""]
    FilterExchange,
    #[serde(rename = "FILTER_EXCHANGE_CODE")]
    #[doc = ""]
    FilterExchangeCode,
    #[serde(rename = "FILTER_EXTENSION")]
    #[doc = ""]
    FilterExtension,
    #[serde(rename = "FILTER_EXTENSION_STATUS")]
    #[doc = ""]
    FilterExtensionStatus,
    #[serde(rename = "FILTER_EXTENSION_TYPE")]
    #[doc = ""]
    FilterExtensionType,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterFirstPartyAudienceListCost,
    #[serde(rename = "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterFirstPartyAudienceListType,
    #[serde(rename = "FILTER_FLOODLIGHT_ACTIVITY")]
    #[doc = ""]
    FilterFloodlightActivity,
    #[serde(rename = "FILTER_FORMAT")]
    #[doc = ""]
    FilterFormat,
    #[serde(rename = "FILTER_GMAIL_AGE")]
    #[doc = ""]
    FilterGmailAge,
    #[serde(rename = "FILTER_GMAIL_CITY")]
    #[doc = ""]
    FilterGmailCity,
    #[serde(rename = "FILTER_GMAIL_COUNTRY")]
    #[doc = ""]
    FilterGmailCountry,
    #[serde(rename = "FILTER_GMAIL_COUNTRY_NAME")]
    #[doc = ""]
    FilterGmailCountryName,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE")]
    #[doc = ""]
    FilterGmailDeviceType,
    #[serde(rename = "FILTER_GMAIL_DEVICE_TYPE_NAME")]
    #[doc = ""]
    FilterGmailDeviceTypeName,
    #[serde(rename = "FILTER_GMAIL_GENDER")]
    #[doc = ""]
    FilterGmailGender,
    #[serde(rename = "FILTER_GMAIL_REGION")]
    #[doc = ""]
    FilterGmailRegion,
    #[serde(rename = "FILTER_GMAIL_REMARKETING_LIST")]
    #[doc = ""]
    FilterGmailRemarketingList,
    #[serde(rename = "FILTER_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterHouseholdIncome,
    #[serde(rename = "FILTER_IMPRESSION_COUNTING_METHOD")]
    #[doc = ""]
    FilterImpressionCountingMethod,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedInsertionOrder,
    #[serde(rename = "FILTER_INSERTION_ORDER_INTEGRATION_CODE")]
    #[doc = ""]
    FilterInsertionOrderIntegrationCode,
    #[serde(rename = "FILTER_INSERTION_ORDER_STATUS")]
    #[doc = ""]
    FilterInsertionOrderStatus,
    #[serde(rename = "FILTER_INTEREST")]
    #[doc = ""]
    FilterInterest,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP")]
    #[doc = ""]
    FilterInventorySourceGroup,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_GROUP_ID")]
    #[doc = ""]
    FilterInventorySourceGroupId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_ID")]
    #[doc = ""]
    FilterInventorySourceId,
    #[serde(rename = "FILTER_INVENTORY_SOURCE_NAME")]
    #[doc = ""]
    FilterInventorySourceName,
    #[serde(rename = "FILTER_LIFE_EVENT")]
    #[doc = ""]
    FilterLifeEvent,
    #[serde(rename = "FILTER_LIFE_EVENTS")]
    #[doc = ""]
    FilterLifeEvents,
    #[serde(rename = "FILTER_LINE_ITEM_INTEGRATION_CODE")]
    #[doc = ""]
    FilterLineItemIntegrationCode,
    #[serde(rename = "FILTER_LINE_ITEM_NAME")]
    #[doc = ""]
    FilterLineItemName,
    #[serde(rename = "FILTER_LINE_ITEM_STATUS")]
    #[doc = ""]
    FilterLineItemStatus,
    #[serde(rename = "FILTER_MATCH_RATIO")]
    #[doc = ""]
    FilterMatchRatio,
    #[serde(rename = "FILTER_MEASUREMENT_SOURCE")]
    #[doc = ""]
    FilterMeasurementSource,
    #[serde(rename = "FILTER_MEDIA_PLAN_NAME")]
    #[doc = ""]
    FilterMediaPlanName,
    #[serde(rename = "FILTER_PARENTAL_STATUS")]
    #[doc = ""]
    FilterParentalStatus,
    #[serde(rename = "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementAllYoutubeChannels,
    #[serde(rename = "FILTER_PLATFORM")]
    #[doc = ""]
    FilterPlatform,
    #[serde(rename = "FILTER_PLAYBACK_METHOD")]
    #[doc = ""]
    FilterPlaybackMethod,
    #[serde(rename = "FILTER_POSITION_IN_CONTENT")]
    #[doc = ""]
    FilterPositionInContent,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY")]
    #[doc = ""]
    FilterPublisherProperty,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_ID")]
    #[doc = ""]
    FilterPublisherPropertyId,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION")]
    #[doc = ""]
    FilterPublisherPropertySection,
    #[serde(rename = "FILTER_PUBLISHER_PROPERTY_SECTION_ID")]
    #[doc = ""]
    FilterPublisherPropertySectionId,
    #[serde(rename = "FILTER_REFUND_REASON")]
    #[doc = ""]
    FilterRefundReason,
    #[serde(rename = "FILTER_REMARKETING_LIST")]
    #[doc = ""]
    FilterRemarketingList,
    #[serde(rename = "FILTER_REWARDED")]
    #[doc = ""]
    FilterRewarded,
    #[serde(rename = "FILTER_SENSITIVE_CATEGORY")]
    #[doc = ""]
    FilterSensitiveCategory,
    #[serde(rename = "FILTER_SERVED_PIXEL_DENSITY")]
    #[doc = ""]
    FilterServedPixelDensity,
    #[serde(rename = "FILTER_TARGETED_DATA_PROVIDERS")]
    #[doc = ""]
    FilterTargetedDataProviders,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST")]
    #[doc = ""]
    FilterThirdPartyAudienceListCost,
    #[serde(rename = "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE")]
    #[doc = ""]
    FilterThirdPartyAudienceListType,
    #[serde(rename = "FILTER_TRUEVIEW_AD")]
    #[doc = ""]
    FilterTrueviewAd,
    #[serde(rename = "FILTER_TRUEVIEW_AD_GROUP")]
    #[doc = ""]
    FilterTrueviewAdGroup,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS")]
    #[doc = ""]
    FilterTrueviewDetailedDemographics,
    #[serde(rename = "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID")]
    #[doc = ""]
    FilterTrueviewDetailedDemographicsId,
    #[serde(rename = "FILTER_TRUEVIEW_HOUSEHOLD_INCOME")]
    #[doc = ""]
    FilterTrueviewHouseholdIncome,
    #[serde(rename = "FILTER_TRUEVIEW_IAR_COUNTRY_NAME")]
    #[doc = ""]
    FilterTrueviewIarCountryName,
    #[serde(rename = "FILTER_TRUEVIEW_REMARKETING_LIST_NAME")]
    #[doc = ""]
    FilterTrueviewRemarketingListName,
    #[serde(rename = "FILTER_VARIANT_ID")]
    #[doc = ""]
    FilterVariantId,
    #[serde(rename = "FILTER_VARIANT_NAME")]
    #[doc = ""]
    FilterVariantName,
    #[serde(rename = "FILTER_VARIANT_VERSION")]
    #[doc = ""]
    FilterVariantVersion,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE")]
    #[doc = ""]
    FilterVerificationVideoPlayerSize,
    #[serde(rename = "FILTER_VERIFICATION_VIDEO_POSITION")]
    #[doc = ""]
    FilterVerificationVideoPosition,
    #[serde(rename = "FILTER_VIDEO_COMPANION_CREATIVE_SIZE")]
    #[doc = ""]
    FilterVideoCompanionCreativeSize,
    #[serde(rename = "FILTER_VIDEO_CONTINUOUS_PLAY")]
    #[doc = ""]
    FilterVideoContinuousPlay,
    #[serde(rename = "FILTER_VIDEO_DURATION")]
    #[doc = ""]
    FilterVideoDuration,
    #[serde(rename = "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST")]
    #[doc = ""]
    FilterYoutubeAdaptedAudienceList,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO")]
    #[doc = ""]
    FilterYoutubeAdVideo,
    #[serde(rename = "FILTER_YOUTUBE_AD_VIDEO_ID")]
    #[doc = ""]
    FilterYoutubeAdVideoId,
    #[serde(rename = "FILTER_YOUTUBE_CHANNEL")]
    #[doc = ""]
    FilterYoutubeChannel,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedAdvertiser,
    #[serde(rename = "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER")]
    #[doc = ""]
    FilterYoutubeProgrammaticGuaranteedPartner,
    #[serde(rename = "FILTER_YOUTUBE_VIDEO")]
    #[doc = ""]
    FilterYoutubeVideo,
    #[serde(rename = "FILTER_ZIP_POSTAL_CODE")]
    #[doc = ""]
    FilterZipPostalCode,
    #[serde(rename = "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS")]
    #[doc = ""]
    FilterPlacementNameAllYoutubeChannels,
    #[serde(rename = "FILTER_TRUEVIEW_PLACEMENT_ID")]
    #[doc = ""]
    FilterTrueviewPlacementId,
    #[serde(rename = "FILTER_PATH_PATTERN_ID")]
    #[doc = ""]
    FilterPathPatternId,
    #[serde(rename = "FILTER_PATH_EVENT_INDEX")]
    #[doc = ""]
    FilterPathEventIndex,
    #[serde(rename = "FILTER_EVENT_TYPE")]
    #[doc = ""]
    FilterEventType,
    #[serde(rename = "FILTER_CHANNEL_GROUPING")]
    #[doc = ""]
    FilterChannelGrouping,
    #[serde(rename = "FILTER_OM_SDK_AVAILABLE")]
    #[doc = ""]
    FilterOmSdkAvailable,
    #[serde(rename = "FILTER_DATA_SOURCE")]
    #[doc = ""]
    FilterDataSource,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates how the filter should be matched to the value."]
pub enum PathQueryOptionsFilterMatchEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = ""]
    Unknown,
    #[serde(rename = "EXACT")]
    #[doc = ""]
    Exact,
    #[serde(rename = "PARTIAL")]
    #[doc = ""]
    Partial,
    #[serde(rename = "BEGINS_WITH")]
    #[doc = ""]
    BeginsWith,
    #[serde(rename = "WILDCARD_EXPRESSION")]
    #[doc = ""]
    WildcardExpression,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a query."]
pub struct Query {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#query\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query metadata."]
    pub metadata: ::std::option::Option<::std::boxed::Box<QueryMetadata>>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query parameters."]
    pub params: ::std::option::Option<::std::boxed::Box<Parameters>>,
    #[serde(rename = "queryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query ID."]
    pub query_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDataEndTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
    pub report_data_end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDataStartTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
    pub report_data_start_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information on how often and when to run a query."]
    pub schedule: ::std::option::Option<::std::boxed::Box<QuerySchedule>>,
    #[serde(rename = "timezoneCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
    pub timezone_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query metadata."]
pub struct QueryMetadata {
    #[serde(rename = "dataRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Range of report data."]
    pub data_range: ::std::option::Option<QueryMetadataDataRangeEnum>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format of the generated report."]
    pub format: ::std::option::Option<QueryMetadataFormatEnum>,
    #[serde(rename = "googleCloudStoragePathForLatestReport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to the location in Google Cloud Storage where the latest report is stored."]
    pub google_cloud_storage_path_for_latest_report: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleDrivePathForLatestReport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path in Google Drive for the latest report."]
    pub google_drive_path_for_latest_report: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestReportRunTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the latest report started to run."]
    pub latest_report_run_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locale of the generated reports. Valid values are cs CZECH de GERMAN en ENGLISH es SPANISH fr FRENCH it ITALIAN ja JAPANESE ko KOREAN pl POLISH pt-BR BRAZILIAN_PORTUGUESE ru RUSSIAN tr TURKISH uk UKRAINIAN zh-CN CHINA_CHINESE zh-TW TAIWAN_CHINESE An locale string not in the list above will generate reports in English."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of reports that have been generated for the query."]
    pub report_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "running")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the latest report is currently running."]
    pub running: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sendNotification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to send an email notification when a report is ready. Default to false."]
    pub send_notification: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "shareEmailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of email addresses which are sent email notifications when the report is finished. Separate from sendNotification."]
    pub share_email_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query title. It is used to name the reports generated from this query."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Range of report data."]
pub enum QueryMetadataDataRangeEnum {
    #[serde(rename = "CUSTOM_DATES")]
    #[doc = ""]
    CustomDates,
    #[serde(rename = "CURRENT_DAY")]
    #[doc = ""]
    CurrentDay,
    #[serde(rename = "PREVIOUS_DAY")]
    #[doc = ""]
    PreviousDay,
    #[serde(rename = "WEEK_TO_DATE")]
    #[doc = ""]
    WeekToDate,
    #[serde(rename = "MONTH_TO_DATE")]
    #[doc = ""]
    MonthToDate,
    #[serde(rename = "QUARTER_TO_DATE")]
    #[doc = ""]
    QuarterToDate,
    #[serde(rename = "YEAR_TO_DATE")]
    #[doc = ""]
    YearToDate,
    #[serde(rename = "PREVIOUS_WEEK")]
    #[doc = ""]
    PreviousWeek,
    #[serde(rename = "PREVIOUS_HALF_MONTH")]
    #[doc = ""]
    PreviousHalfMonth,
    #[serde(rename = "PREVIOUS_MONTH")]
    #[doc = ""]
    PreviousMonth,
    #[serde(rename = "PREVIOUS_QUARTER")]
    #[doc = ""]
    PreviousQuarter,
    #[serde(rename = "PREVIOUS_YEAR")]
    #[doc = ""]
    PreviousYear,
    #[serde(rename = "LAST_7_DAYS")]
    #[doc = ""]
    Last7Days,
    #[serde(rename = "LAST_30_DAYS")]
    #[doc = ""]
    Last30Days,
    #[serde(rename = "LAST_90_DAYS")]
    #[doc = ""]
    Last90Days,
    #[serde(rename = "LAST_365_DAYS")]
    #[doc = ""]
    Last365Days,
    #[serde(rename = "ALL_TIME")]
    #[doc = ""]
    AllTime,
    #[serde(rename = "LAST_14_DAYS")]
    #[doc = ""]
    Last14Days,
    #[serde(rename = "TYPE_NOT_SUPPORTED")]
    #[doc = ""]
    TypeNotSupported,
    #[serde(rename = "LAST_60_DAYS")]
    #[doc = ""]
    Last60Days,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Format of the generated report."]
pub enum QueryMetadataFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
    #[serde(rename = "EXCEL_CSV")]
    #[doc = ""]
    ExcelCsv,
    #[serde(rename = "XLSX")]
    #[doc = ""]
    Xlsx,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information on how frequently and when to run a query."]
pub struct QuerySchedule {
    #[serde(rename = "endTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Datetime to periodically run the query until."]
    pub end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How often the query is run."]
    pub frequency: ::std::option::Option<QueryScheduleFrequencyEnum>,
    #[serde(rename = "nextRunMinuteOfDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports."]
    pub next_run_minute_of_day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nextRunTimezoneCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonical timezone code for report generation time. Defaults to America/New_York."]
    pub next_run_timezone_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When to start running the query. Not applicable to `ONE_TIME` frequency."]
    pub start_time_ms: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How often the query is run."]
pub enum QueryScheduleFrequencyEnum {
    #[serde(rename = "ONE_TIME")]
    #[doc = ""]
    OneTime,
    #[serde(rename = "DAILY")]
    #[doc = ""]
    Daily,
    #[serde(rename = "WEEKLY")]
    #[doc = ""]
    Weekly,
    #[serde(rename = "SEMI_MONTHLY")]
    #[doc = ""]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    #[doc = ""]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    #[doc = ""]
    Quarterly,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a report."]
pub struct Report {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key used to identify a report."]
    pub key: ::std::option::Option<::std::boxed::Box<ReportKey>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report metadata."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ReportMetadata>>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report parameters."]
    pub params: ::std::option::Option<::std::boxed::Box<Parameters>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An explanation of a report failure."]
pub struct ReportFailure {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error code that shows why the report was not created."]
    pub error_code: ::std::option::Option<ReportFailureErrorCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Error code that shows why the report was not created."]
pub enum ReportFailureErrorCodeEnum {
    #[serde(rename = "AUTHENTICATION_ERROR")]
    #[doc = ""]
    AuthenticationError,
    #[serde(rename = "UNAUTHORIZED_API_ACCESS")]
    #[doc = ""]
    UnauthorizedApiAccess,
    #[serde(rename = "SERVER_ERROR")]
    #[doc = ""]
    ServerError,
    #[serde(rename = "VALIDATION_ERROR")]
    #[doc = ""]
    ValidationError,
    #[serde(rename = "REPORTING_FATAL_ERROR")]
    #[doc = ""]
    ReportingFatalError,
    #[serde(rename = "REPORTING_TRANSIENT_ERROR")]
    #[doc = ""]
    ReportingTransientError,
    #[serde(rename = "REPORTING_IMCOMPATIBLE_METRICS")]
    #[doc = ""]
    ReportingImcompatibleMetrics,
    #[serde(rename = "REPORTING_ILLEGAL_FILENAME")]
    #[doc = ""]
    ReportingIllegalFilename,
    #[serde(rename = "REPORTING_QUERY_NOT_FOUND")]
    #[doc = ""]
    ReportingQueryNotFound,
    #[serde(rename = "REPORTING_BUCKET_NOT_FOUND")]
    #[doc = ""]
    ReportingBucketNotFound,
    #[serde(rename = "REPORTING_CREATE_BUCKET_FAILED")]
    #[doc = ""]
    ReportingCreateBucketFailed,
    #[serde(rename = "REPORTING_DELETE_BUCKET_FAILED")]
    #[doc = ""]
    ReportingDeleteBucketFailed,
    #[serde(rename = "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED")]
    #[doc = ""]
    ReportingUpdateBucketPermissionFailed,
    #[serde(rename = "REPORTING_WRITE_BUCKET_OBJECT_FAILED")]
    #[doc = ""]
    ReportingWriteBucketObjectFailed,
    #[serde(rename = "DEPRECATED_REPORTING_INVALID_QUERY")]
    #[doc = ""]
    DeprecatedReportingInvalidQuery,
    #[serde(rename = "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS")]
    #[doc = ""]
    ReportingInvalidQueryTooManyUnfilteredLargeGroupBys,
    #[serde(rename = "REPORTING_INVALID_QUERY_TITLE_MISSING")]
    #[doc = ""]
    ReportingInvalidQueryTitleMissing,
    #[serde(rename = "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS")]
    #[doc = ""]
    ReportingInvalidQueryMissingPartnerAndAdvertiserFilters,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Key used to identify a report."]
pub struct ReportKey {
    #[serde(rename = "queryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query ID."]
    pub query_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report ID."]
    pub report_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Report metadata."]
pub struct ReportMetadata {
    #[serde(rename = "googleCloudStoragePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to the location in Google Cloud Storage where the report is stored."]
    pub google_cloud_storage_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDataEndTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ending time for the data that is shown in the report."]
    pub report_data_end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDataStartTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting time for the data that is shown in the report."]
    pub report_data_start_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report status."]
    pub status: ::std::option::Option<::std::boxed::Box<ReportStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Report status."]
pub struct ReportStatus {
    #[serde(rename = "failure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the report failed, this records the cause."]
    pub failure: ::std::option::Option<::std::boxed::Box<ReportFailure>>,
    #[serde(rename = "finishTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when this report either completed successfully or failed."]
    pub finish_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file type of the report."]
    pub format: ::std::option::Option<ReportStatusFormatEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the report."]
    pub state: ::std::option::Option<ReportStatusStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The file type of the report."]
pub enum ReportStatusFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
    #[serde(rename = "EXCEL_CSV")]
    #[doc = ""]
    ExcelCsv,
    #[serde(rename = "XLSX")]
    #[doc = ""]
    Xlsx,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the report."]
pub enum ReportStatusStateEnum {
    #[serde(rename = "RUNNING")]
    #[doc = ""]
    Running,
    #[serde(rename = "DONE")]
    #[doc = ""]
    Done,
    #[serde(rename = "FAILED")]
    #[doc = ""]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the upload status of a row in the request."]
pub struct RowStatus {
    #[serde(rename = "changed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the stored entity is changed as a result of upload."]
    pub changed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity Id."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity name."]
    pub entity_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reasons why the entity can't be uploaded."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "persisted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the entity is persisted."]
    pub persisted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rowNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Row number."]
    pub row_number: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Rule defines a name, and a boolean expression in [conjunctive normal form](http: //mathworld.wolfram.com/ConjunctiveNormalForm.html){.external} that can be // applied to a path event to determine if that name should be applied."]
pub struct Rule {
    #[serde(rename = "disjunctiveMatchStatements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disjunctive_match_statements:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DisjunctiveMatchStatement>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rule name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to run a stored query to generate a report."]
pub struct RunQueryRequest {
    #[serde(rename = "dataRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report data range used to generate the report."]
    pub data_range: ::std::option::Option<RunQueryRequestDataRangeEnum>,
    #[serde(rename = "reportDataEndTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
    pub report_data_end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDataStartTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
    pub report_data_start_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timezoneCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
    pub timezone_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Report data range used to generate the report."]
pub enum RunQueryRequestDataRangeEnum {
    #[serde(rename = "CUSTOM_DATES")]
    #[doc = ""]
    CustomDates,
    #[serde(rename = "CURRENT_DAY")]
    #[doc = ""]
    CurrentDay,
    #[serde(rename = "PREVIOUS_DAY")]
    #[doc = ""]
    PreviousDay,
    #[serde(rename = "WEEK_TO_DATE")]
    #[doc = ""]
    WeekToDate,
    #[serde(rename = "MONTH_TO_DATE")]
    #[doc = ""]
    MonthToDate,
    #[serde(rename = "QUARTER_TO_DATE")]
    #[doc = ""]
    QuarterToDate,
    #[serde(rename = "YEAR_TO_DATE")]
    #[doc = ""]
    YearToDate,
    #[serde(rename = "PREVIOUS_WEEK")]
    #[doc = ""]
    PreviousWeek,
    #[serde(rename = "PREVIOUS_HALF_MONTH")]
    #[doc = ""]
    PreviousHalfMonth,
    #[serde(rename = "PREVIOUS_MONTH")]
    #[doc = ""]
    PreviousMonth,
    #[serde(rename = "PREVIOUS_QUARTER")]
    #[doc = ""]
    PreviousQuarter,
    #[serde(rename = "PREVIOUS_YEAR")]
    #[doc = ""]
    PreviousYear,
    #[serde(rename = "LAST_7_DAYS")]
    #[doc = ""]
    Last7Days,
    #[serde(rename = "LAST_30_DAYS")]
    #[doc = ""]
    Last30Days,
    #[serde(rename = "LAST_90_DAYS")]
    #[doc = ""]
    Last90Days,
    #[serde(rename = "LAST_365_DAYS")]
    #[doc = ""]
    Last365Days,
    #[serde(rename = "ALL_TIME")]
    #[doc = ""]
    AllTime,
    #[serde(rename = "LAST_14_DAYS")]
    #[doc = ""]
    Last14Days,
    #[serde(rename = "TYPE_NOT_SUPPORTED")]
    #[doc = ""]
    TypeNotSupported,
    #[serde(rename = "LAST_60_DAYS")]
    #[doc = ""]
    Last60Days,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to upload line items."]
pub struct UploadLineItemsRequest {
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to get upload status without actually persisting the line items."]
    pub dry_run: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format the line items are in. Default to CSV."]
    pub format: ::std::option::Option<UploadLineItemsRequestFormatEnum>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line items in CSV to upload. Refer to Entity Write File Format for more information on file format."]
    pub line_items: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Format the line items are in. Default to CSV."]
pub enum UploadLineItemsRequestFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Upload line items response."]
pub struct UploadLineItemsResponse {
    #[serde(rename = "uploadStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of upload."]
    pub upload_status: ::std::option::Option<::std::boxed::Box<UploadStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the status of upload."]
pub struct UploadStatus {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reasons why upload can't be completed."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "rowStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Per-row upload status."]
    pub row_status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RowStatus>>>,
}
