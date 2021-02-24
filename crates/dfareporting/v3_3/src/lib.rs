#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Campaign Manager account."]
pub struct Account {
    #[serde(rename = "accountPermissionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account permissions assigned to this account."]
    pub account_permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "accountProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Profile for this account. This is a read-only field that can be left blank."]
    pub account_profile: ::std::option::Option<AccountAccountProfileEnum>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this account is active."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "activeAdsLimitTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of active ads allowed for this account."]
    pub active_ads_limit_tier: ::std::option::Option<AccountActiveAdsLimitTierEnum>,
    #[serde(rename = "activeViewOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to serve creatives with Active View tags. If disabled, viewability data will not be available for any impressions."]
    pub active_view_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "availablePermissionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User role permissions available to the user roles of this account."]
    pub available_permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "countryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the country associated with this account."]
    pub country_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of currency associated with this account. This is a required field. Acceptable values are: - \"1\" for USD - \"2\" for GBP - \"3\" for ESP - \"4\" for SEK - \"5\" for CAD - \"6\" for JPY - \"7\" for DEM - \"8\" for AUD - \"9\" for FRF - \"10\" for ITL - \"11\" for DKK - \"12\" for NOK - \"13\" for FIM - \"14\" for ZAR - \"15\" for IEP - \"16\" for NLG - \"17\" for EUR - \"18\" for KRW - \"19\" for TWD - \"20\" for SGD - \"21\" for CNY - \"22\" for HKD - \"23\" for NZD - \"24\" for MYR - \"25\" for BRL - \"26\" for PTE - \"28\" for CLP - \"29\" for TRY - \"30\" for ARS - \"31\" for PEN - \"32\" for ILS - \"33\" for CHF - \"34\" for VEF - \"35\" for COP - \"36\" for GTQ - \"37\" for PLN - \"39\" for INR - \"40\" for THB - \"41\" for IDR - \"42\" for CZK - \"43\" for RON - \"44\" for HUF - \"45\" for RUB - \"46\" for AED - \"47\" for BGN - \"48\" for HRK - \"49\" for MXN - \"50\" for NGN - \"51\" for EGP "]
    pub currency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultCreativeSizeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default placement dimensions for this account."]
    pub default_creative_size_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of this account."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this account. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#account\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locale of this account. Acceptable values are: - \"cs\" (Czech) - \"de\" (German) - \"en\" (English) - \"en-GB\" (English United Kingdom) - \"es\" (Spanish) - \"fr\" (French) - \"it\" (Italian) - \"ja\" (Japanese) - \"ko\" (Korean) - \"pl\" (Polish) - \"pt-BR\" (Portuguese Brazil) - \"ru\" (Russian) - \"sv\" (Swedish) - \"tr\" (Turkish) - \"zh-CN\" (Chinese Simplified) - \"zh-TW\" (Chinese Traditional) "]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maximumImageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum image size allowed for this account, in kilobytes. Value must be greater than or equal to 1."]
    pub maximum_image_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this account. This is a required field, and must be less than 128 characters long and be globally unique."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nielsenOcrEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether campaigns created in this account will be enabled for Nielsen OCR reach ratings by default."]
    pub nielsen_ocr_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reportsConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reporting configuration of this account."]
    pub reports_configuration: ::std::option::Option<::std::boxed::Box<ReportsConfiguration>>,
    #[serde(rename = "shareReportsWithTwitter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Share Path to Conversion reports with Twitter."]
    pub share_reports_with_twitter: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "teaserSizeLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File size limit in kilobytes of Rich Media teaser creatives. Acceptable values are 1 to 10240, inclusive."]
    pub teaser_size_limit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Profile for this account. This is a read-only field that can be left blank."]
pub enum AccountAccountProfileEnum {
    #[serde(rename = "ACCOUNT_PROFILE_BASIC")]
    #[doc = ""]
    AccountProfileBasic,
    #[serde(rename = "ACCOUNT_PROFILE_STANDARD")]
    #[doc = ""]
    AccountProfileStandard,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Maximum number of active ads allowed for this account."]
pub enum AccountActiveAdsLimitTierEnum {
    #[serde(rename = "ACTIVE_ADS_TIER_40K")]
    #[doc = ""]
    ActiveAdsTier40K,
    #[serde(rename = "ACTIVE_ADS_TIER_75K")]
    #[doc = ""]
    ActiveAdsTier75K,
    #[serde(rename = "ACTIVE_ADS_TIER_100K")]
    #[doc = ""]
    ActiveAdsTier100K,
    #[serde(rename = "ACTIVE_ADS_TIER_200K")]
    #[doc = ""]
    ActiveAdsTier200K,
    #[serde(rename = "ACTIVE_ADS_TIER_300K")]
    #[doc = ""]
    ActiveAdsTier300K,
    #[serde(rename = "ACTIVE_ADS_TIER_500K")]
    #[doc = ""]
    ActiveAdsTier500K,
    #[serde(rename = "ACTIVE_ADS_TIER_750K")]
    #[doc = ""]
    ActiveAdsTier750K,
    #[serde(rename = "ACTIVE_ADS_TIER_1M")]
    #[doc = ""]
    ActiveAdsTier1M,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Gets a summary of active ads in an account."]
pub struct AccountActiveAdSummary {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the account."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "activeAds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ads that have been activated for the account"]
    pub active_ads: ::std::option::Option<::std::string::String>,
    #[serde(rename = "activeAdsLimitTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of active ads allowed for the account."]
    pub active_ads_limit_tier: ::std::option::Option<AccountActiveAdSummaryActiveAdsLimitTierEnum>,
    #[serde(rename = "availableAds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ads that can be activated for the account."]
    pub available_ads: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountActiveAdSummary\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Maximum number of active ads allowed for the account."]
pub enum AccountActiveAdSummaryActiveAdsLimitTierEnum {
    #[serde(rename = "ACTIVE_ADS_TIER_40K")]
    #[doc = ""]
    ActiveAdsTier40K,
    #[serde(rename = "ACTIVE_ADS_TIER_75K")]
    #[doc = ""]
    ActiveAdsTier75K,
    #[serde(rename = "ACTIVE_ADS_TIER_100K")]
    #[doc = ""]
    ActiveAdsTier100K,
    #[serde(rename = "ACTIVE_ADS_TIER_200K")]
    #[doc = ""]
    ActiveAdsTier200K,
    #[serde(rename = "ACTIVE_ADS_TIER_300K")]
    #[doc = ""]
    ActiveAdsTier300K,
    #[serde(rename = "ACTIVE_ADS_TIER_500K")]
    #[doc = ""]
    ActiveAdsTier500K,
    #[serde(rename = "ACTIVE_ADS_TIER_750K")]
    #[doc = ""]
    ActiveAdsTier750K,
    #[serde(rename = "ACTIVE_ADS_TIER_1M")]
    #[doc = ""]
    ActiveAdsTier1M,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AccountPermissions contains information about a particular account permission. Some features of Campaign Manager require an account permission to be present in the account."]
pub struct AccountPermission {
    #[serde(rename = "accountProfiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account profiles associated with this account permission. Possible values are: - \"ACCOUNT_PROFILE_BASIC\" - \"ACCOUNT_PROFILE_STANDARD\" "]
    pub account_profiles:
        ::std::option::Option<::std::vec::Vec<AccountPermissionAccountProfilesEnum>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this account permission."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountPermission\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Administrative level required to enable this account permission."]
    pub level: ::std::option::Option<AccountPermissionLevelEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this account permission."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Permission group of this account permission."]
    pub permission_group_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AccountPermissionAccountProfilesEnum {
    #[serde(rename = "ACCOUNT_PROFILE_BASIC")]
    #[doc = ""]
    AccountProfileBasic,
    #[serde(rename = "ACCOUNT_PROFILE_STANDARD")]
    #[doc = ""]
    AccountProfileStandard,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Administrative level required to enable this account permission."]
pub enum AccountPermissionLevelEnum {
    #[serde(rename = "USER")]
    #[doc = ""]
    User,
    #[serde(rename = "ADMINISTRATOR")]
    #[doc = ""]
    Administrator,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AccountPermissionGroups contains a mapping of permission group IDs to names. A permission group is a grouping of account permissions."]
pub struct AccountPermissionGroup {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this account permission group."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountPermissionGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this account permission group."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account Permission Group List Response"]
pub struct AccountPermissionGroupsListResponse {
    #[serde(rename = "accountPermissionGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account permission group collection."]
    pub account_permission_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountPermissionGroup>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountPermissionGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account Permission List Response"]
pub struct AccountPermissionsListResponse {
    #[serde(rename = "accountPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account permission collection."]
    pub account_permissions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountPermission>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountPermissionsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AccountUserProfiles contains properties of a Campaign Manager user profile. This resource is specifically for managing user profiles, whereas UserProfiles is for accessing the API."]
pub struct AccountUserProfile {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of the user profile. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this user profile is active. This defaults to false, and must be set true on insert for the user profile to be usable."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "advertiserFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter that describes which advertisers are visible to the user profile."]
    pub advertiser_filter: ::std::option::Option<::std::boxed::Box<ObjectFilter>>,
    #[serde(rename = "campaignFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter that describes which campaigns are visible to the user profile."]
    pub campaign_filter: ::std::option::Option<::std::boxed::Box<ObjectFilter>>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments for this user profile."]
    pub comments: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email of the user profile. The email addresss must be linked to a Google Account. This field is required on insertion and is read-only after insertion."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user profile. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountUserProfile\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locale of the user profile. This is a required field. Acceptable values are: - \"cs\" (Czech) - \"de\" (German) - \"en\" (English) - \"en-GB\" (English United Kingdom) - \"es\" (Spanish) - \"fr\" (French) - \"it\" (Italian) - \"ja\" (Japanese) - \"ko\" (Korean) - \"pl\" (Polish) - \"pt-BR\" (Portuguese Brazil) - \"ru\" (Russian) - \"sv\" (Swedish) - \"tr\" (Turkish) - \"zh-CN\" (Chinese Simplified) - \"zh-TW\" (Chinese Traditional) "]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the user profile. This is a required field. Must be less than 64 characters long, must be globally unique, and cannot contain whitespace or any of the following characters: \"&;<>\"#%,\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter that describes which sites are visible to the user profile."]
    pub site_filter: ::std::option::Option<::std::boxed::Box<ObjectFilter>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of the user profile. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traffickerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trafficker type of this user profile. This is a read-only field."]
    pub trafficker_type: ::std::option::Option<AccountUserProfileTraffickerTypeEnum>,
    #[serde(rename = "userAccessType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User type of the user profile. This is a read-only field that can be left blank."]
    pub user_access_type: ::std::option::Option<AccountUserProfileUserAccessTypeEnum>,
    #[serde(rename = "userRoleFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter that describes which user roles are visible to the user profile."]
    pub user_role_filter: ::std::option::Option<::std::boxed::Box<ObjectFilter>>,
    #[serde(rename = "userRoleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User role ID of the user profile. This is a required field."]
    pub user_role_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Trafficker type of this user profile. This is a read-only field."]
pub enum AccountUserProfileTraffickerTypeEnum {
    #[serde(rename = "INTERNAL_NON_TRAFFICKER")]
    #[doc = ""]
    InternalNonTrafficker,
    #[serde(rename = "INTERNAL_TRAFFICKER")]
    #[doc = ""]
    InternalTrafficker,
    #[serde(rename = "EXTERNAL_TRAFFICKER")]
    #[doc = ""]
    ExternalTrafficker,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "User type of the user profile. This is a read-only field that can be left blank."]
pub enum AccountUserProfileUserAccessTypeEnum {
    #[serde(rename = "NORMAL_USER")]
    #[doc = ""]
    NormalUser,
    #[serde(rename = "SUPER_USER")]
    #[doc = ""]
    SuperUser,
    #[serde(rename = "INTERNAL_ADMINISTRATOR")]
    #[doc = ""]
    InternalAdministrator,
    #[serde(rename = "READ_ONLY_SUPER_USER")]
    #[doc = ""]
    ReadOnlySuperUser,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account User Profile List Response"]
pub struct AccountUserProfilesListResponse {
    #[serde(rename = "accountUserProfiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account user profile collection."]
    pub account_user_profiles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountUserProfile>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountUserProfilesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account List Response"]
pub struct AccountsListResponse {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account collection."]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#accountsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an activity group."]
pub struct Activities {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of activity filters. The dimension values need to be all either of type \"dfa:activity\" or \"dfa:activityGroup\"."]
    pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#activities."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of names of floodlight activity metrics."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Campaign Manager ad."]
pub struct Ad {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this ad. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad is active. When true, archived must be false."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this ad. This is a required field on insertion."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad is archived. When true, active must be false."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audienceSegmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience segment ID that is being targeted for this ad. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub audience_segment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign ID of this ad. This is a required field on insertion."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the campaign. This is a read-only, auto-generated field."]
    pub campaign_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL for this ad. This is a required field on insertion. Applicable when type is AD_SERVING_CLICK_TRACKER."]
    pub click_through_url: ::std::option::Option<::std::boxed::Box<ClickThroughUrl>>,
    #[serde(rename = "clickThroughUrlSuffixProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL suffix properties for this ad. Applies to the URL in the ad or (if overriding ad properties) the URL in the creative."]
    pub click_through_url_suffix_properties:
        ::std::option::Option<::std::boxed::Box<ClickThroughUrlSuffixProperties>>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments for this ad."]
    pub comments: ::std::option::Option<::std::string::String>,
    #[serde(rename = "compatibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compatibility of this ad. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to either rendering on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are only used for existing default ads. New mobile placements must be assigned DISPLAY or DISPLAY_INTERSTITIAL and default ads created for those placements will be limited to those compatibility types. IN_STREAM_VIDEO refers to rendering in-stream video ads developed with the VAST standard."]
    pub compatibility: ::std::option::Option<AdCompatibilityEnum>,
    #[serde(rename = "createInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the creation of this ad. This is a read-only field."]
    pub create_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "creativeGroupAssignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative group assignments for this ad. Applicable when type is AD_SERVING_CLICK_TRACKER. Only one assignment per creative group number is allowed for a maximum of two assignments."]
    pub creative_group_assignments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeGroupAssignment>>>,
    #[serde(rename = "creativeRotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative rotation for this ad. Applicable when type is AD_SERVING_DEFAULT_AD, AD_SERVING_STANDARD_AD, or AD_SERVING_TRACKING. When type is AD_SERVING_DEFAULT_AD, this field should have exactly one creativeAssignment ."]
    pub creative_rotation: ::std::option::Option<::std::boxed::Box<CreativeRotation>>,
    #[serde(rename = "dayPartTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time and day targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub day_part_targeting: ::std::option::Option<::std::boxed::Box<DayPartTargeting>>,
    #[serde(rename = "defaultClickThroughEventTagProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default click-through event tag properties for this ad."]
    pub default_click_through_event_tag_properties:
        ::std::option::Option<::std::boxed::Box<DefaultClickThroughEventTagProperties>>,
    #[serde(rename = "deliverySchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delivery schedule information for this ad. Applicable when type is AD_SERVING_STANDARD_AD or AD_SERVING_TRACKING. This field along with subfields priority and impressionRatio are required on insertion when type is AD_SERVING_STANDARD_AD."]
    pub delivery_schedule: ::std::option::Option<::std::boxed::Box<DeliverySchedule>>,
    #[serde(rename = "dynamicClickTracker")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad is a dynamic click tracker. Applicable when type is AD_SERVING_CLICK_TRACKER. This is a required field on insert, and is read-only after insert."]
    pub dynamic_click_tracker: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventTagOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Event tag overrides for this ad."]
    pub event_tag_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventTagOverride>>>,
    #[serde(rename = "geoTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographical targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub geo_targeting: ::std::option::Option<::std::boxed::Box<GeoTargeting>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this ad. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this ad. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "keyValueTargetingExpression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key-value targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub key_value_targeting_expression:
        ::std::option::Option<::std::boxed::Box<KeyValueTargetingExpression>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#ad\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub language_targeting: ::std::option::Option<::std::boxed::Box<LanguageTargeting>>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this ad. This is a read-only field."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this ad. This is a required field and must be less than 256 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementAssignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement assignments for this ad."]
    pub placement_assignments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlacementAssignment>>>,
    #[serde(rename = "remarketingListExpression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list targeting expression for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub remarketing_list_expression:
        ::std::option::Option<::std::boxed::Box<ListTargetingExpression>>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of this ad. Applicable when type is AD_SERVING_DEFAULT_AD."]
    pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad is ssl compliant. This is a read-only field that is auto-generated when the ad is inserted or updated."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sslRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad requires ssl. This is a read-only field that is auto-generated when the ad is inserted or updated."]
    pub ssl_required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this ad. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingTemplateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targeting template ID, used to apply preconfigured targeting information to this ad. This cannot be set while any of dayPartTargeting, geoTargeting, keyValueTargetingExpression, languageTargeting, remarketingListExpression, or technologyTargeting are set. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub targeting_template_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "technologyTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Technology platform targeting information for this ad. This field must be left blank if the ad is using a targeting template. Applicable when type is AD_SERVING_STANDARD_AD."]
    pub technology_targeting: ::std::option::Option<::std::boxed::Box<TechnologyTargeting>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of ad. This is a required field on insertion. Note that default ads ( AD_SERVING_DEFAULT_AD) cannot be created directly (see Creative resource)."]
    pub _type: ::std::option::Option<AdTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Compatibility of this ad. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to either rendering on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are only used for existing default ads. New mobile placements must be assigned DISPLAY or DISPLAY_INTERSTITIAL and default ads created for those placements will be limited to those compatibility types. IN_STREAM_VIDEO refers to rendering in-stream video ads developed with the VAST standard."]
pub enum AdCompatibilityEnum {
    #[serde(rename = "DISPLAY")]
    #[doc = ""]
    Display,
    #[serde(rename = "DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    DisplayInterstitial,
    #[serde(rename = "APP")]
    #[doc = ""]
    App,
    #[serde(rename = "APP_INTERSTITIAL")]
    #[doc = ""]
    AppInterstitial,
    #[serde(rename = "IN_STREAM_VIDEO")]
    #[doc = ""]
    InStreamVideo,
    #[serde(rename = "IN_STREAM_AUDIO")]
    #[doc = ""]
    InStreamAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of ad. This is a required field on insertion. Note that default ads ( AD_SERVING_DEFAULT_AD) cannot be created directly (see Creative resource)."]
pub enum AdTypeEnum {
    #[serde(rename = "AD_SERVING_STANDARD_AD")]
    #[doc = ""]
    AdServingStandardAd,
    #[serde(rename = "AD_SERVING_DEFAULT_AD")]
    #[doc = ""]
    AdServingDefaultAd,
    #[serde(rename = "AD_SERVING_CLICK_TRACKER")]
    #[doc = ""]
    AdServingClickTracker,
    #[serde(rename = "AD_SERVING_TRACKING")]
    #[doc = ""]
    AdServingTracking,
    #[serde(rename = "AD_SERVING_BRAND_SAFE_AD")]
    #[doc = ""]
    AdServingBrandSafeAd,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Campaign ad blocking settings."]
pub struct AdBlockingConfiguration {
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL used by brand-neutral ads. This is a required field when overrideClickThroughUrl is set to true."]
    pub click_through_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeBundleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of a creative bundle to use for this campaign. If set, brand-neutral ads will select creatives from this bundle. Otherwise, a default transparent pixel will be used."]
    pub creative_bundle_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this campaign has enabled ad blocking. When true, ad blocking is enabled for placements in the campaign, but this may be overridden by site and placement settings. When false, ad blocking is disabled for all placements under the campaign, regardless of site and placement settings."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "overrideClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the brand-neutral ad's click-through URL comes from the campaign's creative bundle or the override URL. Must be set to true if ad blocking is enabled and no creative bundle is configured."]
    pub override_click_through_url: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ad Slot"]
pub struct AdSlot {
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comment for this ad slot."]
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "compatibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad slot compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop, mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard."]
    pub compatibility: ::std::option::Option<AdSlotCompatibilityEnum>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height of this ad slot."]
    pub height: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkedPlacementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the placement from an external platform that is linked to this ad slot."]
    pub linked_placement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this ad slot."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paymentSourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payment source type of this ad slot."]
    pub payment_source_type: ::std::option::Option<AdSlotPaymentSourceTypeEnum>,
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Primary ad slot of a roadblock inventory item."]
    pub primary: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of this ad slot."]
    pub width: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Ad slot compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop, mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard."]
pub enum AdSlotCompatibilityEnum {
    #[serde(rename = "DISPLAY")]
    #[doc = ""]
    Display,
    #[serde(rename = "DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    DisplayInterstitial,
    #[serde(rename = "APP")]
    #[doc = ""]
    App,
    #[serde(rename = "APP_INTERSTITIAL")]
    #[doc = ""]
    AppInterstitial,
    #[serde(rename = "IN_STREAM_VIDEO")]
    #[doc = ""]
    InStreamVideo,
    #[serde(rename = "IN_STREAM_AUDIO")]
    #[doc = ""]
    InStreamAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Payment source type of this ad slot."]
pub enum AdSlotPaymentSourceTypeEnum {
    #[serde(rename = "PLANNING_PAYMENT_SOURCE_TYPE_AGENCY_PAID")]
    #[doc = ""]
    PlanningPaymentSourceTypeAgencyPaid,
    #[serde(rename = "PLANNING_PAYMENT_SOURCE_TYPE_PUBLISHER_PAID")]
    #[doc = ""]
    PlanningPaymentSourceTypePublisherPaid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ad List Response"]
pub struct AdsListResponse {
    #[serde(rename = "ads")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad collection."]
    pub ads: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Ad>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#adsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Campaign Manager advertiser."]
pub struct Advertiser {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this advertiser.This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the advertiser group this advertiser belongs to. You can group advertisers for reporting purposes, allowing you to see aggregated information for all advertisers in each group."]
    pub advertiser_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickThroughUrlSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suffix added to click-through URL of ad creative associations under this advertiser. Must be less than 129 characters long."]
    pub click_through_url_suffix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultClickThroughEventTagId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the click-through event tag to apply by default to the landing pages of this advertiser's campaigns."]
    pub default_click_through_event_tag_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default email address used in sender field for tag emails."]
    pub default_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight configuration ID of this advertiser. The floodlight configuration ID will be created automatically, so on insert this field should be left blank. This field can be set to another advertiser's floodlight configuration ID in order to share that advertiser's floodlight configuration with this advertiser, so long as: - This advertiser's original floodlight configuration is not already associated with floodlight activities or floodlight activity groups. - This advertiser's original floodlight configuration is not already shared with another advertiser. "]
    pub floodlight_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightConfigurationIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field."]
    pub floodlight_configuration_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this advertiser. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this advertiser. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#advertiser\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this advertiser. This is a required field and must be less than 256 characters long and unique among advertisers of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalFloodlightConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Original floodlight configuration before any sharing occurred. Set the floodlightConfigurationId of this advertiser to originalFloodlightConfigurationId to unshare the advertiser's current floodlight configuration. You cannot unshare an advertiser's floodlight configuration if the shared configuration has activities associated with any campaign or placement."]
    pub original_floodlight_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this advertiser."]
    pub status: ::std::option::Option<AdvertiserStatusEnum>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this advertiser.This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suspended")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suspension status of this advertiser."]
    pub suspended: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of this advertiser."]
pub enum AdvertiserStatusEnum {
    #[serde(rename = "APPROVED")]
    #[doc = ""]
    Approved,
    #[serde(rename = "ON_HOLD")]
    #[doc = ""]
    OnHold,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Groups advertisers together so that reports can be generated for the entire group at once."]
pub struct AdvertiserGroup {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this advertiser group. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this advertiser group. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#advertiserGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this advertiser group. This is a required field and must be less than 256 characters long and unique among advertiser groups of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Advertiser Group List Response"]
pub struct AdvertiserGroupsListResponse {
    #[serde(rename = "advertiserGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser group collection."]
    pub advertiser_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdvertiserGroup>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#advertiserGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Landing Page List Response"]
pub struct AdvertiserLandingPagesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#advertiserLandingPagesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "landingPages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Landing page collection"]
    pub landing_pages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LandingPage>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Advertiser List Response"]
pub struct AdvertisersListResponse {
    #[serde(rename = "advertisers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser collection."]
    pub advertisers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Advertiser>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#advertisersListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Audience Segment."]
pub struct AudienceSegment {
    #[serde(rename = "allocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Weight allocated to this segment. The weight assigned will be understood in proportion to the weights assigned to other segments in the same segment group. Acceptable values are 1 to 1000, inclusive."]
    pub allocation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this audience segment. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this audience segment. This is a required field and must be less than 65 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Audience Segment Group."]
pub struct AudienceSegmentGroup {
    #[serde(rename = "audienceSegments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience segments assigned to this group. The number of segments must be between 2 and 100."]
    pub audience_segments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AudienceSegment>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this audience segment group. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this audience segment group. This is a required field and must be less than 65 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a browser that can be targeted by ads."]
pub struct Browser {
    #[serde(rename = "browserVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID referring to this grouping of browser and version numbers. This is the ID used for targeting."]
    pub browser_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this browser. This is the ID used when generating reports."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#browser\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "majorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Major version number (leftmost number) of this browser. For example, for Chrome 5.0.376.86 beta, this field should be set to 5. An asterisk (*) may be used to target any version number, and a question mark (?) may be used to target cases where the version number cannot be identified. For example, Chrome *.* targets any version of Chrome: 1.2, 2.5, 3.5, and so on. Chrome 3.* targets Chrome 3.1, 3.5, but not 4.0. Firefox ?.? targets cases where the ad server knows the browser is Firefox but can't tell which version it is."]
    pub major_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minor version number (number after first dot on left) of this browser. For example, for Chrome 5.0.375.86 beta, this field should be set to 0. An asterisk (*) may be used to target any version number, and a question mark (?) may be used to target cases where the version number cannot be identified. For example, Chrome *.* targets any version of Chrome: 1.2, 2.5, 3.5, and so on. Chrome 3.* targets Chrome 3.1, 3.5, but not 4.0. Firefox ?.? targets cases where the ad server knows the browser is Firefox but can't tell which version it is."]
    pub minor_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this browser."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Browser List Response"]
pub struct BrowsersListResponse {
    #[serde(rename = "browsers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browser collection."]
    pub browsers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Browser>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#browsersListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Campaign Manager campaign."]
pub struct Campaign {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this campaign. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adBlockingConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad blocking settings for this campaign."]
    pub ad_blocking_configuration:
        ::std::option::Option<::std::boxed::Box<AdBlockingConfiguration>>,
    #[serde(rename = "additionalCreativeOptimizationConfigurations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional creative optimization configurations for the campaign."]
    pub additional_creative_optimization_configurations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<CreativeOptimizationConfiguration>>,
    >,
    #[serde(rename = "advertiserGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser group ID of the associated advertiser."]
    pub advertiser_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this campaign. This is a required field."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the advertiser ID of this campaign. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this campaign has been archived."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "audienceSegmentGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience segment groups assigned to this campaign. Cannot have more than 300 segment groups."]
    pub audience_segment_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AudienceSegmentGroup>>>,
    #[serde(rename = "billingInvoiceCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Billing invoice code included in the Campaign Manager client billing invoices associated with the campaign."]
    pub billing_invoice_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickThroughUrlSuffixProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL suffix override properties for this campaign."]
    pub click_through_url_suffix_properties:
        ::std::option::Option<::std::boxed::Box<ClickThroughUrlSuffixProperties>>,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Arbitrary comments about this campaign. Must be less than 256 characters long."]
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the creation of this campaign. This is a read-only field."]
    pub create_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "creativeGroupIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of creative group IDs that are assigned to the campaign."]
    pub creative_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "creativeOptimizationConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative optimization configuration for the campaign."]
    pub creative_optimization_configuration:
        ::std::option::Option<::std::boxed::Box<CreativeOptimizationConfiguration>>,
    #[serde(rename = "defaultClickThroughEventTagProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through event tag ID override properties for this campaign."]
    pub default_click_through_event_tag_properties:
        ::std::option::Option<::std::boxed::Box<DefaultClickThroughEventTagProperties>>,
    #[serde(rename = "defaultLandingPageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default landing page ID for this campaign."]
    pub default_landing_page_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventTagOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overrides that can be used to activate or deactivate advertiser event tags."]
    pub event_tag_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventTagOverride>>>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External ID for this campaign."]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this campaign. This is a read-only auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this campaign. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#campaign\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this campaign. This is a read-only field."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this campaign. This is a required field and must be less than 256 characters long and unique among campaigns of the same advertiser."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nielsenOcrEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Nielsen reports are enabled for this campaign."]
    pub nielsen_ocr_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this campaign. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traffickerEmails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign trafficker contact emails."]
    pub trafficker_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifies a creative which has been associated with a given campaign."]
pub struct CampaignCreativeAssociation {
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative associated with the campaign. This is a required field."]
    pub creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#campaignCreativeAssociation\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Campaign Creative Association List Response"]
pub struct CampaignCreativeAssociationsListResponse {
    #[serde(rename = "campaignCreativeAssociations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign creative association collection"]
    pub campaign_creative_associations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CampaignCreativeAssociation>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#campaignCreativeAssociationsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Campaign List Response"]
pub struct CampaignsListResponse {
    #[serde(rename = "campaigns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign collection."]
    pub campaigns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Campaign>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#campaignsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a change that a user has made to a resource."]
pub struct ChangeLog {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of the modified object."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Action which caused the change."]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "changeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub change_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field name of the object which changed."]
    pub field_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this change log."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#changeLog\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "newValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "New value of the object field."]
    pub new_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the object of this change log. The object could be a campaign, placement, ad, or other type."]
    pub object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object type of the change log."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oldValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Old value of the object field."]
    pub old_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of the modified object."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transaction ID of this change log. When a single API call results in many changes, each change will have a separate ID in the change log but will share the same transactionId."]
    pub transaction_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user who modified the object."]
    pub user_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userProfileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User profile name of the user who modified the object."]
    pub user_profile_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Change Log List Response"]
pub struct ChangeLogsListResponse {
    #[serde(rename = "changeLogs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Change log collection."]
    pub change_logs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChangeLog>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#changeLogsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "City List Response"]
pub struct CitiesListResponse {
    #[serde(rename = "cities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "City collection."]
    pub cities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<City>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#citiesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a city that can be targeted by ads."]
pub struct City {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code of the country to which this city belongs."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the country to which this city belongs."]
    pub country_dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this city. This is the ID used for targeting and generating reports."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#city\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metroCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metro region code of the metro region (DMA) to which this city belongs."]
    pub metro_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metroDmaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the metro region (DMA) to which this city belongs."]
    pub metro_dma_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this city."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Region code of the region to which this city belongs."]
    pub region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the region to which this city belongs."]
    pub region_dart_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Click Tag."]
pub struct ClickTag {
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter value for the specified click tag. This field contains a click-through url."]
    pub click_through_url: ::std::option::Option<::std::boxed::Box<CreativeClickThroughUrl>>,
    #[serde(rename = "eventName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser event name associated with the click tag. This field is used by DISPLAY_IMAGE_GALLERY and HTML5_BANNER creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub event_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter name for the specified click tag. For DISPLAY_IMAGE_GALLERY creative assets, this field must match the value of the creative asset's creativeAssetId.name field."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Click-through URL"]
pub struct ClickThroughUrl {
    #[serde(rename = "computedClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: - If defaultLandingPage is enabled then the campaign's default landing page URL is assigned to this field. - If defaultLandingPage is not enabled and a landingPageId is specified then that landing page's URL is assigned to this field. - If neither of the above cases apply, then the customClickThroughUrl is assigned to this field. "]
    pub computed_click_through_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom click-through URL. Applicable if the defaultLandingPage field is set to false and the landingPageId field is left unset."]
    pub custom_click_through_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultLandingPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the campaign default landing page is used."]
    pub default_landing_page: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "landingPageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the landing page for the click-through URL. Applicable if the defaultLandingPage field is set to false."]
    pub landing_page_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Click Through URL Suffix settings."]
pub struct ClickThroughUrlSuffixProperties {
    #[serde(rename = "clickThroughUrlSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL suffix to apply to all ads in this entity's scope. Must be less than 128 characters long."]
    pub click_through_url_suffix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overrideInheritedSuffix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this entity should override the inherited click-through URL suffix with its own defined value."]
    pub override_inherited_suffix: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Companion Click-through override."]
pub struct CompanionClickThroughOverride {
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL of this companion click-through override."]
    pub click_through_url: ::std::option::Option<::std::boxed::Box<ClickThroughUrl>>,
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative for this companion click-through override."]
    pub creative_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Companion Settings"]
pub struct CompanionSetting {
    #[serde(rename = "companionsDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether companions are disabled for this placement."]
    pub companions_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabledSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowlist of companion sizes to be served to this placement. Set this list to null or empty to serve all companion sizes."]
    pub enabled_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
    #[serde(rename = "imageOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to serve only static images as companions."]
    pub image_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#companionSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a response to the queryCompatibleFields method."]
pub struct CompatibleFields {
    #[serde(rename = "crossDimensionReachReportCompatibleFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains items that are compatible to be selected for a report of type \"CROSS_DIMENSION_REACH\"."]
    pub cross_dimension_reach_report_compatible_fields:
        ::std::option::Option<::std::boxed::Box<CrossDimensionReachReportCompatibleFields>>,
    #[serde(rename = "floodlightReportCompatibleFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains items that are compatible to be selected for a report of type \"FLOODLIGHT\"."]
    pub floodlight_report_compatible_fields:
        ::std::option::Option<::std::boxed::Box<FloodlightReportCompatibleFields>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#compatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pathToConversionReportCompatibleFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains items that are compatible to be selected for a report of type \"PATH_TO_CONVERSION\"."]
    pub path_to_conversion_report_compatible_fields:
        ::std::option::Option<::std::boxed::Box<PathToConversionReportCompatibleFields>>,
    #[serde(rename = "reachReportCompatibleFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains items that are compatible to be selected for a report of type \"REACH\"."]
    pub reach_report_compatible_fields:
        ::std::option::Option<::std::boxed::Box<ReachReportCompatibleFields>>,
    #[serde(rename = "reportCompatibleFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains items that are compatible to be selected for a report of type \"STANDARD\"."]
    pub report_compatible_fields: ::std::option::Option<::std::boxed::Box<ReportCompatibleFields>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about an internet connection type that can be targeted by ads. Clients can use the connection type to target mobile vs. broadband users."]
pub struct ConnectionType {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this connection type."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#connectionType\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this connection type."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Connection Type List Response"]
pub struct ConnectionTypesListResponse {
    #[serde(rename = "connectionTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of connection types such as broadband and mobile."]
    pub connection_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectionType>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#connectionTypesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Content Category List Response"]
pub struct ContentCategoriesListResponse {
    #[serde(rename = "contentCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content category collection."]
    pub content_categories:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContentCategory>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#contentCategoriesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Organizes placements according to the contents of their associated webpages."]
pub struct ContentCategory {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this content category. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this content category. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#contentCategory\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this content category. This is a required field and must be less than 256 characters long and unique among content categories of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Conversion represents when a user successfully performs a desired action after seeing an ad."]
pub struct Conversion {
    #[serde(rename = "childDirectedTreatment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this particular request may come from a user under the age of 13, under COPPA compliance."]
    pub child_directed_treatment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "customVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom floodlight variables."]
    pub custom_variables:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomFloodlightVariable>>>,
    #[serde(rename = "encryptedUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alphanumeric encrypted user ID. When set, encryptionInfo should also be specified. This field is mutually exclusive with encryptedUserIdCandidates[], matchId, mobileDeviceId and gclid. This or encryptedUserIdCandidates[] or matchId or mobileDeviceId or gclid is a required field."]
    pub encrypted_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "encryptedUserIdCandidates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the alphanumeric encrypted user IDs. Any user ID with exposure prior to the conversion timestamp will be used in the inserted conversion. If no such user ID is found then the conversion will be rejected with INVALID_ARGUMENT error. When set, encryptionInfo should also be specified. This field may only be used when calling batchinsert; it is not supported by batchupdate. This field is mutually exclusive with encryptedUserId, matchId, mobileDeviceId and gclid. This or encryptedUserId or matchId or mobileDeviceId or gclid is a required field."]
    pub encrypted_user_id_candidates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "floodlightActivityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight Activity ID of this conversion. This is a required field."]
    pub floodlight_activity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight Configuration ID of this conversion. This is a required field."]
    pub floodlight_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gclid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google click ID. This field is mutually exclusive with encryptedUserId, encryptedUserIdCandidates[], matchId and mobileDeviceId. This or encryptedUserId or encryptedUserIdCandidates[] or matchId or mobileDeviceId is a required field."]
    pub gclid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversion\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limitAdTracking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Limit Ad Tracking is enabled. When set to true, the conversion will be used for reporting but not targeting. This will prevent remarketing."]
    pub limit_ad_tracking: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "matchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The match ID field. A match ID is your own first-party identifier that has been synced with Google using the match ID feature in Floodlight. This field is mutually exclusive with encryptedUserId, encryptedUserIdCandidates[],mobileDeviceId and gclid. This or encryptedUserId or encryptedUserIdCandidates[] or mobileDeviceId or gclid is a required field."]
    pub match_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileDeviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mobile device ID. This field is mutually exclusive with encryptedUserId, encryptedUserIdCandidates[], matchId and gclid. This or encryptedUserId or encryptedUserIdCandidates[] or matchId or gclid is a required field."]
    pub mobile_device_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonPersonalizedAd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the conversion was for a non personalized ad."]
    pub non_personalized_ad: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ordinal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ordinal of the conversion. Use this field to control how conversions of the same user and day are de-duplicated. This is a required field."]
    pub ordinal: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the conversion."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of conversion, in Unix epoch micros. This is a required field."]
    pub timestamp_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "treatmentForUnderage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this particular request may come from a user under the age of 16 (may differ by country), under compliance with the European Union's General Data Protection Regulation (GDPR)."]
    pub treatment_for_underage: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the conversion."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The error code and description for a conversion that failed to insert or update."]
pub struct ConversionError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code."]
    pub code: ::std::option::Option<ConversionErrorCodeEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionError\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the error."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The error code."]
pub enum ConversionErrorCodeEnum {
    #[serde(rename = "INVALID_ARGUMENT")]
    #[doc = ""]
    InvalidArgument,
    #[serde(rename = "INTERNAL")]
    #[doc = ""]
    Internal,
    #[serde(rename = "PERMISSION_DENIED")]
    #[doc = ""]
    PermissionDenied,
    #[serde(rename = "NOT_FOUND")]
    #[doc = ""]
    NotFound,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The original conversion that was inserted or updated and whether there were any errors."]
pub struct ConversionStatus {
    #[serde(rename = "conversion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original conversion that was inserted or updated."]
    pub conversion: ::std::option::Option<::std::boxed::Box<Conversion>>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors related to this conversion."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConversionError>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionStatus\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Insert Conversions Request."]
pub struct ConversionsBatchInsertRequest {
    #[serde(rename = "conversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of conversions to insert."]
    pub conversions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Conversion>>>,
    #[serde(rename = "encryptionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes how encryptedUserId or encryptedUserIdCandidates[] is encrypted. This is a required field if encryptedUserId or encryptedUserIdCandidates[] is used."]
    pub encryption_info: ::std::option::Option<::std::boxed::Box<EncryptionInfo>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionsBatchInsertRequest\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Insert Conversions Response."]
pub struct ConversionsBatchInsertResponse {
    #[serde(rename = "hasFailures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that some or all conversions failed to insert."]
    pub has_failures: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionsBatchInsertResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The insert status of each conversion. Statuses are returned in the same order that conversions are inserted."]
    pub status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConversionStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Update Conversions Request."]
pub struct ConversionsBatchUpdateRequest {
    #[serde(rename = "conversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of conversions to update."]
    pub conversions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Conversion>>>,
    #[serde(rename = "encryptionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes how encryptedUserId is encrypted. This is a required field if encryptedUserId is used."]
    pub encryption_info: ::std::option::Option<::std::boxed::Box<EncryptionInfo>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionsBatchUpdateRequest\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Update Conversions Response."]
pub struct ConversionsBatchUpdateResponse {
    #[serde(rename = "hasFailures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that some or all conversions failed to update."]
    pub has_failures: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#conversionsBatchUpdateResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The update status of each conversion. Statuses are returned in the same order that conversions are updated."]
    pub status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConversionStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Country List Response"]
pub struct CountriesListResponse {
    #[serde(rename = "countries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country collection."]
    pub countries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Country>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#countriesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a country that can be targeted by ads."]
pub struct Country {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this country. This is the ID used for targeting and generating reports."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#country\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this country."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sslEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether ad serving supports secure servers in this country."]
    pub ssl_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Creative."]
pub struct Creative {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the creative is active. Applicable to all creative types."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "adParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad parameters user for VPAID creative. This is a read-only field. Applicable to the following creative types: all VPAID."]
    pub ad_parameters: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adTagKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Keywords for a Rich Media creative. Keywords let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use keywords to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub ad_tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "additionalSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional sizes associated with a responsive creative. When inserting or updating a creative either the size ID field or size width and height fields can be used. Applicable to DISPLAY creatives when the primary asset type is HTML_IMAGE."]
    pub additional_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this creative. This is a required field. Applicable to all creative types."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allowScriptAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether script access is allowed for this creative. This is a read-only and deprecated field which will automatically be set to true on update. Applicable to the following creative types: FLASH_INPAGE."]
    pub allow_script_access: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the creative is archived. Applicable to all creative types."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "artworkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of artwork used for the creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub artwork_type: ::std::option::Option<CreativeArtworkTypeEnum>,
    #[serde(rename = "authoringSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source application where creative was authored. Presently, only DBM authored creatives will have this field set. Applicable to all creative types."]
    pub authoring_source: ::std::option::Option<CreativeAuthoringSourceEnum>,
    #[serde(rename = "authoringTool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authoring tool for HTML5 banner creatives. This is a read-only field. Applicable to the following creative types: HTML5_BANNER."]
    pub authoring_tool: ::std::option::Option<CreativeAuthoringToolEnum>,
    #[serde(rename = "autoAdvanceImages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether images are automatically advanced for image gallery creatives. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY."]
    pub auto_advance_images: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 6-character HTML color code, beginning with #, for the background of the window area where the Flash file is displayed. Default is white. Applicable to the following creative types: FLASH_INPAGE."]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "backupImageClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL for backup image. Applicable to ENHANCED_BANNER when the primary asset type is not HTML_IMAGE."]
    pub backup_image_click_through_url:
        ::std::option::Option<::std::boxed::Box<CreativeClickThroughUrl>>,
    #[serde(rename = "backupImageFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of feature dependencies that will cause a backup image to be served if the browser that serves the ad does not support them. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative asset correctly. This field is initially auto-generated to contain all features detected by Campaign Manager for all the assets of this creative and can then be modified by the client. To reset this field, copy over all the creativeAssets' detected features. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub backup_image_features:
        ::std::option::Option<::std::vec::Vec<CreativeBackupImageFeaturesEnum>>,
    #[serde(rename = "backupImageReportingLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reporting label used for HTML5 banner backup image. Applicable to the following creative types: DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub backup_image_reporting_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "backupImageTargetWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target window for backup image. Applicable to the following creative types: FLASH_INPAGE and HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub backup_image_target_window: ::std::option::Option<::std::boxed::Box<TargetWindow>>,
    #[serde(rename = "clickTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click tags of the creative. For DISPLAY, FLASH_INPAGE, and HTML5_BANNER creatives, this is a subset of detected click tags for the assets associated with this creative. After creating a flash asset, detected click tags will be returned in the creativeAssetMetadata. When inserting the creative, populate the creative clickTags field using the creativeAssetMetadata.clickTags field. For DISPLAY_IMAGE_GALLERY creatives, there should be exactly one entry in this list for each image creative asset. A click tag is matched with a corresponding creative asset by matching the clickTag.name field with the creativeAsset.assetIdentifier.name field. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub click_tags: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClickTag>>>,
    #[serde(rename = "commercialId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Industry standard ID assigned to creative for reach and frequency. Applicable to INSTREAM_VIDEO_REDIRECT creatives."]
    pub commercial_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "companionCreatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of companion creatives assigned to an in-Stream video creative. Acceptable values include IDs of existing flash and image creatives. Applicable to the following creative types: all VPAID, all INSTREAM_AUDIO and all INSTREAM_VIDEO with dynamicAssetSelection set to false."]
    pub companion_creatives: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "compatibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compatibilities associated with this creative. This is a read-only field. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. Only pre-existing creatives may have these compatibilities since new creatives will either be assigned DISPLAY or DISPLAY_INTERSTITIAL instead. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. IN_STREAM_AUDIO refers to rendering in in-stream audio ads developed with the VAST standard. Applicable to all creative types. Acceptable values are: - \"APP\" - \"APP_INTERSTITIAL\" - \"IN_STREAM_VIDEO\" - \"IN_STREAM_AUDIO\" - \"DISPLAY\" - \"DISPLAY_INTERSTITIAL\" "]
    pub compatibility: ::std::option::Option<::std::vec::Vec<CreativeCompatibilityEnum>>,
    #[serde(rename = "convertFlashToHtml5")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Flash assets associated with the creative need to be automatically converted to HTML5. This flag is enabled by default and users can choose to disable it if they don't want the system to generate and use HTML5 asset for this creative. Applicable to the following creative type: FLASH_INPAGE. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub convert_flash_to_html5: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "counterCustomEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of counter events configured for the creative. For DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID."]
    pub counter_custom_events:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeCustomEvent>>>,
    #[serde(rename = "creativeAssetSelection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if dynamicAssetSelection is true."]
    pub creative_asset_selection: ::std::option::Option<::std::boxed::Box<CreativeAssetSelection>>,
    #[serde(rename = "creativeAssets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assets associated with a creative. Applicable to all but the following creative types: INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and REDIRECT"]
    pub creative_assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeAsset>>>,
    #[serde(rename = "creativeFieldAssignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative field assignments for this creative. Applicable to all creative types."]
    pub creative_field_assignments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeFieldAssignment>>>,
    #[serde(rename = "customKeyValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom key-values for a Rich Media creative. Key-values let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use key-values to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub custom_key_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dynamicAssetSelection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set this to true to enable the use of rules to target individual assets in this creative. When set to true creativeAssetSelection must be set. This also controls asset-level companions. When this is true, companion creatives should be assigned to creative assets. Learn more. Applicable to INSTREAM_VIDEO creatives."]
    pub dynamic_asset_selection: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "exitCustomEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of exit events configured for the creative. For DISPLAY and DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags, For DISPLAY, an event is also created from the backupImageReportingLabel. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub exit_custom_events:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeCustomEvent>>>,
    #[serde(rename = "fsCommand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OpenWindow FSCommand of this creative. This lets the SWF file communicate with either Flash Player or the program hosting Flash Player, such as a web browser. This is only triggered if allowScriptAccess field is true. Applicable to the following creative types: FLASH_INPAGE."]
    pub fs_command: ::std::option::Option<::std::boxed::Box<FsCommand>>,
    #[serde(rename = "htmlCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTML code for the creative. This is a required field when applicable. This field is ignored if htmlCodeLocked is true. Applicable to the following creative types: all CUSTOM, FLASH_INPAGE, and HTML5_BANNER, and all RICH_MEDIA."]
    pub html_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlCodeLocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether HTML code is generated by Campaign Manager or manually entered. Set to true to ignore changes to htmlCode. Applicable to the following creative types: FLASH_INPAGE and HTML5_BANNER."]
    pub html_code_locked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this creative. This is a read-only, auto-generated field. Applicable to all creative types."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this creative. This is a read-only field. Applicable to all creative types."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creative\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative last modification information. This is a read-only field. Applicable to all creative types."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "latestTraffickedCreativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latest Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub latest_trafficked_creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the audio or video ad. Applicable to the following creative types: all INSTREAM_VIDEO, INSTREAM_AUDIO, and all VPAID."]
    pub media_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative audio or video duration in seconds. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO, INSTREAM_AUDIO, all RICH_MEDIA, and all VPAID."]
    pub media_duration: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the creative. This is a required field and must be less than 256 characters long. Applicable to all creative types."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overrideCss")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Override CSS value for rich media creatives. Applicable to the following creative types: all RICH_MEDIA."]
    pub override_css: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play the video before counting a view. Applicable to the following creative types: all INSTREAM_VIDEO."]
    pub progress_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "redirectUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of hosted image or hosted video or another ad tag. For INSTREAM_VIDEO_REDIRECT creatives this is the in-stream video redirect URL. The standard for a VAST (Video Ad Serving Template) ad response allows for a redirect link to another VAST 2.0 or 3.0 call. This is a required field when applicable. Applicable to the following creative types: DISPLAY_REDIRECT, INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and INSTREAM_VIDEO_REDIRECT"]
    pub redirect_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "renderingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of current rendering version. This is a read-only field. Applicable to all creative types."]
    pub rendering_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "renderingIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the rendering ID of this creative. This is a read-only field. Applicable to all creative types."]
    pub rendering_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "requiredFlashPluginVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum required Flash plugin version for this creative. For example, 11.2.202.235. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub required_flash_plugin_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requiredFlashVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The internal Flash version for this creative as calculated by Studio. This is a read-only field. Applicable to the following creative types: FLASH_INPAGE all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub required_flash_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size associated with this creative. When inserting or updating a creative either the size ID field or size width and height fields can be used. This is a required field when applicable; however for IMAGE, FLASH_INPAGE creatives, and for DISPLAY creatives with a primary asset of type HTML_IMAGE, if left blank, this field will be automatically set using the actual size of the associated image assets. Applicable to the following creative types: DISPLAY, DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER, IMAGE, and all RICH_MEDIA."]
    pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "skipOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play the video before the skip button appears. Applicable to the following creative types: all INSTREAM_VIDEO."]
    pub skip_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "skippable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can choose to skip the creative. Applicable to the following creative types: all INSTREAM_VIDEO and all VPAID."]
    pub skippable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the creative is SSL-compliant. This is a read-only field. Applicable to all creative types."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sslOverride")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether creative should be treated as SSL compliant even if the system scan shows it's not. Applicable to all creative types."]
    pub ssl_override: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "studioAdvertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Studio advertiser ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub studio_advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "studioCreativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Studio creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub studio_creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "studioTraffickedCreativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub studio_trafficked_creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirdPartyBackupImageImpressionsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party URL used to record backup image impressions. Applicable to the following creative types: all RICH_MEDIA."]
    pub third_party_backup_image_impressions_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirdPartyRichMediaImpressionsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party URL used to record rich media impressions. Applicable to the following creative types: all RICH_MEDIA."]
    pub third_party_rich_media_impressions_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirdPartyUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party URLs for tracking in-stream creative events. Applicable to the following creative types: all INSTREAM_VIDEO, all INSTREAM_AUDIO, and all VPAID."]
    pub third_party_urls:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThirdPartyTrackingUrl>>>,
    #[serde(rename = "timerCustomEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of timer events configured for the creative. For DISPLAY_IMAGE_GALLERY creatives, these are read-only and auto-generated from clickTags. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset is not HTML_IMAGE."]
    pub timer_custom_events:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeCustomEvent>>>,
    #[serde(rename = "totalFileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Combined size of all creative assets. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
    pub total_file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this creative. This is a required field. Applicable to all creative types. *Note:* FLASH_INPAGE, HTML5_BANNER, and IMAGE are only used for existing creatives. New creatives should use DISPLAY as a replacement for these types."]
    pub _type: ::std::option::Option<CreativeTypeEnum>,
    #[serde(rename = "universalAdId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Universal Ad ID as per the VAST 4.0 spec. Applicable to the following creative types: INSTREAM_AUDIO and INSTREAM_VIDEO and VPAID."]
    pub universal_ad_id: ::std::option::Option<::std::boxed::Box<UniversalAdId>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version number helps you keep track of multiple versions of your creative in your reports. The version number will always be auto-generated during insert operations to start at 1. For tracking creatives the version cannot be incremented and will always remain at 1. For all other creative types the version can be incremented only by 1 during update operations. In addition, the version will be automatically incremented by 1 when undergoing Rich Media creative merging. Applicable to all creative types."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of artwork used for the creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID."]
pub enum CreativeArtworkTypeEnum {
    #[serde(rename = "ARTWORK_TYPE_FLASH")]
    #[doc = ""]
    ArtworkTypeFlash,
    #[serde(rename = "ARTWORK_TYPE_HTML5")]
    #[doc = ""]
    ArtworkTypeHtml5,
    #[serde(rename = "ARTWORK_TYPE_MIXED")]
    #[doc = ""]
    ArtworkTypeMixed,
    #[serde(rename = "ARTWORK_TYPE_IMAGE")]
    #[doc = ""]
    ArtworkTypeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Source application where creative was authored. Presently, only DBM authored creatives will have this field set. Applicable to all creative types."]
pub enum CreativeAuthoringSourceEnum {
    #[serde(rename = "CREATIVE_AUTHORING_SOURCE_DCM")]
    #[doc = ""]
    CreativeAuthoringSourceDcm,
    #[serde(rename = "CREATIVE_AUTHORING_SOURCE_DBM")]
    #[doc = ""]
    CreativeAuthoringSourceDbm,
    #[serde(rename = "CREATIVE_AUTHORING_SOURCE_STUDIO")]
    #[doc = ""]
    CreativeAuthoringSourceStudio,
    #[serde(rename = "CREATIVE_AUTHORING_SOURCE_GWD")]
    #[doc = ""]
    CreativeAuthoringSourceGwd,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Authoring tool for HTML5 banner creatives. This is a read-only field. Applicable to the following creative types: HTML5_BANNER."]
pub enum CreativeAuthoringToolEnum {
    #[serde(rename = "NINJA")]
    #[doc = ""]
    Ninja,
    #[serde(rename = "SWIFFY")]
    #[doc = ""]
    Swiffy,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeBackupImageFeaturesEnum {
    #[serde(rename = "CSS_FONT_FACE")]
    #[doc = ""]
    CssFontFace,
    #[serde(rename = "CSS_BACKGROUND_SIZE")]
    #[doc = ""]
    CssBackgroundSize,
    #[serde(rename = "CSS_BORDER_IMAGE")]
    #[doc = ""]
    CssBorderImage,
    #[serde(rename = "CSS_BORDER_RADIUS")]
    #[doc = ""]
    CssBorderRadius,
    #[serde(rename = "CSS_BOX_SHADOW")]
    #[doc = ""]
    CssBoxShadow,
    #[serde(rename = "CSS_FLEX_BOX")]
    #[doc = ""]
    CssFlexBox,
    #[serde(rename = "CSS_HSLA")]
    #[doc = ""]
    CssHsla,
    #[serde(rename = "CSS_MULTIPLE_BGS")]
    #[doc = ""]
    CssMultipleBgs,
    #[serde(rename = "CSS_OPACITY")]
    #[doc = ""]
    CssOpacity,
    #[serde(rename = "CSS_RGBA")]
    #[doc = ""]
    CssRgba,
    #[serde(rename = "CSS_TEXT_SHADOW")]
    #[doc = ""]
    CssTextShadow,
    #[serde(rename = "CSS_ANIMATIONS")]
    #[doc = ""]
    CssAnimations,
    #[serde(rename = "CSS_COLUMNS")]
    #[doc = ""]
    CssColumns,
    #[serde(rename = "CSS_GENERATED_CONTENT")]
    #[doc = ""]
    CssGeneratedContent,
    #[serde(rename = "CSS_GRADIENTS")]
    #[doc = ""]
    CssGradients,
    #[serde(rename = "CSS_REFLECTIONS")]
    #[doc = ""]
    CssReflections,
    #[serde(rename = "CSS_TRANSFORMS")]
    #[doc = ""]
    CssTransforms,
    #[serde(rename = "CSS_TRANSFORMS3D")]
    #[doc = ""]
    CssTransforms3D,
    #[serde(rename = "CSS_TRANSITIONS")]
    #[doc = ""]
    CssTransitions,
    #[serde(rename = "APPLICATION_CACHE")]
    #[doc = ""]
    ApplicationCache,
    #[serde(rename = "CANVAS")]
    #[doc = ""]
    Canvas,
    #[serde(rename = "CANVAS_TEXT")]
    #[doc = ""]
    CanvasText,
    #[serde(rename = "DRAG_AND_DROP")]
    #[doc = ""]
    DragAndDrop,
    #[serde(rename = "HASH_CHANGE")]
    #[doc = ""]
    HashChange,
    #[serde(rename = "HISTORY")]
    #[doc = ""]
    History,
    #[serde(rename = "AUDIO")]
    #[doc = ""]
    Audio,
    #[serde(rename = "VIDEO")]
    #[doc = ""]
    Video,
    #[serde(rename = "INDEXED_DB")]
    #[doc = ""]
    IndexedDb,
    #[serde(rename = "INPUT_ATTR_AUTOCOMPLETE")]
    #[doc = ""]
    InputAttrAutocomplete,
    #[serde(rename = "INPUT_ATTR_AUTOFOCUS")]
    #[doc = ""]
    InputAttrAutofocus,
    #[serde(rename = "INPUT_ATTR_LIST")]
    #[doc = ""]
    InputAttrList,
    #[serde(rename = "INPUT_ATTR_PLACEHOLDER")]
    #[doc = ""]
    InputAttrPlaceholder,
    #[serde(rename = "INPUT_ATTR_MAX")]
    #[doc = ""]
    InputAttrMax,
    #[serde(rename = "INPUT_ATTR_MIN")]
    #[doc = ""]
    InputAttrMin,
    #[serde(rename = "INPUT_ATTR_MULTIPLE")]
    #[doc = ""]
    InputAttrMultiple,
    #[serde(rename = "INPUT_ATTR_PATTERN")]
    #[doc = ""]
    InputAttrPattern,
    #[serde(rename = "INPUT_ATTR_REQUIRED")]
    #[doc = ""]
    InputAttrRequired,
    #[serde(rename = "INPUT_ATTR_STEP")]
    #[doc = ""]
    InputAttrStep,
    #[serde(rename = "INPUT_TYPE_SEARCH")]
    #[doc = ""]
    InputTypeSearch,
    #[serde(rename = "INPUT_TYPE_TEL")]
    #[doc = ""]
    InputTypeTel,
    #[serde(rename = "INPUT_TYPE_URL")]
    #[doc = ""]
    InputTypeUrl,
    #[serde(rename = "INPUT_TYPE_EMAIL")]
    #[doc = ""]
    InputTypeEmail,
    #[serde(rename = "INPUT_TYPE_DATETIME")]
    #[doc = ""]
    InputTypeDatetime,
    #[serde(rename = "INPUT_TYPE_DATE")]
    #[doc = ""]
    InputTypeDate,
    #[serde(rename = "INPUT_TYPE_MONTH")]
    #[doc = ""]
    InputTypeMonth,
    #[serde(rename = "INPUT_TYPE_WEEK")]
    #[doc = ""]
    InputTypeWeek,
    #[serde(rename = "INPUT_TYPE_TIME")]
    #[doc = ""]
    InputTypeTime,
    #[serde(rename = "INPUT_TYPE_DATETIME_LOCAL")]
    #[doc = ""]
    InputTypeDatetimeLocal,
    #[serde(rename = "INPUT_TYPE_NUMBER")]
    #[doc = ""]
    InputTypeNumber,
    #[serde(rename = "INPUT_TYPE_RANGE")]
    #[doc = ""]
    InputTypeRange,
    #[serde(rename = "INPUT_TYPE_COLOR")]
    #[doc = ""]
    InputTypeColor,
    #[serde(rename = "LOCAL_STORAGE")]
    #[doc = ""]
    LocalStorage,
    #[serde(rename = "POST_MESSAGE")]
    #[doc = ""]
    PostMessage,
    #[serde(rename = "SESSION_STORAGE")]
    #[doc = ""]
    SessionStorage,
    #[serde(rename = "WEB_SOCKETS")]
    #[doc = ""]
    WebSockets,
    #[serde(rename = "WEB_SQL_DATABASE")]
    #[doc = ""]
    WebSqlDatabase,
    #[serde(rename = "WEB_WORKERS")]
    #[doc = ""]
    WebWorkers,
    #[serde(rename = "GEO_LOCATION")]
    #[doc = ""]
    GeoLocation,
    #[serde(rename = "INLINE_SVG")]
    #[doc = ""]
    InlineSvg,
    #[serde(rename = "SMIL")]
    #[doc = ""]
    Smil,
    #[serde(rename = "SVG_HREF")]
    #[doc = ""]
    SvgHref,
    #[serde(rename = "SVG_CLIP_PATHS")]
    #[doc = ""]
    SvgClipPaths,
    #[serde(rename = "TOUCH")]
    #[doc = ""]
    Touch,
    #[serde(rename = "WEBGL")]
    #[doc = ""]
    Webgl,
    #[serde(rename = "SVG_FILTERS")]
    #[doc = ""]
    SvgFilters,
    #[serde(rename = "SVG_FE_IMAGE")]
    #[doc = ""]
    SvgFeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeCompatibilityEnum {
    #[serde(rename = "DISPLAY")]
    #[doc = ""]
    Display,
    #[serde(rename = "DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    DisplayInterstitial,
    #[serde(rename = "APP")]
    #[doc = ""]
    App,
    #[serde(rename = "APP_INTERSTITIAL")]
    #[doc = ""]
    AppInterstitial,
    #[serde(rename = "IN_STREAM_VIDEO")]
    #[doc = ""]
    InStreamVideo,
    #[serde(rename = "IN_STREAM_AUDIO")]
    #[doc = ""]
    InStreamAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of this creative. This is a required field. Applicable to all creative types. *Note:* FLASH_INPAGE, HTML5_BANNER, and IMAGE are only used for existing creatives. New creatives should use DISPLAY as a replacement for these types."]
pub enum CreativeTypeEnum {
    #[serde(rename = "IMAGE")]
    #[doc = ""]
    Image,
    #[serde(rename = "DISPLAY_REDIRECT")]
    #[doc = ""]
    DisplayRedirect,
    #[serde(rename = "CUSTOM_DISPLAY")]
    #[doc = ""]
    CustomDisplay,
    #[serde(rename = "INTERNAL_REDIRECT")]
    #[doc = ""]
    InternalRedirect,
    #[serde(rename = "CUSTOM_DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    CustomDisplayInterstitial,
    #[serde(rename = "INTERSTITIAL_INTERNAL_REDIRECT")]
    #[doc = ""]
    InterstitialInternalRedirect,
    #[serde(rename = "TRACKING_TEXT")]
    #[doc = ""]
    TrackingText,
    #[serde(rename = "RICH_MEDIA_DISPLAY_BANNER")]
    #[doc = ""]
    RichMediaDisplayBanner,
    #[serde(rename = "RICH_MEDIA_INPAGE_FLOATING")]
    #[doc = ""]
    RichMediaInpageFloating,
    #[serde(rename = "RICH_MEDIA_IM_EXPAND")]
    #[doc = ""]
    RichMediaImExpand,
    #[serde(rename = "RICH_MEDIA_DISPLAY_EXPANDING")]
    #[doc = ""]
    RichMediaDisplayExpanding,
    #[serde(rename = "RICH_MEDIA_DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    RichMediaDisplayInterstitial,
    #[serde(rename = "RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL")]
    #[doc = ""]
    RichMediaDisplayMultiFloatingInterstitial,
    #[serde(rename = "RICH_MEDIA_MOBILE_IN_APP")]
    #[doc = ""]
    RichMediaMobileInApp,
    #[serde(rename = "FLASH_INPAGE")]
    #[doc = ""]
    FlashInpage,
    #[serde(rename = "INSTREAM_VIDEO")]
    #[doc = ""]
    InstreamVideo,
    #[serde(rename = "VPAID_LINEAR_VIDEO")]
    #[doc = ""]
    VpaidLinearVideo,
    #[serde(rename = "VPAID_NON_LINEAR_VIDEO")]
    #[doc = ""]
    VpaidNonLinearVideo,
    #[serde(rename = "INSTREAM_VIDEO_REDIRECT")]
    #[doc = ""]
    InstreamVideoRedirect,
    #[serde(rename = "RICH_MEDIA_PEEL_DOWN")]
    #[doc = ""]
    RichMediaPeelDown,
    #[serde(rename = "HTML5_BANNER")]
    #[doc = ""]
    Html5Banner,
    #[serde(rename = "DISPLAY")]
    #[doc = ""]
    Display,
    #[serde(rename = "DISPLAY_IMAGE_GALLERY")]
    #[doc = ""]
    DisplayImageGallery,
    #[serde(rename = "BRAND_SAFE_DEFAULT_INSTREAM_VIDEO")]
    #[doc = ""]
    BrandSafeDefaultInstreamVideo,
    #[serde(rename = "INSTREAM_AUDIO")]
    #[doc = ""]
    InstreamAudio,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Asset."]
pub struct CreativeAsset {
    #[serde(rename = "actionScript3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether ActionScript3 is enabled for the flash asset. This is a read-only field. Applicable to the following creative type: FLASH_INPAGE. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub action_script3: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the video or audio asset is active. This is a read-only field for VPAID_NON_LINEAR_VIDEO assets. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "additionalSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional sizes associated with this creative asset. HTML5 asset generated by compatible software such as GWD will be able to support more sizes this creative asset can render."]
    pub additional_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Possible alignments for an asset. This is a read-only field. Applicable to the following creative types: RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL ."]
    pub alignment: ::std::option::Option<CreativeAssetAlignmentEnum>,
    #[serde(rename = "artworkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Artwork type of rich media creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
    pub artwork_type: ::std::option::Option<CreativeAssetArtworkTypeEnum>,
    #[serde(rename = "assetIdentifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of this asset. This is the same identifier returned during creative asset insert operation. This is a required field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT."]
    pub asset_identifier: ::std::option::Option<::std::boxed::Box<CreativeAssetId>>,
    #[serde(rename = "audioBitRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audio stream bit rate in kbps. This is a read-only field. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub audio_bit_rate: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "audioSampleRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audio sample bit rate in hertz. This is a read-only field. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub audio_sample_rate: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "backupImageExit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exit event configured for the backup image. Applicable to the following creative types: all RICH_MEDIA."]
    pub backup_image_exit: ::std::option::Option<::std::boxed::Box<CreativeCustomEvent>>,
    #[serde(rename = "bitRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected bit-rate for audio or video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub bit_rate: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "childAssetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich media child asset type. This is a read-only field. Applicable to the following creative types: all VPAID."]
    pub child_asset_type: ::std::option::Option<CreativeAssetChildAssetTypeEnum>,
    #[serde(rename = "collapsedSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of an asset when collapsed. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID. Additionally, applicable to assets whose displayType is ASSET_DISPLAY_TYPE_EXPANDING or ASSET_DISPLAY_TYPE_PEEL_DOWN."]
    pub collapsed_size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "companionCreativeIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of companion creatives assigned to an in-stream video creative asset. Acceptable values include IDs of existing flash and image creatives. Applicable to INSTREAM_VIDEO creative type with dynamicAssetSelection set to true."]
    pub companion_creative_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "customStartTimeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom start time in seconds for making the asset visible. Applicable to the following creative types: all RICH_MEDIA. Value must be greater than or equal to 0."]
    pub custom_start_time_value: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "detectedFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub detected_features:
        ::std::option::Option<::std::vec::Vec<CreativeAssetDetectedFeaturesEnum>>,
    #[serde(rename = "displayType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of rich media asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
    pub display_type: ::std::option::Option<CreativeAssetDisplayTypeEnum>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration in seconds for which an asset will be displayed. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and VPAID_LINEAR_VIDEO. Value must be greater than or equal to 1."]
    pub duration: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "durationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration type for which an asset will be displayed. Applicable to the following creative types: all RICH_MEDIA."]
    pub duration_type: ::std::option::Option<CreativeAssetDurationTypeEnum>,
    #[serde(rename = "expandedDimension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected expanded dimension for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID."]
    pub expanded_dimension: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File size associated with this creative asset. This is a read-only field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT."]
    pub file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flashVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flash version of the asset. This is a read-only field. Applicable to the following creative types: FLASH_INPAGE, all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub flash_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "frameRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video frame rate for video asset in frames per second. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID."]
    pub frame_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "hideFlashObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to hide Flash objects flag for an asset. Applicable to the following creative types: all RICH_MEDIA."]
    pub hide_flash_objects: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "hideSelectionBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to hide selection boxes flag for an asset. Applicable to the following creative types: all RICH_MEDIA."]
    pub hide_selection_boxes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "horizontallyLocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset is horizontally locked. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
    pub horizontally_locked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numeric ID of this creative asset. This is a required field and should not be modified. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the asset. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "mediaDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected duration for audio or video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub media_duration: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected MIME type for audio or video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and all VPAID."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset position for an asset in collapsed mode. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID. Additionally, only applicable to assets whose displayType is ASSET_DISPLAY_TYPE_EXPANDING or ASSET_DISPLAY_TYPE_PEEL_DOWN."]
    pub offset: ::std::option::Option<::std::boxed::Box<OffsetPosition>>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Orientation of video asset. This is a read-only, auto-generated field."]
    pub orientation: ::std::option::Option<CreativeAssetOrientationEnum>,
    #[serde(rename = "originalBackup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the backup asset is original or changed by the user in Campaign Manager. Applicable to the following creative types: all RICH_MEDIA."]
    pub original_backup: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "politeLoad")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this asset is used as a polite load asset."]
    pub polite_load: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset position for an asset. Applicable to the following creative types: all RICH_MEDIA."]
    pub position: ::std::option::Option<::std::boxed::Box<OffsetPosition>>,
    #[serde(rename = "positionLeftUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset left unit for an asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
    pub position_left_unit: ::std::option::Option<CreativeAssetPositionLeftUnitEnum>,
    #[serde(rename = "positionTopUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset top unit for an asset. This is a read-only field if the asset displayType is ASSET_DISPLAY_TYPE_OVERLAY. Applicable to the following creative types: all RICH_MEDIA."]
    pub position_top_unit: ::std::option::Option<CreativeAssetPositionTopUnitEnum>,
    #[serde(rename = "progressiveServingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progressive URL for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID."]
    pub progressive_serving_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pushdown")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset pushes down other content. Applicable to the following creative types: all RICH_MEDIA. Additionally, only applicable when the asset offsets are 0, the collapsedSize.width matches size.width, and the collapsedSize.height is less than size.height."]
    pub pushdown: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pushdownDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pushdown duration in seconds for an asset. Applicable to the following creative types: all RICH_MEDIA.Additionally, only applicable when the asset pushdown field is true, the offsets are 0, the collapsedSize.width matches size.width, and the collapsedSize.height is less than size.height. Acceptable values are 0 to 9.99, inclusive."]
    pub pushdown_duration: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role of the asset in relation to creative. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT. This is a required field. PRIMARY applies to DISPLAY, FLASH_INPAGE, HTML5_BANNER, IMAGE, DISPLAY_IMAGE_GALLERY, all RICH_MEDIA (which may contain multiple primary assets), and all VPAID creatives. BACKUP_IMAGE applies to FLASH_INPAGE, HTML5_BANNER, all RICH_MEDIA, and all VPAID creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE. ADDITIONAL_IMAGE and ADDITIONAL_FLASH apply to FLASH_INPAGE creatives. OTHER refers to assets from sources other than Campaign Manager, such as Studio uploaded assets, applicable to all RICH_MEDIA and all VPAID creatives. PARENT_VIDEO refers to videos uploaded by the user in Campaign Manager and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. TRANSCODED_VIDEO refers to videos transcoded by Campaign Manager from PARENT_VIDEO assets and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. ALTERNATE_VIDEO refers to the Campaign Manager representation of child asset videos from Studio, and is applicable to VPAID_LINEAR_VIDEO creatives. These cannot be added or removed within Campaign Manager. For VPAID_LINEAR_VIDEO creatives, PARENT_VIDEO, TRANSCODED_VIDEO and ALTERNATE_VIDEO assets that are marked active serve as backup in case the VPAID creative cannot be served. Only PARENT_VIDEO assets can be added or removed for an INSTREAM_VIDEO or VPAID_LINEAR_VIDEO creative. PARENT_AUDIO refers to audios uploaded by the user in Campaign Manager and is applicable to INSTREAM_AUDIO creatives. TRANSCODED_AUDIO refers to audios transcoded by Campaign Manager from PARENT_AUDIO assets and is applicable to INSTREAM_AUDIO creatives. "]
    pub role: ::std::option::Option<CreativeAssetRoleEnum>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size associated with this creative asset. This is a required field when applicable; however for IMAGE and FLASH_INPAGE, creatives if left blank, this field will be automatically set using the actual size of the associated image asset. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY, FLASH_INPAGE, HTML5_BANNER, IMAGE, and all RICH_MEDIA. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE."]
    pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset is SSL-compliant. This is a read-only field. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startTimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Initial wait time type before making the asset visible. Applicable to the following creative types: all RICH_MEDIA."]
    pub start_time_type: ::std::option::Option<CreativeAssetStartTimeTypeEnum>,
    #[serde(rename = "streamingServingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming URL for video asset. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO and all VPAID."]
    pub streaming_serving_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transparency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset is transparent. Applicable to the following creative types: all RICH_MEDIA. Additionally, only applicable to HTML5 assets."]
    pub transparency: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "verticallyLocked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset is vertically locked. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
    pub vertically_locked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "windowMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Window mode options for flash assets. Applicable to the following creative types: FLASH_INPAGE, RICH_MEDIA_DISPLAY_EXPANDING, RICH_MEDIA_IM_EXPAND, RICH_MEDIA_DISPLAY_BANNER, and RICH_MEDIA_INPAGE_FLOATING."]
    pub window_mode: ::std::option::Option<CreativeAssetWindowModeEnum>,
    #[serde(rename = "zIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "zIndex value of an asset. Applicable to the following creative types: all RICH_MEDIA.Additionally, only applicable to assets whose displayType is NOT one of the following types: ASSET_DISPLAY_TYPE_INPAGE or ASSET_DISPLAY_TYPE_OVERLAY. Acceptable values are -999999999 to 999999999, inclusive."]
    pub z_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "zipFilename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File name of zip file. This is a read-only field. Applicable to the following creative types: HTML5_BANNER."]
    pub zip_filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zipFilesize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of zip file. This is a read-only field. Applicable to the following creative types: HTML5_BANNER."]
    pub zip_filesize: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Possible alignments for an asset. This is a read-only field. Applicable to the following creative types: RICH_MEDIA_DISPLAY_MULTI_FLOATING_INTERSTITIAL ."]
pub enum CreativeAssetAlignmentEnum {
    #[serde(rename = "ALIGNMENT_TOP")]
    #[doc = ""]
    AlignmentTop,
    #[serde(rename = "ALIGNMENT_RIGHT")]
    #[doc = ""]
    AlignmentRight,
    #[serde(rename = "ALIGNMENT_BOTTOM")]
    #[doc = ""]
    AlignmentBottom,
    #[serde(rename = "ALIGNMENT_LEFT")]
    #[doc = ""]
    AlignmentLeft,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Artwork type of rich media creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetArtworkTypeEnum {
    #[serde(rename = "ARTWORK_TYPE_FLASH")]
    #[doc = ""]
    ArtworkTypeFlash,
    #[serde(rename = "ARTWORK_TYPE_HTML5")]
    #[doc = ""]
    ArtworkTypeHtml5,
    #[serde(rename = "ARTWORK_TYPE_MIXED")]
    #[doc = ""]
    ArtworkTypeMixed,
    #[serde(rename = "ARTWORK_TYPE_IMAGE")]
    #[doc = ""]
    ArtworkTypeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Rich media child asset type. This is a read-only field. Applicable to the following creative types: all VPAID."]
pub enum CreativeAssetChildAssetTypeEnum {
    #[serde(rename = "CHILD_ASSET_TYPE_FLASH")]
    #[doc = ""]
    ChildAssetTypeFlash,
    #[serde(rename = "CHILD_ASSET_TYPE_VIDEO")]
    #[doc = ""]
    ChildAssetTypeVideo,
    #[serde(rename = "CHILD_ASSET_TYPE_IMAGE")]
    #[doc = ""]
    ChildAssetTypeImage,
    #[serde(rename = "CHILD_ASSET_TYPE_DATA")]
    #[doc = ""]
    ChildAssetTypeData,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeAssetDetectedFeaturesEnum {
    #[serde(rename = "CSS_FONT_FACE")]
    #[doc = ""]
    CssFontFace,
    #[serde(rename = "CSS_BACKGROUND_SIZE")]
    #[doc = ""]
    CssBackgroundSize,
    #[serde(rename = "CSS_BORDER_IMAGE")]
    #[doc = ""]
    CssBorderImage,
    #[serde(rename = "CSS_BORDER_RADIUS")]
    #[doc = ""]
    CssBorderRadius,
    #[serde(rename = "CSS_BOX_SHADOW")]
    #[doc = ""]
    CssBoxShadow,
    #[serde(rename = "CSS_FLEX_BOX")]
    #[doc = ""]
    CssFlexBox,
    #[serde(rename = "CSS_HSLA")]
    #[doc = ""]
    CssHsla,
    #[serde(rename = "CSS_MULTIPLE_BGS")]
    #[doc = ""]
    CssMultipleBgs,
    #[serde(rename = "CSS_OPACITY")]
    #[doc = ""]
    CssOpacity,
    #[serde(rename = "CSS_RGBA")]
    #[doc = ""]
    CssRgba,
    #[serde(rename = "CSS_TEXT_SHADOW")]
    #[doc = ""]
    CssTextShadow,
    #[serde(rename = "CSS_ANIMATIONS")]
    #[doc = ""]
    CssAnimations,
    #[serde(rename = "CSS_COLUMNS")]
    #[doc = ""]
    CssColumns,
    #[serde(rename = "CSS_GENERATED_CONTENT")]
    #[doc = ""]
    CssGeneratedContent,
    #[serde(rename = "CSS_GRADIENTS")]
    #[doc = ""]
    CssGradients,
    #[serde(rename = "CSS_REFLECTIONS")]
    #[doc = ""]
    CssReflections,
    #[serde(rename = "CSS_TRANSFORMS")]
    #[doc = ""]
    CssTransforms,
    #[serde(rename = "CSS_TRANSFORMS3D")]
    #[doc = ""]
    CssTransforms3D,
    #[serde(rename = "CSS_TRANSITIONS")]
    #[doc = ""]
    CssTransitions,
    #[serde(rename = "APPLICATION_CACHE")]
    #[doc = ""]
    ApplicationCache,
    #[serde(rename = "CANVAS")]
    #[doc = ""]
    Canvas,
    #[serde(rename = "CANVAS_TEXT")]
    #[doc = ""]
    CanvasText,
    #[serde(rename = "DRAG_AND_DROP")]
    #[doc = ""]
    DragAndDrop,
    #[serde(rename = "HASH_CHANGE")]
    #[doc = ""]
    HashChange,
    #[serde(rename = "HISTORY")]
    #[doc = ""]
    History,
    #[serde(rename = "AUDIO")]
    #[doc = ""]
    Audio,
    #[serde(rename = "VIDEO")]
    #[doc = ""]
    Video,
    #[serde(rename = "INDEXED_DB")]
    #[doc = ""]
    IndexedDb,
    #[serde(rename = "INPUT_ATTR_AUTOCOMPLETE")]
    #[doc = ""]
    InputAttrAutocomplete,
    #[serde(rename = "INPUT_ATTR_AUTOFOCUS")]
    #[doc = ""]
    InputAttrAutofocus,
    #[serde(rename = "INPUT_ATTR_LIST")]
    #[doc = ""]
    InputAttrList,
    #[serde(rename = "INPUT_ATTR_PLACEHOLDER")]
    #[doc = ""]
    InputAttrPlaceholder,
    #[serde(rename = "INPUT_ATTR_MAX")]
    #[doc = ""]
    InputAttrMax,
    #[serde(rename = "INPUT_ATTR_MIN")]
    #[doc = ""]
    InputAttrMin,
    #[serde(rename = "INPUT_ATTR_MULTIPLE")]
    #[doc = ""]
    InputAttrMultiple,
    #[serde(rename = "INPUT_ATTR_PATTERN")]
    #[doc = ""]
    InputAttrPattern,
    #[serde(rename = "INPUT_ATTR_REQUIRED")]
    #[doc = ""]
    InputAttrRequired,
    #[serde(rename = "INPUT_ATTR_STEP")]
    #[doc = ""]
    InputAttrStep,
    #[serde(rename = "INPUT_TYPE_SEARCH")]
    #[doc = ""]
    InputTypeSearch,
    #[serde(rename = "INPUT_TYPE_TEL")]
    #[doc = ""]
    InputTypeTel,
    #[serde(rename = "INPUT_TYPE_URL")]
    #[doc = ""]
    InputTypeUrl,
    #[serde(rename = "INPUT_TYPE_EMAIL")]
    #[doc = ""]
    InputTypeEmail,
    #[serde(rename = "INPUT_TYPE_DATETIME")]
    #[doc = ""]
    InputTypeDatetime,
    #[serde(rename = "INPUT_TYPE_DATE")]
    #[doc = ""]
    InputTypeDate,
    #[serde(rename = "INPUT_TYPE_MONTH")]
    #[doc = ""]
    InputTypeMonth,
    #[serde(rename = "INPUT_TYPE_WEEK")]
    #[doc = ""]
    InputTypeWeek,
    #[serde(rename = "INPUT_TYPE_TIME")]
    #[doc = ""]
    InputTypeTime,
    #[serde(rename = "INPUT_TYPE_DATETIME_LOCAL")]
    #[doc = ""]
    InputTypeDatetimeLocal,
    #[serde(rename = "INPUT_TYPE_NUMBER")]
    #[doc = ""]
    InputTypeNumber,
    #[serde(rename = "INPUT_TYPE_RANGE")]
    #[doc = ""]
    InputTypeRange,
    #[serde(rename = "INPUT_TYPE_COLOR")]
    #[doc = ""]
    InputTypeColor,
    #[serde(rename = "LOCAL_STORAGE")]
    #[doc = ""]
    LocalStorage,
    #[serde(rename = "POST_MESSAGE")]
    #[doc = ""]
    PostMessage,
    #[serde(rename = "SESSION_STORAGE")]
    #[doc = ""]
    SessionStorage,
    #[serde(rename = "WEB_SOCKETS")]
    #[doc = ""]
    WebSockets,
    #[serde(rename = "WEB_SQL_DATABASE")]
    #[doc = ""]
    WebSqlDatabase,
    #[serde(rename = "WEB_WORKERS")]
    #[doc = ""]
    WebWorkers,
    #[serde(rename = "GEO_LOCATION")]
    #[doc = ""]
    GeoLocation,
    #[serde(rename = "INLINE_SVG")]
    #[doc = ""]
    InlineSvg,
    #[serde(rename = "SMIL")]
    #[doc = ""]
    Smil,
    #[serde(rename = "SVG_HREF")]
    #[doc = ""]
    SvgHref,
    #[serde(rename = "SVG_CLIP_PATHS")]
    #[doc = ""]
    SvgClipPaths,
    #[serde(rename = "TOUCH")]
    #[doc = ""]
    Touch,
    #[serde(rename = "WEBGL")]
    #[doc = ""]
    Webgl,
    #[serde(rename = "SVG_FILTERS")]
    #[doc = ""]
    SvgFilters,
    #[serde(rename = "SVG_FE_IMAGE")]
    #[doc = ""]
    SvgFeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of rich media asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetDisplayTypeEnum {
    #[serde(rename = "ASSET_DISPLAY_TYPE_INPAGE")]
    #[doc = ""]
    AssetDisplayTypeInpage,
    #[serde(rename = "ASSET_DISPLAY_TYPE_FLOATING")]
    #[doc = ""]
    AssetDisplayTypeFloating,
    #[serde(rename = "ASSET_DISPLAY_TYPE_OVERLAY")]
    #[doc = ""]
    AssetDisplayTypeOverlay,
    #[serde(rename = "ASSET_DISPLAY_TYPE_EXPANDING")]
    #[doc = ""]
    AssetDisplayTypeExpanding,
    #[serde(rename = "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH")]
    #[doc = ""]
    AssetDisplayTypeFlashInFlash,
    #[serde(rename = "ASSET_DISPLAY_TYPE_FLASH_IN_FLASH_EXPANDING")]
    #[doc = ""]
    AssetDisplayTypeFlashInFlashExpanding,
    #[serde(rename = "ASSET_DISPLAY_TYPE_PEEL_DOWN")]
    #[doc = ""]
    AssetDisplayTypePeelDown,
    #[serde(rename = "ASSET_DISPLAY_TYPE_VPAID_LINEAR")]
    #[doc = ""]
    AssetDisplayTypeVpaidLinear,
    #[serde(rename = "ASSET_DISPLAY_TYPE_VPAID_NON_LINEAR")]
    #[doc = ""]
    AssetDisplayTypeVpaidNonLinear,
    #[serde(rename = "ASSET_DISPLAY_TYPE_BACKDROP")]
    #[doc = ""]
    AssetDisplayTypeBackdrop,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Duration type for which an asset will be displayed. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetDurationTypeEnum {
    #[serde(rename = "ASSET_DURATION_TYPE_AUTO")]
    #[doc = ""]
    AssetDurationTypeAuto,
    #[serde(rename = "ASSET_DURATION_TYPE_NONE")]
    #[doc = ""]
    AssetDurationTypeNone,
    #[serde(rename = "ASSET_DURATION_TYPE_CUSTOM")]
    #[doc = ""]
    AssetDurationTypeCustom,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Orientation of video asset. This is a read-only, auto-generated field."]
pub enum CreativeAssetOrientationEnum {
    #[serde(rename = "LANDSCAPE")]
    #[doc = ""]
    Landscape,
    #[serde(rename = "PORTRAIT")]
    #[doc = ""]
    Portrait,
    #[serde(rename = "SQUARE")]
    #[doc = ""]
    Square,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Offset left unit for an asset. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetPositionLeftUnitEnum {
    #[serde(rename = "OFFSET_UNIT_PIXEL")]
    #[doc = ""]
    OffsetUnitPixel,
    #[serde(rename = "OFFSET_UNIT_PERCENT")]
    #[doc = ""]
    OffsetUnitPercent,
    #[serde(rename = "OFFSET_UNIT_PIXEL_FROM_CENTER")]
    #[doc = ""]
    OffsetUnitPixelFromCenter,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Offset top unit for an asset. This is a read-only field if the asset displayType is ASSET_DISPLAY_TYPE_OVERLAY. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetPositionTopUnitEnum {
    #[serde(rename = "OFFSET_UNIT_PIXEL")]
    #[doc = ""]
    OffsetUnitPixel,
    #[serde(rename = "OFFSET_UNIT_PERCENT")]
    #[doc = ""]
    OffsetUnitPercent,
    #[serde(rename = "OFFSET_UNIT_PIXEL_FROM_CENTER")]
    #[doc = ""]
    OffsetUnitPixelFromCenter,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Role of the asset in relation to creative. Applicable to all but the following creative types: all REDIRECT and TRACKING_TEXT. This is a required field. PRIMARY applies to DISPLAY, FLASH_INPAGE, HTML5_BANNER, IMAGE, DISPLAY_IMAGE_GALLERY, all RICH_MEDIA (which may contain multiple primary assets), and all VPAID creatives. BACKUP_IMAGE applies to FLASH_INPAGE, HTML5_BANNER, all RICH_MEDIA, and all VPAID creatives. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE. ADDITIONAL_IMAGE and ADDITIONAL_FLASH apply to FLASH_INPAGE creatives. OTHER refers to assets from sources other than Campaign Manager, such as Studio uploaded assets, applicable to all RICH_MEDIA and all VPAID creatives. PARENT_VIDEO refers to videos uploaded by the user in Campaign Manager and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. TRANSCODED_VIDEO refers to videos transcoded by Campaign Manager from PARENT_VIDEO assets and is applicable to INSTREAM_VIDEO and VPAID_LINEAR_VIDEO creatives. ALTERNATE_VIDEO refers to the Campaign Manager representation of child asset videos from Studio, and is applicable to VPAID_LINEAR_VIDEO creatives. These cannot be added or removed within Campaign Manager. For VPAID_LINEAR_VIDEO creatives, PARENT_VIDEO, TRANSCODED_VIDEO and ALTERNATE_VIDEO assets that are marked active serve as backup in case the VPAID creative cannot be served. Only PARENT_VIDEO assets can be added or removed for an INSTREAM_VIDEO or VPAID_LINEAR_VIDEO creative. PARENT_AUDIO refers to audios uploaded by the user in Campaign Manager and is applicable to INSTREAM_AUDIO creatives. TRANSCODED_AUDIO refers to audios transcoded by Campaign Manager from PARENT_AUDIO assets and is applicable to INSTREAM_AUDIO creatives. "]
pub enum CreativeAssetRoleEnum {
    #[serde(rename = "PRIMARY")]
    #[doc = ""]
    Primary,
    #[serde(rename = "BACKUP_IMAGE")]
    #[doc = ""]
    BackupImage,
    #[serde(rename = "ADDITIONAL_IMAGE")]
    #[doc = ""]
    AdditionalImage,
    #[serde(rename = "ADDITIONAL_FLASH")]
    #[doc = ""]
    AdditionalFlash,
    #[serde(rename = "PARENT_VIDEO")]
    #[doc = ""]
    ParentVideo,
    #[serde(rename = "TRANSCODED_VIDEO")]
    #[doc = ""]
    TranscodedVideo,
    #[serde(rename = "OTHER")]
    #[doc = ""]
    Other,
    #[serde(rename = "ALTERNATE_VIDEO")]
    #[doc = ""]
    AlternateVideo,
    #[serde(rename = "PARENT_AUDIO")]
    #[doc = ""]
    ParentAudio,
    #[serde(rename = "TRANSCODED_AUDIO")]
    #[doc = ""]
    TranscodedAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Initial wait time type before making the asset visible. Applicable to the following creative types: all RICH_MEDIA."]
pub enum CreativeAssetStartTimeTypeEnum {
    #[serde(rename = "ASSET_START_TIME_TYPE_NONE")]
    #[doc = ""]
    AssetStartTimeTypeNone,
    #[serde(rename = "ASSET_START_TIME_TYPE_CUSTOM")]
    #[doc = ""]
    AssetStartTimeTypeCustom,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Window mode options for flash assets. Applicable to the following creative types: FLASH_INPAGE, RICH_MEDIA_DISPLAY_EXPANDING, RICH_MEDIA_IM_EXPAND, RICH_MEDIA_DISPLAY_BANNER, and RICH_MEDIA_INPAGE_FLOATING."]
pub enum CreativeAssetWindowModeEnum {
    #[serde(rename = "OPAQUE")]
    #[doc = ""]
    Opaque,
    #[serde(rename = "WINDOW")]
    #[doc = ""]
    Window,
    #[serde(rename = "TRANSPARENT")]
    #[doc = ""]
    Transparent,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Asset ID."]
pub struct CreativeAssetId {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the creative asset. This is a required field while inserting an asset. After insertion, this assetIdentifier is used to identify the uploaded asset. Characters in the name must be alphanumeric or one of the following: \".-_ \". Spaces are allowed."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE."]
    pub _type: ::std::option::Option<CreativeAssetIdTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE."]
pub enum CreativeAssetIdTypeEnum {
    #[serde(rename = "IMAGE")]
    #[doc = ""]
    Image,
    #[serde(rename = "FLASH")]
    #[doc = ""]
    Flash,
    #[serde(rename = "VIDEO")]
    #[doc = ""]
    Video,
    #[serde(rename = "HTML")]
    #[doc = ""]
    Html,
    #[serde(rename = "HTML_IMAGE")]
    #[doc = ""]
    HtmlImage,
    #[serde(rename = "AUDIO")]
    #[doc = ""]
    Audio,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CreativeAssets contains properties of a creative asset file which will be uploaded or has already been uploaded. Refer to the creative sample code for how to upload assets and insert a creative."]
pub struct CreativeAssetMetadata {
    #[serde(rename = "assetIdentifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative asset. This is a required field."]
    pub asset_identifier: ::std::option::Option<::std::boxed::Box<CreativeAssetId>>,
    #[serde(rename = "clickTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of detected click tags for assets. This is a read-only, auto-generated field. This field is empty for a rich media asset."]
    pub click_tags: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClickTag>>>,
    #[serde(rename = "detectedFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field."]
    pub detected_features:
        ::std::option::Option<::std::vec::Vec<CreativeAssetMetadataDetectedFeaturesEnum>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numeric ID of the asset. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the numeric ID of the asset. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeAssetMetadata\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warnedValidationRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rules validated during code generation that generated a warning. This is a read-only, auto-generated field. Possible values are: - \"ADMOB_REFERENCED\" - \"ASSET_FORMAT_UNSUPPORTED_DCM\" - \"ASSET_INVALID\" - \"CLICK_TAG_HARD_CODED\" - \"CLICK_TAG_INVALID\" - \"CLICK_TAG_IN_GWD\" - \"CLICK_TAG_MISSING\" - \"CLICK_TAG_MORE_THAN_ONE\" - \"CLICK_TAG_NON_TOP_LEVEL\" - \"COMPONENT_UNSUPPORTED_DCM\" - \"ENABLER_UNSUPPORTED_METHOD_DCM\" - \"EXTERNAL_FILE_REFERENCED\" - \"FILE_DETAIL_EMPTY\" - \"FILE_TYPE_INVALID\" - \"GWD_PROPERTIES_INVALID\" - \"HTML5_FEATURE_UNSUPPORTED\" - \"LINKED_FILE_NOT_FOUND\" - \"MAX_FLASH_VERSION_11\" - \"MRAID_REFERENCED\" - \"NOT_SSL_COMPLIANT\" - \"ORPHANED_ASSET\" - \"PRIMARY_HTML_MISSING\" - \"SVG_INVALID\" - \"ZIP_INVALID\" "]
    pub warned_validation_rules:
        ::std::option::Option<::std::vec::Vec<CreativeAssetMetadataWarnedValidationRulesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeAssetMetadataDetectedFeaturesEnum {
    #[serde(rename = "CSS_FONT_FACE")]
    #[doc = ""]
    CssFontFace,
    #[serde(rename = "CSS_BACKGROUND_SIZE")]
    #[doc = ""]
    CssBackgroundSize,
    #[serde(rename = "CSS_BORDER_IMAGE")]
    #[doc = ""]
    CssBorderImage,
    #[serde(rename = "CSS_BORDER_RADIUS")]
    #[doc = ""]
    CssBorderRadius,
    #[serde(rename = "CSS_BOX_SHADOW")]
    #[doc = ""]
    CssBoxShadow,
    #[serde(rename = "CSS_FLEX_BOX")]
    #[doc = ""]
    CssFlexBox,
    #[serde(rename = "CSS_HSLA")]
    #[doc = ""]
    CssHsla,
    #[serde(rename = "CSS_MULTIPLE_BGS")]
    #[doc = ""]
    CssMultipleBgs,
    #[serde(rename = "CSS_OPACITY")]
    #[doc = ""]
    CssOpacity,
    #[serde(rename = "CSS_RGBA")]
    #[doc = ""]
    CssRgba,
    #[serde(rename = "CSS_TEXT_SHADOW")]
    #[doc = ""]
    CssTextShadow,
    #[serde(rename = "CSS_ANIMATIONS")]
    #[doc = ""]
    CssAnimations,
    #[serde(rename = "CSS_COLUMNS")]
    #[doc = ""]
    CssColumns,
    #[serde(rename = "CSS_GENERATED_CONTENT")]
    #[doc = ""]
    CssGeneratedContent,
    #[serde(rename = "CSS_GRADIENTS")]
    #[doc = ""]
    CssGradients,
    #[serde(rename = "CSS_REFLECTIONS")]
    #[doc = ""]
    CssReflections,
    #[serde(rename = "CSS_TRANSFORMS")]
    #[doc = ""]
    CssTransforms,
    #[serde(rename = "CSS_TRANSFORMS3D")]
    #[doc = ""]
    CssTransforms3D,
    #[serde(rename = "CSS_TRANSITIONS")]
    #[doc = ""]
    CssTransitions,
    #[serde(rename = "APPLICATION_CACHE")]
    #[doc = ""]
    ApplicationCache,
    #[serde(rename = "CANVAS")]
    #[doc = ""]
    Canvas,
    #[serde(rename = "CANVAS_TEXT")]
    #[doc = ""]
    CanvasText,
    #[serde(rename = "DRAG_AND_DROP")]
    #[doc = ""]
    DragAndDrop,
    #[serde(rename = "HASH_CHANGE")]
    #[doc = ""]
    HashChange,
    #[serde(rename = "HISTORY")]
    #[doc = ""]
    History,
    #[serde(rename = "AUDIO")]
    #[doc = ""]
    Audio,
    #[serde(rename = "VIDEO")]
    #[doc = ""]
    Video,
    #[serde(rename = "INDEXED_DB")]
    #[doc = ""]
    IndexedDb,
    #[serde(rename = "INPUT_ATTR_AUTOCOMPLETE")]
    #[doc = ""]
    InputAttrAutocomplete,
    #[serde(rename = "INPUT_ATTR_AUTOFOCUS")]
    #[doc = ""]
    InputAttrAutofocus,
    #[serde(rename = "INPUT_ATTR_LIST")]
    #[doc = ""]
    InputAttrList,
    #[serde(rename = "INPUT_ATTR_PLACEHOLDER")]
    #[doc = ""]
    InputAttrPlaceholder,
    #[serde(rename = "INPUT_ATTR_MAX")]
    #[doc = ""]
    InputAttrMax,
    #[serde(rename = "INPUT_ATTR_MIN")]
    #[doc = ""]
    InputAttrMin,
    #[serde(rename = "INPUT_ATTR_MULTIPLE")]
    #[doc = ""]
    InputAttrMultiple,
    #[serde(rename = "INPUT_ATTR_PATTERN")]
    #[doc = ""]
    InputAttrPattern,
    #[serde(rename = "INPUT_ATTR_REQUIRED")]
    #[doc = ""]
    InputAttrRequired,
    #[serde(rename = "INPUT_ATTR_STEP")]
    #[doc = ""]
    InputAttrStep,
    #[serde(rename = "INPUT_TYPE_SEARCH")]
    #[doc = ""]
    InputTypeSearch,
    #[serde(rename = "INPUT_TYPE_TEL")]
    #[doc = ""]
    InputTypeTel,
    #[serde(rename = "INPUT_TYPE_URL")]
    #[doc = ""]
    InputTypeUrl,
    #[serde(rename = "INPUT_TYPE_EMAIL")]
    #[doc = ""]
    InputTypeEmail,
    #[serde(rename = "INPUT_TYPE_DATETIME")]
    #[doc = ""]
    InputTypeDatetime,
    #[serde(rename = "INPUT_TYPE_DATE")]
    #[doc = ""]
    InputTypeDate,
    #[serde(rename = "INPUT_TYPE_MONTH")]
    #[doc = ""]
    InputTypeMonth,
    #[serde(rename = "INPUT_TYPE_WEEK")]
    #[doc = ""]
    InputTypeWeek,
    #[serde(rename = "INPUT_TYPE_TIME")]
    #[doc = ""]
    InputTypeTime,
    #[serde(rename = "INPUT_TYPE_DATETIME_LOCAL")]
    #[doc = ""]
    InputTypeDatetimeLocal,
    #[serde(rename = "INPUT_TYPE_NUMBER")]
    #[doc = ""]
    InputTypeNumber,
    #[serde(rename = "INPUT_TYPE_RANGE")]
    #[doc = ""]
    InputTypeRange,
    #[serde(rename = "INPUT_TYPE_COLOR")]
    #[doc = ""]
    InputTypeColor,
    #[serde(rename = "LOCAL_STORAGE")]
    #[doc = ""]
    LocalStorage,
    #[serde(rename = "POST_MESSAGE")]
    #[doc = ""]
    PostMessage,
    #[serde(rename = "SESSION_STORAGE")]
    #[doc = ""]
    SessionStorage,
    #[serde(rename = "WEB_SOCKETS")]
    #[doc = ""]
    WebSockets,
    #[serde(rename = "WEB_SQL_DATABASE")]
    #[doc = ""]
    WebSqlDatabase,
    #[serde(rename = "WEB_WORKERS")]
    #[doc = ""]
    WebWorkers,
    #[serde(rename = "GEO_LOCATION")]
    #[doc = ""]
    GeoLocation,
    #[serde(rename = "INLINE_SVG")]
    #[doc = ""]
    InlineSvg,
    #[serde(rename = "SMIL")]
    #[doc = ""]
    Smil,
    #[serde(rename = "SVG_HREF")]
    #[doc = ""]
    SvgHref,
    #[serde(rename = "SVG_CLIP_PATHS")]
    #[doc = ""]
    SvgClipPaths,
    #[serde(rename = "TOUCH")]
    #[doc = ""]
    Touch,
    #[serde(rename = "WEBGL")]
    #[doc = ""]
    Webgl,
    #[serde(rename = "SVG_FILTERS")]
    #[doc = ""]
    SvgFilters,
    #[serde(rename = "SVG_FE_IMAGE")]
    #[doc = ""]
    SvgFeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CreativeAssetMetadataWarnedValidationRulesEnum {
    #[serde(rename = "CLICK_TAG_NON_TOP_LEVEL")]
    #[doc = ""]
    ClickTagNonTopLevel,
    #[serde(rename = "CLICK_TAG_MISSING")]
    #[doc = ""]
    ClickTagMissing,
    #[serde(rename = "CLICK_TAG_MORE_THAN_ONE")]
    #[doc = ""]
    ClickTagMoreThanOne,
    #[serde(rename = "CLICK_TAG_INVALID")]
    #[doc = ""]
    ClickTagInvalid,
    #[serde(rename = "ORPHANED_ASSET")]
    #[doc = ""]
    OrphanedAsset,
    #[serde(rename = "PRIMARY_HTML_MISSING")]
    #[doc = ""]
    PrimaryHtmlMissing,
    #[serde(rename = "EXTERNAL_FILE_REFERENCED")]
    #[doc = ""]
    ExternalFileReferenced,
    #[serde(rename = "MRAID_REFERENCED")]
    #[doc = ""]
    MraidReferenced,
    #[serde(rename = "ADMOB_REFERENCED")]
    #[doc = ""]
    AdmobReferenced,
    #[serde(rename = "FILE_TYPE_INVALID")]
    #[doc = ""]
    FileTypeInvalid,
    #[serde(rename = "ZIP_INVALID")]
    #[doc = ""]
    ZipInvalid,
    #[serde(rename = "LINKED_FILE_NOT_FOUND")]
    #[doc = ""]
    LinkedFileNotFound,
    #[serde(rename = "MAX_FLASH_VERSION_11")]
    #[doc = ""]
    MaxFlashVersion11,
    #[serde(rename = "NOT_SSL_COMPLIANT")]
    #[doc = ""]
    NotSslCompliant,
    #[serde(rename = "FILE_DETAIL_EMPTY")]
    #[doc = ""]
    FileDetailEmpty,
    #[serde(rename = "ASSET_INVALID")]
    #[doc = ""]
    AssetInvalid,
    #[serde(rename = "GWD_PROPERTIES_INVALID")]
    #[doc = ""]
    GwdPropertiesInvalid,
    #[serde(rename = "ENABLER_UNSUPPORTED_METHOD_DCM")]
    #[doc = ""]
    EnablerUnsupportedMethodDcm,
    #[serde(rename = "ASSET_FORMAT_UNSUPPORTED_DCM")]
    #[doc = ""]
    AssetFormatUnsupportedDcm,
    #[serde(rename = "COMPONENT_UNSUPPORTED_DCM")]
    #[doc = ""]
    ComponentUnsupportedDcm,
    #[serde(rename = "HTML5_FEATURE_UNSUPPORTED")]
    #[doc = ""]
    Html5FeatureUnsupported,
    #[serde(rename = "CLICK_TAG_IN_GWD")]
    #[doc = ""]
    ClickTagInGwd,
    #[serde(rename = "CLICK_TAG_HARD_CODED")]
    #[doc = ""]
    ClickTagHardCoded,
    #[serde(rename = "SVG_INVALID")]
    #[doc = ""]
    SvgInvalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates the list of rules for asset selection and a default asset in case none of the rules match. Applicable to INSTREAM_VIDEO creatives."]
pub struct CreativeAssetSelection {
    #[serde(rename = "defaultAssetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A creativeAssets[].id. This should refer to one of the parent assets in this creative, and will be served if none of the rules match. This is a required field."]
    pub default_asset_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rules determine which asset will be served to a viewer. Rules will be evaluated in the order in which they are stored in this list. This list must contain at least one rule. Applicable to INSTREAM_VIDEO creatives."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Rule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Assignment."]
pub struct CreativeAssignment {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this creative assignment is active. When true, the creative will be included in the ad's rotation."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "applyEventTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether applicable event tags should fire when this creative assignment is rendered. If this value is unset when the ad is inserted or updated, it will default to true for all creative types EXCEPT for INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and INSTREAM_VIDEO."]
    pub apply_event_tags: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL of the creative assignment."]
    pub click_through_url: ::std::option::Option<::std::boxed::Box<ClickThroughUrl>>,
    #[serde(rename = "companionCreativeOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Companion creative overrides for this creative assignment. Applicable to video ads."]
    pub companion_creative_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompanionClickThroughOverride>>>,
    #[serde(rename = "creativeGroupAssignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative group assignments for this creative assignment. Only one assignment per creative group number is allowed for a maximum of two assignments."]
    pub creative_group_assignments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeGroupAssignment>>>,
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative to be assigned. This is a required field."]
    pub creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the creative. This is a read-only, auto-generated field."]
    pub creative_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "richMediaExitOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rich media exit overrides for this creative assignment. Applicable when the creative type is any of the following: - DISPLAY - RICH_MEDIA_INPAGE - RICH_MEDIA_INPAGE_FLOATING - RICH_MEDIA_IM_EXPAND - RICH_MEDIA_EXPANDING - RICH_MEDIA_INTERSTITIAL_FLOAT - RICH_MEDIA_MOBILE_IN_APP - RICH_MEDIA_MULTI_FLOATING - RICH_MEDIA_PEEL_DOWN - VPAID_LINEAR - VPAID_NON_LINEAR "]
    pub rich_media_exit_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RichMediaExitOverride>>>,
    #[serde(rename = "sequence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sequence number of the creative assignment, applicable when the rotation type is CREATIVE_ROTATION_TYPE_SEQUENTIAL. Acceptable values are 1 to 65535, inclusive."]
    pub sequence: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the creative to be assigned is SSL-compliant. This is a read-only field that is auto-generated when the ad is inserted or updated."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Weight of the creative assignment, applicable when the rotation type is CREATIVE_ROTATION_TYPE_RANDOM. Value must be greater than or equal to 1."]
    pub weight: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Click-through URL"]
pub struct CreativeClickThroughUrl {
    #[serde(rename = "computedClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: - If landingPageId is specified then that landing page's URL is assigned to this field. - Otherwise, the customClickThroughUrl is assigned to this field. "]
    pub computed_click_through_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom click-through URL. Applicable if the landingPageId field is left unset."]
    pub custom_click_through_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "landingPageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the landing page for the click-through URL."]
    pub landing_page_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Custom Event."]
pub struct CreativeCustomEvent {
    #[serde(rename = "advertiserCustomEventId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID of this event used by Reporting and Data Transfer. This is a read-only field."]
    pub advertiser_custom_event_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserCustomEventName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-entered name for the event."]
    pub advertiser_custom_event_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserCustomEventType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the event. This is a read-only field."]
    pub advertiser_custom_event_type:
        ::std::option::Option<CreativeCustomEventAdvertiserCustomEventTypeEnum>,
    #[serde(rename = "artworkLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Artwork label column, used to link events in Campaign Manager back to events in Studio. This is a required field and should not be modified after insertion."]
    pub artwork_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "artworkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Artwork type used by the creative.This is a read-only field."]
    pub artwork_type: ::std::option::Option<CreativeCustomEventArtworkTypeEnum>,
    #[serde(rename = "exitClickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exit click-through URL for the event. This field is used only for exit events."]
    pub exit_click_through_url: ::std::option::Option<::std::boxed::Box<CreativeClickThroughUrl>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this event. This is a required field and should not be modified after insertion."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "popupWindowProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties for rich media popup windows. This field is used only for exit events."]
    pub popup_window_properties: ::std::option::Option<::std::boxed::Box<PopupWindowProperties>>,
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target type used by the event."]
    pub target_type: ::std::option::Option<CreativeCustomEventTargetTypeEnum>,
    #[serde(rename = "videoReportingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video reporting ID, used to differentiate multiple videos in a single creative. This is a read-only field."]
    pub video_reporting_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the event. This is a read-only field."]
pub enum CreativeCustomEventAdvertiserCustomEventTypeEnum {
    #[serde(rename = "ADVERTISER_EVENT_TIMER")]
    #[doc = ""]
    AdvertiserEventTimer,
    #[serde(rename = "ADVERTISER_EVENT_EXIT")]
    #[doc = ""]
    AdvertiserEventExit,
    #[serde(rename = "ADVERTISER_EVENT_COUNTER")]
    #[doc = ""]
    AdvertiserEventCounter,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Artwork type used by the creative.This is a read-only field."]
pub enum CreativeCustomEventArtworkTypeEnum {
    #[serde(rename = "ARTWORK_TYPE_FLASH")]
    #[doc = ""]
    ArtworkTypeFlash,
    #[serde(rename = "ARTWORK_TYPE_HTML5")]
    #[doc = ""]
    ArtworkTypeHtml5,
    #[serde(rename = "ARTWORK_TYPE_MIXED")]
    #[doc = ""]
    ArtworkTypeMixed,
    #[serde(rename = "ARTWORK_TYPE_IMAGE")]
    #[doc = ""]
    ArtworkTypeImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target type used by the event."]
pub enum CreativeCustomEventTargetTypeEnum {
    #[serde(rename = "TARGET_BLANK")]
    #[doc = ""]
    TargetBlank,
    #[serde(rename = "TARGET_TOP")]
    #[doc = ""]
    TargetTop,
    #[serde(rename = "TARGET_SELF")]
    #[doc = ""]
    TargetSelf,
    #[serde(rename = "TARGET_PARENT")]
    #[doc = ""]
    TargetParent,
    #[serde(rename = "TARGET_POPUP")]
    #[doc = ""]
    TargetPopup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a creative field."]
pub struct CreativeField {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this creative field. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this creative field. This is a required field on insertion."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this creative field. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeField\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this creative field. This is a required field and must be less than 256 characters long and unique among creative fields of the same advertiser."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this creative field. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Field Assignment."]
pub struct CreativeFieldAssignment {
    #[serde(rename = "creativeFieldId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative field."]
    pub creative_field_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeFieldValueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative field value."]
    pub creative_field_value_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a creative field value."]
pub struct CreativeFieldValue {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this creative field value. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeFieldValue\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of this creative field value. It needs to be less than 256 characters in length and unique per creative field."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Field Value List Response"]
pub struct CreativeFieldValuesListResponse {
    #[serde(rename = "creativeFieldValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative field value collection."]
    pub creative_field_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeFieldValue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeFieldValuesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Field List Response"]
pub struct CreativeFieldsListResponse {
    #[serde(rename = "creativeFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative field collection."]
    pub creative_fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeField>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeFieldsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a creative group."]
pub struct CreativeGroup {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this creative group. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this creative group. This is a required field on insertion."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "groupNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subgroup of the creative group. Assign your creative groups to a subgroup in order to filter or manage them more easily. This field is required on insertion and is read-only after insertion. Acceptable values are 1 to 2, inclusive."]
    pub group_number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this creative group. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this creative group. This is a required field and must be less than 256 characters long and unique among creative groups of the same advertiser."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this creative group. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Group Assignment."]
pub struct CreativeGroupAssignment {
    #[serde(rename = "creativeGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the creative group to be assigned."]
    pub creative_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeGroupNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative group number of the creative group assignment."]
    pub creative_group_number:
        ::std::option::Option<CreativeGroupAssignmentCreativeGroupNumberEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Creative group number of the creative group assignment."]
pub enum CreativeGroupAssignmentCreativeGroupNumberEnum {
    #[serde(rename = "CREATIVE_GROUP_ONE")]
    #[doc = ""]
    CreativeGroupOne,
    #[serde(rename = "CREATIVE_GROUP_TWO")]
    #[doc = ""]
    CreativeGroupTwo,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Group List Response"]
pub struct CreativeGroupsListResponse {
    #[serde(rename = "creativeGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative group collection."]
    pub creative_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeGroup>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativeGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative optimization settings."]
pub struct CreativeOptimizationConfiguration {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this creative optimization config. This field is auto-generated when the campaign is inserted or updated. It can be null for existing campaigns."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this creative optimization config. This is a required field and must be less than 129 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optimizationActivitys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of optimization activities associated with this configuration."]
    pub optimization_activitys:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OptimizationActivity>>>,
    #[serde(rename = "optimizationModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optimization model for this configuration."]
    pub optimization_model:
        ::std::option::Option<CreativeOptimizationConfigurationOptimizationModelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optimization model for this configuration."]
pub enum CreativeOptimizationConfigurationOptimizationModelEnum {
    #[serde(rename = "CLICK")]
    #[doc = ""]
    Click,
    #[serde(rename = "POST_CLICK")]
    #[doc = ""]
    PostClick,
    #[serde(rename = "POST_IMPRESSION")]
    #[doc = ""]
    PostImpression,
    #[serde(rename = "POST_CLICK_AND_IMPRESSION")]
    #[doc = ""]
    PostClickAndImpression,
    #[serde(rename = "VIDEO_COMPLETION")]
    #[doc = ""]
    VideoCompletion,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative Rotation."]
pub struct CreativeRotation {
    #[serde(rename = "creativeAssignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative assignments in this creative rotation."]
    pub creative_assignments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeAssignment>>>,
    #[serde(rename = "creativeOptimizationConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative optimization configuration that is used by this ad. It should refer to one of the existing optimization configurations in the ad's campaign. If it is unset or set to 0, then the campaign's default optimization configuration will be used for this ad."]
    pub creative_optimization_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of creative rotation. Can be used to specify whether to use sequential or random rotation."]
    pub _type: ::std::option::Option<CreativeRotationTypeEnum>,
    #[serde(rename = "weightCalculationStrategy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Strategy for calculating weights. Used with CREATIVE_ROTATION_TYPE_RANDOM."]
    pub weight_calculation_strategy:
        ::std::option::Option<CreativeRotationWeightCalculationStrategyEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of creative rotation. Can be used to specify whether to use sequential or random rotation."]
pub enum CreativeRotationTypeEnum {
    #[serde(rename = "CREATIVE_ROTATION_TYPE_SEQUENTIAL")]
    #[doc = ""]
    CreativeRotationTypeSequential,
    #[serde(rename = "CREATIVE_ROTATION_TYPE_RANDOM")]
    #[doc = ""]
    CreativeRotationTypeRandom,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Strategy for calculating weights. Used with CREATIVE_ROTATION_TYPE_RANDOM."]
pub enum CreativeRotationWeightCalculationStrategyEnum {
    #[serde(rename = "WEIGHT_STRATEGY_EQUAL")]
    #[doc = ""]
    WeightStrategyEqual,
    #[serde(rename = "WEIGHT_STRATEGY_CUSTOM")]
    #[doc = ""]
    WeightStrategyCustom,
    #[serde(rename = "WEIGHT_STRATEGY_HIGHEST_CTR")]
    #[doc = ""]
    WeightStrategyHighestCtr,
    #[serde(rename = "WEIGHT_STRATEGY_OPTIMIZED")]
    #[doc = ""]
    WeightStrategyOptimized,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative List Response"]
pub struct CreativesListResponse {
    #[serde(rename = "creatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative collection."]
    pub creatives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Creative>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#creativesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fields that are compatible to be selected for a report of type \"CROSS_DIMENSION_REACH\"."]
pub struct CrossDimensionReachReportCompatibleFields {
    #[serde(rename = "breakdown")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"breakdown\" section of the report."]
    pub breakdown: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensionFilters\" section of the report."]
    pub dimension_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#crossDimensionReachReportCompatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"metricNames\" section of the report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "overlapMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"overlapMetricNames\" section of the report."]
    pub overlap_metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A custom floodlight variable."]
pub struct CustomFloodlightVariable {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#customFloodlightVariable\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of custom floodlight variable to supply a value for. These map to the \"u[1-20]=\" in the tags."]
    pub _type: ::std::option::Option<CustomFloodlightVariableTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the custom floodlight variable. The length of string must not exceed 100 characters."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of custom floodlight variable to supply a value for. These map to the \"u[1-20]=\" in the tags."]
pub enum CustomFloodlightVariableTypeEnum {
    #[serde(rename = "U1")]
    #[doc = ""]
    U1,
    #[serde(rename = "U2")]
    #[doc = ""]
    U2,
    #[serde(rename = "U3")]
    #[doc = ""]
    U3,
    #[serde(rename = "U4")]
    #[doc = ""]
    U4,
    #[serde(rename = "U5")]
    #[doc = ""]
    U5,
    #[serde(rename = "U6")]
    #[doc = ""]
    U6,
    #[serde(rename = "U7")]
    #[doc = ""]
    U7,
    #[serde(rename = "U8")]
    #[doc = ""]
    U8,
    #[serde(rename = "U9")]
    #[doc = ""]
    U9,
    #[serde(rename = "U10")]
    #[doc = ""]
    U10,
    #[serde(rename = "U11")]
    #[doc = ""]
    U11,
    #[serde(rename = "U12")]
    #[doc = ""]
    U12,
    #[serde(rename = "U13")]
    #[doc = ""]
    U13,
    #[serde(rename = "U14")]
    #[doc = ""]
    U14,
    #[serde(rename = "U15")]
    #[doc = ""]
    U15,
    #[serde(rename = "U16")]
    #[doc = ""]
    U16,
    #[serde(rename = "U17")]
    #[doc = ""]
    U17,
    #[serde(rename = "U18")]
    #[doc = ""]
    U18,
    #[serde(rename = "U19")]
    #[doc = ""]
    U19,
    #[serde(rename = "U20")]
    #[doc = ""]
    U20,
    #[serde(rename = "U21")]
    #[doc = ""]
    U21,
    #[serde(rename = "U22")]
    #[doc = ""]
    U22,
    #[serde(rename = "U23")]
    #[doc = ""]
    U23,
    #[serde(rename = "U24")]
    #[doc = ""]
    U24,
    #[serde(rename = "U25")]
    #[doc = ""]
    U25,
    #[serde(rename = "U26")]
    #[doc = ""]
    U26,
    #[serde(rename = "U27")]
    #[doc = ""]
    U27,
    #[serde(rename = "U28")]
    #[doc = ""]
    U28,
    #[serde(rename = "U29")]
    #[doc = ""]
    U29,
    #[serde(rename = "U30")]
    #[doc = ""]
    U30,
    #[serde(rename = "U31")]
    #[doc = ""]
    U31,
    #[serde(rename = "U32")]
    #[doc = ""]
    U32,
    #[serde(rename = "U33")]
    #[doc = ""]
    U33,
    #[serde(rename = "U34")]
    #[doc = ""]
    U34,
    #[serde(rename = "U35")]
    #[doc = ""]
    U35,
    #[serde(rename = "U36")]
    #[doc = ""]
    U36,
    #[serde(rename = "U37")]
    #[doc = ""]
    U37,
    #[serde(rename = "U38")]
    #[doc = ""]
    U38,
    #[serde(rename = "U39")]
    #[doc = ""]
    U39,
    #[serde(rename = "U40")]
    #[doc = ""]
    U40,
    #[serde(rename = "U41")]
    #[doc = ""]
    U41,
    #[serde(rename = "U42")]
    #[doc = ""]
    U42,
    #[serde(rename = "U43")]
    #[doc = ""]
    U43,
    #[serde(rename = "U44")]
    #[doc = ""]
    U44,
    #[serde(rename = "U45")]
    #[doc = ""]
    U45,
    #[serde(rename = "U46")]
    #[doc = ""]
    U46,
    #[serde(rename = "U47")]
    #[doc = ""]
    U47,
    #[serde(rename = "U48")]
    #[doc = ""]
    U48,
    #[serde(rename = "U49")]
    #[doc = ""]
    U49,
    #[serde(rename = "U50")]
    #[doc = ""]
    U50,
    #[serde(rename = "U51")]
    #[doc = ""]
    U51,
    #[serde(rename = "U52")]
    #[doc = ""]
    U52,
    #[serde(rename = "U53")]
    #[doc = ""]
    U53,
    #[serde(rename = "U54")]
    #[doc = ""]
    U54,
    #[serde(rename = "U55")]
    #[doc = ""]
    U55,
    #[serde(rename = "U56")]
    #[doc = ""]
    U56,
    #[serde(rename = "U57")]
    #[doc = ""]
    U57,
    #[serde(rename = "U58")]
    #[doc = ""]
    U58,
    #[serde(rename = "U59")]
    #[doc = ""]
    U59,
    #[serde(rename = "U60")]
    #[doc = ""]
    U60,
    #[serde(rename = "U61")]
    #[doc = ""]
    U61,
    #[serde(rename = "U62")]
    #[doc = ""]
    U62,
    #[serde(rename = "U63")]
    #[doc = ""]
    U63,
    #[serde(rename = "U64")]
    #[doc = ""]
    U64,
    #[serde(rename = "U65")]
    #[doc = ""]
    U65,
    #[serde(rename = "U66")]
    #[doc = ""]
    U66,
    #[serde(rename = "U67")]
    #[doc = ""]
    U67,
    #[serde(rename = "U68")]
    #[doc = ""]
    U68,
    #[serde(rename = "U69")]
    #[doc = ""]
    U69,
    #[serde(rename = "U70")]
    #[doc = ""]
    U70,
    #[serde(rename = "U71")]
    #[doc = ""]
    U71,
    #[serde(rename = "U72")]
    #[doc = ""]
    U72,
    #[serde(rename = "U73")]
    #[doc = ""]
    U73,
    #[serde(rename = "U74")]
    #[doc = ""]
    U74,
    #[serde(rename = "U75")]
    #[doc = ""]
    U75,
    #[serde(rename = "U76")]
    #[doc = ""]
    U76,
    #[serde(rename = "U77")]
    #[doc = ""]
    U77,
    #[serde(rename = "U78")]
    #[doc = ""]
    U78,
    #[serde(rename = "U79")]
    #[doc = ""]
    U79,
    #[serde(rename = "U80")]
    #[doc = ""]
    U80,
    #[serde(rename = "U81")]
    #[doc = ""]
    U81,
    #[serde(rename = "U82")]
    #[doc = ""]
    U82,
    #[serde(rename = "U83")]
    #[doc = ""]
    U83,
    #[serde(rename = "U84")]
    #[doc = ""]
    U84,
    #[serde(rename = "U85")]
    #[doc = ""]
    U85,
    #[serde(rename = "U86")]
    #[doc = ""]
    U86,
    #[serde(rename = "U87")]
    #[doc = ""]
    U87,
    #[serde(rename = "U88")]
    #[doc = ""]
    U88,
    #[serde(rename = "U89")]
    #[doc = ""]
    U89,
    #[serde(rename = "U90")]
    #[doc = ""]
    U90,
    #[serde(rename = "U91")]
    #[doc = ""]
    U91,
    #[serde(rename = "U92")]
    #[doc = ""]
    U92,
    #[serde(rename = "U93")]
    #[doc = ""]
    U93,
    #[serde(rename = "U94")]
    #[doc = ""]
    U94,
    #[serde(rename = "U95")]
    #[doc = ""]
    U95,
    #[serde(rename = "U96")]
    #[doc = ""]
    U96,
    #[serde(rename = "U97")]
    #[doc = ""]
    U97,
    #[serde(rename = "U98")]
    #[doc = ""]
    U98,
    #[serde(rename = "U99")]
    #[doc = ""]
    U99,
    #[serde(rename = "U100")]
    #[doc = ""]
    U100,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Custom Rich Media Events group."]
pub struct CustomRichMediaEvents {
    #[serde(rename = "filteredEventIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of custom rich media event IDs. Dimension values must be all of type dfa:richMediaEventTypeIdAndName."]
    pub filtered_event_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#customRichMediaEvents."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom Viewability Metric"]
pub struct CustomViewabilityMetric {
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of the custom viewability metric."]
    pub configuration:
        ::std::option::Option<::std::boxed::Box<CustomViewabilityMetricConfiguration>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the custom viewability metric."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the custom viewability metric."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The attributes, like playtime and percent onscreen, that define the Custom Viewability Metric."]
pub struct CustomViewabilityMetricConfiguration {
    #[serde(rename = "audible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the video must be audible to count an impression."]
    pub audible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "timeMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time in milliseconds the video must play for the Custom Viewability Metric to count an impression. If both this and timePercent are specified, the earlier of the two will be used."]
    pub time_millis: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timePercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of video that must play for the Custom Viewability Metric to count an impression. If both this and timeMillis are specified, the earlier of the two will be used."]
    pub time_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "viewabilityPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of video that must be on screen for the Custom Viewability Metric to count an impression."]
    pub viewability_percent: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a date range."]
pub struct DateRange {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#dateRange."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relativeDateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range relative to the date of when the report is run."]
    pub relative_date_range: ::std::option::Option<DateRangeRelativeDateRangeEnum>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The date range relative to the date of when the report is run."]
pub enum DateRangeRelativeDateRangeEnum {
    #[serde(rename = "TODAY")]
    #[doc = ""]
    Today,
    #[serde(rename = "YESTERDAY")]
    #[doc = ""]
    Yesterday,
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
    #[serde(rename = "LAST_24_MONTHS")]
    #[doc = ""]
    Last24Months,
    #[serde(rename = "LAST_14_DAYS")]
    #[doc = ""]
    Last14Days,
    #[serde(rename = "LAST_60_DAYS")]
    #[doc = ""]
    Last60Days,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Day Part Targeting."]
pub struct DayPartTargeting {
    #[serde(rename = "daysOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Days of the week when the ad will serve. Acceptable values are: - \"SUNDAY\" - \"MONDAY\" - \"TUESDAY\" - \"WEDNESDAY\" - \"THURSDAY\" - \"FRIDAY\" - \"SATURDAY\" "]
    pub days_of_week: ::std::option::Option<::std::vec::Vec<DayPartTargetingDaysOfWeekEnum>>,
    #[serde(rename = "hoursOfDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hours of the day when the ad will serve, where 0 is midnight to 1 AM and 23 is 11 PM to midnight. Can be specified with days of week, in which case the ad would serve during these hours on the specified days. For example if Monday, Wednesday, Friday are the days of week specified and 9-10am, 3-5pm (hours 9, 15, and 16) is specified, the ad would serve Monday, Wednesdays, and Fridays at 9-10am and 3-5pm. Acceptable values are 0 to 23, inclusive."]
    pub hours_of_day: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "userLocalTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to use the user's local time. If false, the America/New York time zone applies."]
    pub user_local_time: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DayPartTargetingDaysOfWeekEnum {
    #[serde(rename = "MONDAY")]
    #[doc = ""]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = ""]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = ""]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = ""]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = ""]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = ""]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = ""]
    Sunday,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a landing page deep link."]
pub struct DeepLink {
    #[serde(rename = "appUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the mobile app being linked to."]
    pub app_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fallbackUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fallback URL. This URL will be served to users who do not have the mobile app installed."]
    pub fallback_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#deepLink\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mobile app targeted by this deep link."]
    pub mobile_app: ::std::option::Option<::std::boxed::Box<MobileApp>>,
    #[serde(rename = "remarketingListIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ads served to users on these remarketing lists will use this deep link. Applicable when mobileApp.directory is APPLE_APP_STORE."]
    pub remarketing_list_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Properties of inheriting and overriding the default click-through event tag. A campaign may override the event tag defined at the advertiser level, and an ad may also override the campaign's setting further."]
pub struct DefaultClickThroughEventTagProperties {
    #[serde(rename = "defaultClickThroughEventTagId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the click-through event tag to apply to all ads in this entity's scope."]
    pub default_click_through_event_tag_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overrideInheritedEventTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this entity should override the inherited default click-through event tag with its own defined value."]
    pub override_inherited_event_tag: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Delivery Schedule."]
pub struct DeliverySchedule {
    #[serde(rename = "frequencyCap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Limit on the number of times an individual user can be served the ad within a specified period of time."]
    pub frequency_cap: ::std::option::Option<::std::boxed::Box<FrequencyCap>>,
    #[serde(rename = "hardCutoff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not hard cutoff is enabled. If true, the ad will not serve after the end date and time. Otherwise the ad will continue to be served until it has reached its delivery goals."]
    pub hard_cutoff: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "impressionRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Impression ratio for this ad. This ratio determines how often each ad is served relative to the others. For example, if ad A has an impression ratio of 1 and ad B has an impression ratio of 3, then Campaign Manager will serve ad B three times as often as ad A. Acceptable values are 1 to 10, inclusive."]
    pub impression_ratio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serving priority of an ad, with respect to other ads. The lower the priority number, the greater the priority with which it is served."]
    pub priority: ::std::option::Option<DeliverySchedulePriorityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Serving priority of an ad, with respect to other ads. The lower the priority number, the greater the priority with which it is served."]
pub enum DeliverySchedulePriorityEnum {
    #[serde(rename = "AD_PRIORITY_01")]
    #[doc = ""]
    AdPriority01,
    #[serde(rename = "AD_PRIORITY_02")]
    #[doc = ""]
    AdPriority02,
    #[serde(rename = "AD_PRIORITY_03")]
    #[doc = ""]
    AdPriority03,
    #[serde(rename = "AD_PRIORITY_04")]
    #[doc = ""]
    AdPriority04,
    #[serde(rename = "AD_PRIORITY_05")]
    #[doc = ""]
    AdPriority05,
    #[serde(rename = "AD_PRIORITY_06")]
    #[doc = ""]
    AdPriority06,
    #[serde(rename = "AD_PRIORITY_07")]
    #[doc = ""]
    AdPriority07,
    #[serde(rename = "AD_PRIORITY_08")]
    #[doc = ""]
    AdPriority08,
    #[serde(rename = "AD_PRIORITY_09")]
    #[doc = ""]
    AdPriority09,
    #[serde(rename = "AD_PRIORITY_10")]
    #[doc = ""]
    AdPriority10,
    #[serde(rename = "AD_PRIORITY_11")]
    #[doc = ""]
    AdPriority11,
    #[serde(rename = "AD_PRIORITY_12")]
    #[doc = ""]
    AdPriority12,
    #[serde(rename = "AD_PRIORITY_13")]
    #[doc = ""]
    AdPriority13,
    #[serde(rename = "AD_PRIORITY_14")]
    #[doc = ""]
    AdPriority14,
    #[serde(rename = "AD_PRIORITY_15")]
    #[doc = ""]
    AdPriority15,
    #[serde(rename = "AD_PRIORITY_16")]
    #[doc = ""]
    AdPriority16,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Ad Manager Settings"]
pub struct DfpSettings {
    #[serde(rename = "dfpNetworkCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad Manager network code for this directory site."]
    pub dfp_network_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dfpNetworkName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad Manager network name for this directory site."]
    pub dfp_network_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "programmaticPlacementAccepted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this directory site accepts programmatic placements."]
    pub programmatic_placement_accepted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pubPaidPlacementAccepted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this directory site accepts publisher-paid tags."]
    pub pub_paid_placement_accepted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "publisherPortalOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this directory site is available only via Publisher Portal."]
    pub publisher_portal_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a dimension."]
pub struct Dimension {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#dimension."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimension name, e.g. dfa:advertiser"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a dimension filter."]
pub struct DimensionFilter {
    #[serde(rename = "dimensionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the dimension to filter."]
    pub dimension_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#dimensionFilter."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the dimension to filter."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a DimensionValue resource."]
pub struct DimensionValue {
    #[serde(rename = "dimensionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the dimension."]
    pub dimension_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The eTag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID associated with the value if available."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#dimensionValue."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT."]
    pub match_type: ::std::option::Option<DimensionValueMatchTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the dimension."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Determines how the 'value' field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, '*' is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions ('dfa:paidSearch*') allow a matchType other than EXACT."]
pub enum DimensionValueMatchTypeEnum {
    #[serde(rename = "EXACT")]
    #[doc = ""]
    Exact,
    #[serde(rename = "BEGINS_WITH")]
    #[doc = ""]
    BeginsWith,
    #[serde(rename = "CONTAINS")]
    #[doc = ""]
    Contains,
    #[serde(rename = "WILDCARD_EXPRESSION")]
    #[doc = ""]
    WildcardExpression,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the list of DimensionValue resources."]
pub struct DimensionValueList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The eTag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimension values returned in this response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of list this is, in this case dfareporting#dimensionValueList."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through dimension values. To retrieve the next page of results, set the next request's \"pageToken\" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a DimensionValuesRequest."]
pub struct DimensionValueRequest {
    #[serde(rename = "dimensionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the dimension for which values should be requested."]
    pub dimension_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters by which to filter values. The filters are ANDed."]
    pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionFilter>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of request this is, in this case dfareporting#dimensionValueRequest ."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DirectorySites contains properties of a website from the Site Directory. Sites need to be added to an account via the Sites resource before they can be assigned to a placement."]
pub struct DirectorySite {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this directory site is active."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this directory site. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this directory site. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "inpageTagFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag types for regular placements. Acceptable values are: - \"STANDARD\" - \"IFRAME_JAVASCRIPT_INPAGE\" - \"INTERNAL_REDIRECT_INPAGE\" - \"JAVASCRIPT_INPAGE\" "]
    pub inpage_tag_formats:
        ::std::option::Option<::std::vec::Vec<DirectorySiteInpageTagFormatsEnum>>,
    #[serde(rename = "interstitialTagFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag types for interstitial placements. Acceptable values are: - \"IFRAME_JAVASCRIPT_INTERSTITIAL\" - \"INTERNAL_REDIRECT_INTERSTITIAL\" - \"JAVASCRIPT_INTERSTITIAL\" "]
    pub interstitial_tag_formats:
        ::std::option::Option<::std::vec::Vec<DirectorySiteInterstitialTagFormatsEnum>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#directorySite\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this directory site."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site settings."]
    pub settings: ::std::option::Option<::std::boxed::Box<DirectorySiteSettings>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of this directory site."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DirectorySiteInpageTagFormatsEnum {
    #[serde(rename = "STANDARD")]
    #[doc = ""]
    Standard,
    #[serde(rename = "IFRAME_JAVASCRIPT_INPAGE")]
    #[doc = ""]
    IframeJavascriptInpage,
    #[serde(rename = "INTERNAL_REDIRECT_INPAGE")]
    #[doc = ""]
    InternalRedirectInpage,
    #[serde(rename = "JAVASCRIPT_INPAGE")]
    #[doc = ""]
    JavascriptInpage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DirectorySiteInterstitialTagFormatsEnum {
    #[serde(rename = "IFRAME_JAVASCRIPT_INTERSTITIAL")]
    #[doc = ""]
    IframeJavascriptInterstitial,
    #[serde(rename = "INTERNAL_REDIRECT_INTERSTITIAL")]
    #[doc = ""]
    InternalRedirectInterstitial,
    #[serde(rename = "JAVASCRIPT_INTERSTITIAL")]
    #[doc = ""]
    JavascriptInterstitial,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Directory Site Settings"]
pub struct DirectorySiteSettings {
    #[serde(rename = "activeViewOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this directory site has disabled active view creatives."]
    pub active_view_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dfpSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site Ad Manager settings."]
    pub dfp_settings: ::std::option::Option<::std::boxed::Box<DfpSettings>>,
    #[serde(rename = "instreamVideoPlacementAccepted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this site accepts in-stream video ads."]
    pub instream_video_placement_accepted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "interstitialPlacementAccepted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this site accepts interstitial ads."]
    pub interstitial_placement_accepted: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Directory Site List Response"]
pub struct DirectorySitesListResponse {
    #[serde(rename = "directorySites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site collection."]
    pub directory_sites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DirectorySite>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#directorySitesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a dynamic targeting key. Dynamic targeting keys are unique, user-friendly labels, created at the advertiser level in DCM, that can be assigned to ads, creatives, and placements and used for targeting with Studio dynamic creatives. Use these labels instead of numeric Campaign Manager IDs (such as placement IDs) to save time and avoid errors in your dynamic feeds."]
pub struct DynamicTargetingKey {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#dynamicTargetingKey\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this dynamic targeting key. This is a required field. Must be less than 256 characters long and cannot contain commas. All characters are converted to lowercase."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the object of this dynamic targeting key. This is a required field."]
    pub object_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the object of this dynamic targeting key. This is a required field."]
    pub object_type: ::std::option::Option<DynamicTargetingKeyObjectTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the object of this dynamic targeting key. This is a required field."]
pub enum DynamicTargetingKeyObjectTypeEnum {
    #[serde(rename = "OBJECT_ADVERTISER")]
    #[doc = ""]
    ObjectAdvertiser,
    #[serde(rename = "OBJECT_AD")]
    #[doc = ""]
    ObjectAd,
    #[serde(rename = "OBJECT_CREATIVE")]
    #[doc = ""]
    ObjectCreative,
    #[serde(rename = "OBJECT_PLACEMENT")]
    #[doc = ""]
    ObjectPlacement,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dynamic Targeting Key List Response"]
pub struct DynamicTargetingKeysListResponse {
    #[serde(rename = "dynamicTargetingKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dynamic targeting key collection."]
    pub dynamic_targeting_keys:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicTargetingKey>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#dynamicTargetingKeysListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of how user IDs are encrypted."]
pub struct EncryptionInfo {
    #[serde(rename = "encryptionEntityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encryption entity ID. This should match the encryption configuration for ad serving or Data Transfer."]
    pub encryption_entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "encryptionEntityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encryption entity type. This should match the encryption configuration for ad serving or Data Transfer."]
    pub encryption_entity_type: ::std::option::Option<EncryptionInfoEncryptionEntityTypeEnum>,
    #[serde(rename = "encryptionSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes whether the encrypted cookie was received from ad serving (the %m macro) or from Data Transfer."]
    pub encryption_source: ::std::option::Option<EncryptionInfoEncryptionSourceEnum>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#encryptionInfo\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encryption entity type. This should match the encryption configuration for ad serving or Data Transfer."]
pub enum EncryptionInfoEncryptionEntityTypeEnum {
    #[serde(rename = "ENCRYPTION_ENTITY_TYPE_UNKNOWN")]
    #[doc = ""]
    EncryptionEntityTypeUnknown,
    #[serde(rename = "DCM_ACCOUNT")]
    #[doc = ""]
    DcmAccount,
    #[serde(rename = "DCM_ADVERTISER")]
    #[doc = ""]
    DcmAdvertiser,
    #[serde(rename = "DBM_PARTNER")]
    #[doc = ""]
    DbmPartner,
    #[serde(rename = "DBM_ADVERTISER")]
    #[doc = ""]
    DbmAdvertiser,
    #[serde(rename = "ADWORDS_CUSTOMER")]
    #[doc = ""]
    AdwordsCustomer,
    #[serde(rename = "DFP_NETWORK_CODE")]
    #[doc = ""]
    DfpNetworkCode,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Describes whether the encrypted cookie was received from ad serving (the %m macro) or from Data Transfer."]
pub enum EncryptionInfoEncryptionSourceEnum {
    #[serde(rename = "ENCRYPTION_SCOPE_UNKNOWN")]
    #[doc = ""]
    EncryptionScopeUnknown,
    #[serde(rename = "AD_SERVING")]
    #[doc = ""]
    AdServing,
    #[serde(rename = "DATA_TRANSFER")]
    #[doc = ""]
    DataTransfer,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of an event tag."]
pub struct EventTag {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this event tag. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this event tag. This field or the campaignId field is required on insertion."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign ID of this event tag. This field or the advertiserId field is required on insertion."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the campaign. This is a read-only, auto-generated field."]
    pub campaign_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "enabledByDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this event tag should be automatically enabled for all of the advertiser's campaigns and ads."]
    pub enabled_by_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "excludeFromAdxRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to remove this event tag from ads that are trafficked through Display & Video 360 to Ad Exchange. This may be useful if the event tag uses a pixel that is unapproved for Ad Exchange bids on one or more networks, such as the Google Display Network."]
    pub exclude_from_adx_requests: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this event tag. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#eventTag\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this event tag. This is a required field and must be less than 256 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteFilterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site filter type for this event tag. If no type is specified then the event tag will be applied to all sites."]
    pub site_filter_type: ::std::option::Option<EventTagSiteFilterTypeEnum>,
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter list of site IDs associated with this event tag. The siteFilterType determines whether this is a allowlist or blocklist filter."]
    pub site_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this tag is SSL-compliant or not. This is a read-only field."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this event tag. Must be ENABLED for this event tag to fire. This is a required field."]
    pub status: ::std::option::Option<EventTagStatusEnum>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this event tag. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Event tag type. Can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking. This is a required field."]
    pub _type: ::std::option::Option<EventTagTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payload URL for this event tag. The URL on a click-through event tag should have a landing page URL appended to the end of it. This field is required on insertion."]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlEscapeLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of times the landing page URL should be URL-escaped before being appended to the click-through event tag URL. Only applies to click-through event tags as specified by the event tag type."]
    pub url_escape_levels: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Site filter type for this event tag. If no type is specified then the event tag will be applied to all sites."]
pub enum EventTagSiteFilterTypeEnum {
    #[serde(rename = "WHITELIST")]
    #[doc = ""]
    Whitelist,
    #[serde(rename = "BLACKLIST")]
    #[doc = ""]
    Blacklist,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of this event tag. Must be ENABLED for this event tag to fire. This is a required field."]
pub enum EventTagStatusEnum {
    #[serde(rename = "ENABLED")]
    #[doc = ""]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = ""]
    Disabled,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Event tag type. Can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking. This is a required field."]
pub enum EventTagTypeEnum {
    #[serde(rename = "IMPRESSION_IMAGE_EVENT_TAG")]
    #[doc = ""]
    ImpressionImageEventTag,
    #[serde(rename = "IMPRESSION_JAVASCRIPT_EVENT_TAG")]
    #[doc = ""]
    ImpressionJavascriptEventTag,
    #[serde(rename = "CLICK_THROUGH_EVENT_TAG")]
    #[doc = ""]
    ClickThroughEventTag,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Event tag override information."]
pub struct EventTagOverride {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this override is enabled."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this event tag override. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Event Tag List Response"]
pub struct EventTagsListResponse {
    #[serde(rename = "eventTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Event tag collection."]
    pub event_tags: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventTag>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#eventTagsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a File resource. A file contains the metadata for a report run. It shows the status of the run and holds the URLs to the generated report data if the run is finished and the status is \"REPORT_AVAILABLE\"."]
pub struct File {
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range for which the file has report data. The date range will always be the absolute date range for which the report is run."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of this resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filename of the file."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output format of the report. Only available once the file is available."]
    pub format: ::std::option::Option<FileFormatEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of this report file."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#file\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp in milliseconds since epoch when this file was last modified."]
    pub last_modified_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the report this file was generated from."]
    pub report_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the report file."]
    pub status: ::std::option::Option<FileStatusEnum>,
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URLs where the completed report file can be downloaded."]
    pub urls: ::std::option::Option<FileUrls>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The output format of the report. Only available once the file is available."]
pub enum FileFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
    #[serde(rename = "EXCEL")]
    #[doc = ""]
    Excel,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the report file."]
pub enum FileStatusEnum {
    #[serde(rename = "PROCESSING")]
    #[doc = ""]
    Processing,
    #[serde(rename = "REPORT_AVAILABLE")]
    #[doc = ""]
    ReportAvailable,
    #[serde(rename = "FAILED")]
    #[doc = ""]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = ""]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The URLs where the completed report file can be downloaded."]
pub struct FileUrls {
    #[serde(rename = "apiUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for downloading the report data through the API."]
    pub api_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "browserUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for downloading the report data through a browser."]
    pub browser_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of files for a report."]
pub struct FileList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of this resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The files returned in this response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#fileList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through files. To retrieve the next page of results, set the next request's \"pageToken\" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Flight"]
pub struct Flight {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rateOrCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate or cost of this flight."]
    pub rate_or_cost: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Units of this flight."]
    pub units: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Floodlight Activity GenerateTag Response"]
pub struct FloodlightActivitiesGenerateTagResponse {
    #[serde(rename = "floodlightActivityTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Generated tag for this Floodlight activity. For global site tags, this is the event snippet."]
    pub floodlight_activity_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "globalSiteTagGlobalSnippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The global snippet section of a global site tag. The global site tag sets new cookies on your domain, which will store a unique identifier for a user or the ad click that brought the user to your site. Learn more."]
    pub global_site_tag_global_snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightActivitiesGenerateTagResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Floodlight Activity List Response"]
pub struct FloodlightActivitiesListResponse {
    #[serde(rename = "floodlightActivities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity collection."]
    pub floodlight_activities:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FloodlightActivity>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightActivitiesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Floodlight activity."]
pub struct FloodlightActivity {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this floodlight activity. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group's advertiser or the existing activity's advertiser."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "cacheBustingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code type used for cache busting in the generated tag. Applicable only when floodlightActivityGroupType is COUNTER and countingMethod is STANDARD_COUNTING or UNIQUE_COUNTING."]
    pub cache_busting_type: ::std::option::Option<FloodlightActivityCacheBustingTypeEnum>,
    #[serde(rename = "countingMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Counting method for conversions for this floodlight activity. This is a required field."]
    pub counting_method: ::std::option::Option<FloodlightActivityCountingMethodEnum>,
    #[serde(rename = "defaultTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dynamic floodlight tags."]
    pub default_tags:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FloodlightActivityDynamicTag>>>,
    #[serde(rename = "expectedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL where this tag will be deployed. If specified, must be less than 256 characters long."]
    pub expected_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity group ID of this floodlight activity. This is a required field."]
    pub floodlight_activity_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityGroupName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the associated floodlight activity group. This is a read-only field."]
    pub floodlight_activity_group_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityGroupTagString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag string of the associated floodlight activity group. This is a read-only field."]
    pub floodlight_activity_group_tag_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityGroupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the associated floodlight activity group. This is a read-only field."]
    pub floodlight_activity_group_type:
        ::std::option::Option<FloodlightActivityFloodlightActivityGroupTypeEnum>,
    #[serde(rename = "floodlightConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight configuration ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group's floodlight configuration or from the existing activity's floodlight configuration."]
    pub floodlight_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightConfigurationIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field."]
    pub floodlight_configuration_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "floodlightTagType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of Floodlight tag this activity will generate. This is a required field."]
    pub floodlight_tag_type: ::std::option::Option<FloodlightActivityFloodlightTagTypeEnum>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this activity is archived."]
    pub hidden: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this floodlight activity. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this floodlight activity. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightActivity\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this floodlight activity. This is a required field. Must be less than 129 characters long and cannot contain quotes."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General notes or implementation instructions for the tag."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher dynamic floodlight tags."]
    pub publisher_tags: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<FloodlightActivityPublisherDynamicTag>>,
    >,
    #[serde(rename = "secure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this tag should use SSL."]
    pub secure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sslCompliant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the floodlight activity is SSL-compliant. This is a read-only field, its value detected by the system from the floodlight tags."]
    pub ssl_compliant: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sslRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this floodlight activity must be SSL-compliant."]
    pub ssl_required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this floodlight activity. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tagFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag format type for the floodlight activity. If left blank, the tag format will default to HTML."]
    pub tag_format: ::std::option::Option<FloodlightActivityTagFormatEnum>,
    #[serde(rename = "tagString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the cat= parameter in the floodlight tag, which the ad servers use to identify the activity. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being a-z0-9[ _ ]. This tag string must also be unique among activities of the same activity group. This field is read-only after insertion."]
    pub tag_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userDefinedVariableTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the user-defined variables used by this conversion tag. These map to the \"u[1-100]=\" in the tags. Each of these can have a user defined type. Acceptable values are U1 to U100, inclusive. "]
    pub user_defined_variable_types:
        ::std::option::Option<::std::vec::Vec<FloodlightActivityUserDefinedVariableTypesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Code type used for cache busting in the generated tag. Applicable only when floodlightActivityGroupType is COUNTER and countingMethod is STANDARD_COUNTING or UNIQUE_COUNTING."]
pub enum FloodlightActivityCacheBustingTypeEnum {
    #[serde(rename = "JAVASCRIPT")]
    #[doc = ""]
    Javascript,
    #[serde(rename = "ACTIVE_SERVER_PAGE")]
    #[doc = ""]
    ActiveServerPage,
    #[serde(rename = "JSP")]
    #[doc = ""]
    Jsp,
    #[serde(rename = "PHP")]
    #[doc = ""]
    Php,
    #[serde(rename = "COLD_FUSION")]
    #[doc = ""]
    ColdFusion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Counting method for conversions for this floodlight activity. This is a required field."]
pub enum FloodlightActivityCountingMethodEnum {
    #[serde(rename = "STANDARD_COUNTING")]
    #[doc = ""]
    StandardCounting,
    #[serde(rename = "UNIQUE_COUNTING")]
    #[doc = ""]
    UniqueCounting,
    #[serde(rename = "SESSION_COUNTING")]
    #[doc = ""]
    SessionCounting,
    #[serde(rename = "TRANSACTIONS_COUNTING")]
    #[doc = ""]
    TransactionsCounting,
    #[serde(rename = "ITEMS_SOLD_COUNTING")]
    #[doc = ""]
    ItemsSoldCounting,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the associated floodlight activity group. This is a read-only field."]
pub enum FloodlightActivityFloodlightActivityGroupTypeEnum {
    #[serde(rename = "COUNTER")]
    #[doc = ""]
    Counter,
    #[serde(rename = "SALE")]
    #[doc = ""]
    Sale,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of Floodlight tag this activity will generate. This is a required field."]
pub enum FloodlightActivityFloodlightTagTypeEnum {
    #[serde(rename = "IFRAME")]
    #[doc = ""]
    Iframe,
    #[serde(rename = "IMAGE")]
    #[doc = ""]
    Image,
    #[serde(rename = "GLOBAL_SITE_TAG")]
    #[doc = ""]
    GlobalSiteTag,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Tag format type for the floodlight activity. If left blank, the tag format will default to HTML."]
pub enum FloodlightActivityTagFormatEnum {
    #[serde(rename = "HTML")]
    #[doc = ""]
    Html,
    #[serde(rename = "XHTML")]
    #[doc = ""]
    Xhtml,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum FloodlightActivityUserDefinedVariableTypesEnum {
    #[serde(rename = "U1")]
    #[doc = ""]
    U1,
    #[serde(rename = "U2")]
    #[doc = ""]
    U2,
    #[serde(rename = "U3")]
    #[doc = ""]
    U3,
    #[serde(rename = "U4")]
    #[doc = ""]
    U4,
    #[serde(rename = "U5")]
    #[doc = ""]
    U5,
    #[serde(rename = "U6")]
    #[doc = ""]
    U6,
    #[serde(rename = "U7")]
    #[doc = ""]
    U7,
    #[serde(rename = "U8")]
    #[doc = ""]
    U8,
    #[serde(rename = "U9")]
    #[doc = ""]
    U9,
    #[serde(rename = "U10")]
    #[doc = ""]
    U10,
    #[serde(rename = "U11")]
    #[doc = ""]
    U11,
    #[serde(rename = "U12")]
    #[doc = ""]
    U12,
    #[serde(rename = "U13")]
    #[doc = ""]
    U13,
    #[serde(rename = "U14")]
    #[doc = ""]
    U14,
    #[serde(rename = "U15")]
    #[doc = ""]
    U15,
    #[serde(rename = "U16")]
    #[doc = ""]
    U16,
    #[serde(rename = "U17")]
    #[doc = ""]
    U17,
    #[serde(rename = "U18")]
    #[doc = ""]
    U18,
    #[serde(rename = "U19")]
    #[doc = ""]
    U19,
    #[serde(rename = "U20")]
    #[doc = ""]
    U20,
    #[serde(rename = "U21")]
    #[doc = ""]
    U21,
    #[serde(rename = "U22")]
    #[doc = ""]
    U22,
    #[serde(rename = "U23")]
    #[doc = ""]
    U23,
    #[serde(rename = "U24")]
    #[doc = ""]
    U24,
    #[serde(rename = "U25")]
    #[doc = ""]
    U25,
    #[serde(rename = "U26")]
    #[doc = ""]
    U26,
    #[serde(rename = "U27")]
    #[doc = ""]
    U27,
    #[serde(rename = "U28")]
    #[doc = ""]
    U28,
    #[serde(rename = "U29")]
    #[doc = ""]
    U29,
    #[serde(rename = "U30")]
    #[doc = ""]
    U30,
    #[serde(rename = "U31")]
    #[doc = ""]
    U31,
    #[serde(rename = "U32")]
    #[doc = ""]
    U32,
    #[serde(rename = "U33")]
    #[doc = ""]
    U33,
    #[serde(rename = "U34")]
    #[doc = ""]
    U34,
    #[serde(rename = "U35")]
    #[doc = ""]
    U35,
    #[serde(rename = "U36")]
    #[doc = ""]
    U36,
    #[serde(rename = "U37")]
    #[doc = ""]
    U37,
    #[serde(rename = "U38")]
    #[doc = ""]
    U38,
    #[serde(rename = "U39")]
    #[doc = ""]
    U39,
    #[serde(rename = "U40")]
    #[doc = ""]
    U40,
    #[serde(rename = "U41")]
    #[doc = ""]
    U41,
    #[serde(rename = "U42")]
    #[doc = ""]
    U42,
    #[serde(rename = "U43")]
    #[doc = ""]
    U43,
    #[serde(rename = "U44")]
    #[doc = ""]
    U44,
    #[serde(rename = "U45")]
    #[doc = ""]
    U45,
    #[serde(rename = "U46")]
    #[doc = ""]
    U46,
    #[serde(rename = "U47")]
    #[doc = ""]
    U47,
    #[serde(rename = "U48")]
    #[doc = ""]
    U48,
    #[serde(rename = "U49")]
    #[doc = ""]
    U49,
    #[serde(rename = "U50")]
    #[doc = ""]
    U50,
    #[serde(rename = "U51")]
    #[doc = ""]
    U51,
    #[serde(rename = "U52")]
    #[doc = ""]
    U52,
    #[serde(rename = "U53")]
    #[doc = ""]
    U53,
    #[serde(rename = "U54")]
    #[doc = ""]
    U54,
    #[serde(rename = "U55")]
    #[doc = ""]
    U55,
    #[serde(rename = "U56")]
    #[doc = ""]
    U56,
    #[serde(rename = "U57")]
    #[doc = ""]
    U57,
    #[serde(rename = "U58")]
    #[doc = ""]
    U58,
    #[serde(rename = "U59")]
    #[doc = ""]
    U59,
    #[serde(rename = "U60")]
    #[doc = ""]
    U60,
    #[serde(rename = "U61")]
    #[doc = ""]
    U61,
    #[serde(rename = "U62")]
    #[doc = ""]
    U62,
    #[serde(rename = "U63")]
    #[doc = ""]
    U63,
    #[serde(rename = "U64")]
    #[doc = ""]
    U64,
    #[serde(rename = "U65")]
    #[doc = ""]
    U65,
    #[serde(rename = "U66")]
    #[doc = ""]
    U66,
    #[serde(rename = "U67")]
    #[doc = ""]
    U67,
    #[serde(rename = "U68")]
    #[doc = ""]
    U68,
    #[serde(rename = "U69")]
    #[doc = ""]
    U69,
    #[serde(rename = "U70")]
    #[doc = ""]
    U70,
    #[serde(rename = "U71")]
    #[doc = ""]
    U71,
    #[serde(rename = "U72")]
    #[doc = ""]
    U72,
    #[serde(rename = "U73")]
    #[doc = ""]
    U73,
    #[serde(rename = "U74")]
    #[doc = ""]
    U74,
    #[serde(rename = "U75")]
    #[doc = ""]
    U75,
    #[serde(rename = "U76")]
    #[doc = ""]
    U76,
    #[serde(rename = "U77")]
    #[doc = ""]
    U77,
    #[serde(rename = "U78")]
    #[doc = ""]
    U78,
    #[serde(rename = "U79")]
    #[doc = ""]
    U79,
    #[serde(rename = "U80")]
    #[doc = ""]
    U80,
    #[serde(rename = "U81")]
    #[doc = ""]
    U81,
    #[serde(rename = "U82")]
    #[doc = ""]
    U82,
    #[serde(rename = "U83")]
    #[doc = ""]
    U83,
    #[serde(rename = "U84")]
    #[doc = ""]
    U84,
    #[serde(rename = "U85")]
    #[doc = ""]
    U85,
    #[serde(rename = "U86")]
    #[doc = ""]
    U86,
    #[serde(rename = "U87")]
    #[doc = ""]
    U87,
    #[serde(rename = "U88")]
    #[doc = ""]
    U88,
    #[serde(rename = "U89")]
    #[doc = ""]
    U89,
    #[serde(rename = "U90")]
    #[doc = ""]
    U90,
    #[serde(rename = "U91")]
    #[doc = ""]
    U91,
    #[serde(rename = "U92")]
    #[doc = ""]
    U92,
    #[serde(rename = "U93")]
    #[doc = ""]
    U93,
    #[serde(rename = "U94")]
    #[doc = ""]
    U94,
    #[serde(rename = "U95")]
    #[doc = ""]
    U95,
    #[serde(rename = "U96")]
    #[doc = ""]
    U96,
    #[serde(rename = "U97")]
    #[doc = ""]
    U97,
    #[serde(rename = "U98")]
    #[doc = ""]
    U98,
    #[serde(rename = "U99")]
    #[doc = ""]
    U99,
    #[serde(rename = "U100")]
    #[doc = ""]
    U100,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dynamic Tag"]
pub struct FloodlightActivityDynamicTag {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this dynamic tag. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this tag."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag code."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Floodlight activity group."]
pub struct FloodlightActivityGroup {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this floodlight activity group. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this floodlight activity group. If this field is left blank, the value will be copied over either from the floodlight configuration's advertiser or from the existing activity group's advertiser."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "floodlightConfigurationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight configuration ID of this floodlight activity group. This is a required field."]
    pub floodlight_configuration_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightConfigurationIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the floodlight configuration. This is a read-only, auto-generated field."]
    pub floodlight_configuration_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this floodlight activity group. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this floodlight activity group. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightActivityGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this floodlight activity group. This is a required field. Must be less than 65 characters long and cannot contain quotes."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this floodlight activity group. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tagString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the type= parameter in the floodlight tag, which the ad servers use to identify the activity group that the activity belongs to. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being a-z0-9[ _ ]. This tag string must also be unique among activity groups of the same floodlight configuration. This field is read-only after insertion."]
    pub tag_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the floodlight activity group. This is a required field that is read-only after insertion."]
    pub _type: ::std::option::Option<FloodlightActivityGroupTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the floodlight activity group. This is a required field that is read-only after insertion."]
pub enum FloodlightActivityGroupTypeEnum {
    #[serde(rename = "COUNTER")]
    #[doc = ""]
    Counter,
    #[serde(rename = "SALE")]
    #[doc = ""]
    Sale,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Floodlight Activity Group List Response"]
pub struct FloodlightActivityGroupsListResponse {
    #[serde(rename = "floodlightActivityGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity group collection."]
    pub floodlight_activity_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FloodlightActivityGroup>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightActivityGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publisher Dynamic Tag"]
pub struct FloodlightActivityPublisherDynamicTag {
    #[serde(rename = "clickThrough")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this tag is applicable only for click-throughs."]
    pub click_through: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "directorySiteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site ID of this dynamic tag. This is a write-only field that can be used as an alternative to the siteId field. When this resource is retrieved, only the siteId field will be populated."]
    pub directory_site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dynamicTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dynamic floodlight tag."]
    pub dynamic_tag: ::std::option::Option<::std::boxed::Box<FloodlightActivityDynamicTag>>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site ID of this dynamic tag."]
    pub site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the site. This is a read-only, auto-generated field."]
    pub site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "viewThrough")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this tag is applicable only for view-throughs."]
    pub view_through: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Floodlight configuration."]
pub struct FloodlightConfiguration {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this floodlight configuration. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of the parent advertiser of this floodlight configuration."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "analyticsDataSharingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether advertiser data is shared with Google Analytics."]
    pub analytics_data_sharing_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "customViewabilityMetric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom Viewability metric for the floodlight configuration."]
    pub custom_viewability_metric:
        ::std::option::Option<::std::boxed::Box<CustomViewabilityMetric>>,
    #[serde(rename = "exposureToConversionEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the exposure-to-conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting."]
    pub exposure_to_conversion_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "firstDayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day that will be counted as the first day of the week in reports. This is a required field."]
    pub first_day_of_week: ::std::option::Option<FloodlightConfigurationFirstDayOfWeekEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this floodlight configuration. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this floodlight configuration. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "inAppAttributionTrackingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether in-app attribution tracking is enabled."]
    pub in_app_attribution_tracking_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightConfiguration\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lookbackConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window settings for this floodlight configuration."]
    pub lookback_configuration: ::std::option::Option<::std::boxed::Box<LookbackConfiguration>>,
    #[serde(rename = "naturalSearchConversionAttributionOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Types of attribution options for natural search conversions."]
    pub natural_search_conversion_attribution_option:
        ::std::option::Option<FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum>,
    #[serde(rename = "omnitureSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for Campaign Manager Omniture integration."]
    pub omniture_settings: ::std::option::Option<::std::boxed::Box<OmnitureSettings>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this floodlight configuration. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tagSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration settings for dynamic and image floodlight tags."]
    pub tag_settings: ::std::option::Option<::std::boxed::Box<TagSettings>>,
    #[serde(rename = "thirdPartyAuthenticationTokens")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of third-party authentication tokens enabled for this configuration."]
    pub third_party_authentication_tokens:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThirdPartyAuthenticationToken>>>,
    #[serde(rename = "userDefinedVariableConfigurations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of user defined variables enabled for this configuration."]
    pub user_defined_variable_configurations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserDefinedVariableConfiguration>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Day that will be counted as the first day of the week in reports. This is a required field."]
pub enum FloodlightConfigurationFirstDayOfWeekEnum {
    #[serde(rename = "MONDAY")]
    #[doc = ""]
    Monday,
    #[serde(rename = "SUNDAY")]
    #[doc = ""]
    Sunday,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Types of attribution options for natural search conversions."]
pub enum FloodlightConfigurationNaturalSearchConversionAttributionOptionEnum {
    #[serde(rename = "EXCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION")]
    #[doc = ""]
    ExcludeNaturalSearchConversionAttribution,
    #[serde(rename = "INCLUDE_NATURAL_SEARCH_CONVERSION_ATTRIBUTION")]
    #[doc = ""]
    IncludeNaturalSearchConversionAttribution,
    #[serde(rename = "INCLUDE_NATURAL_SEARCH_TIERED_CONVERSION_ATTRIBUTION")]
    #[doc = ""]
    IncludeNaturalSearchTieredConversionAttribution,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Floodlight Configuration List Response"]
pub struct FloodlightConfigurationsListResponse {
    #[serde(rename = "floodlightConfigurations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight configuration collection."]
    pub floodlight_configurations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FloodlightConfiguration>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#floodlightConfigurationsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fields that are compatible to be selected for a report of type \"FlOODLIGHT\"."]
pub struct FloodlightReportCompatibleFields {
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensionFilters\" section of the report."]
    pub dimension_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensions\" section of the report."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#floodlightReportCompatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"metricNames\" section of the report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Frequency Cap."]
pub struct FrequencyCap {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of time, in seconds, for this frequency cap. The maximum duration is 90 days. Acceptable values are 1 to 7776000, inclusive."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "impressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of times an individual user can be served the ad within the specified duration. Acceptable values are 1 to 15, inclusive."]
    pub impressions: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "FsCommand."]
pub struct FsCommand {
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Distance from the left of the browser.Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER."]
    pub left: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "positionOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in the browser where the window will open."]
    pub position_option: ::std::option::Option<FsCommandPositionOptionEnum>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Distance from the top of the browser. Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER."]
    pub top: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "windowHeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height of the window."]
    pub window_height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "windowWidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of the window."]
    pub window_width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Position in the browser where the window will open."]
pub enum FsCommandPositionOptionEnum {
    #[serde(rename = "CENTERED")]
    #[doc = ""]
    Centered,
    #[serde(rename = "DISTANCE_FROM_TOP_LEFT_CORNER")]
    #[doc = ""]
    DistanceFromTopLeftCorner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Geographical Targeting."]
pub struct GeoTargeting {
    #[serde(rename = "cities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cities to be targeted. For each city only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a city, do not target or exclude the country of the city, and do not target the metro or region of the city."]
    pub cities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<City>>>,
    #[serde(rename = "countries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Countries to be targeted or excluded from targeting, depending on the setting of the excludeCountries field. For each country only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting or excluding a country, do not target regions, cities, metros, or postal codes in the same country."]
    pub countries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Country>>>,
    #[serde(rename = "excludeCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not to exclude the countries in the countries field from targeting. If false, the countries field refers to countries which will be targeted by the ad."]
    pub exclude_countries: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metros to be targeted. For each metro only dmaId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a metro, do not target or exclude the country of the metro."]
    pub metros: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metro>>>,
    #[serde(rename = "postalCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal codes to be targeted. For each postal code only id is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a postal code, do not target or exclude the country of the postal code."]
    pub postal_codes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCode>>>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regions to be targeted. For each region only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting a region, do not target or exclude the country of the region."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Region>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a buy from the Planning inventory store."]
pub struct InventoryItem {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this inventory item."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adSlots")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad slots of this inventory item. If this inventory item represents a standalone placement, there will be exactly one ad slot. If this inventory item represents a placement group, there will be more than one ad slot, each representing one child placement in that placement group."]
    pub ad_slots: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdSlot>>>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this inventory item."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentCategoryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content category ID of this inventory item."]
    pub content_category_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "estimatedClickThroughRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated click-through rate of this inventory item."]
    pub estimated_click_through_rate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "estimatedConversionRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated conversion rate of this inventory item."]
    pub estimated_conversion_rate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this inventory item."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inPlan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this inventory item is in plan."]
    pub in_plan: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#inventoryItem\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this inventory item."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this inventory item. For standalone inventory items, this is the same name as that of its only ad slot. For group inventory items, this can differ from the name of any of its ad slots."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negotiationChannelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Negotiation channel ID of this inventory item."]
    pub negotiation_channel_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Order ID of this inventory item."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementStrategyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement strategy ID of this inventory item."]
    pub placement_strategy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pricing of this inventory item."]
    pub pricing: ::std::option::Option<::std::boxed::Box<Pricing>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID of this inventory item."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rfpId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RFP ID of this inventory item."]
    pub rfp_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the site this inventory item is associated with."]
    pub site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this inventory item."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of inventory item."]
    pub _type: ::std::option::Option<InventoryItemTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of inventory item."]
pub enum InventoryItemTypeEnum {
    #[serde(rename = "PLANNING_PLACEMENT_TYPE_REGULAR")]
    #[doc = ""]
    PlanningPlacementTypeRegular,
    #[serde(rename = "PLANNING_PLACEMENT_TYPE_CREDIT")]
    #[doc = ""]
    PlanningPlacementTypeCredit,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inventory item List Response"]
pub struct InventoryItemsListResponse {
    #[serde(rename = "inventoryItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory item collection"]
    pub inventory_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InventoryItem>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#inventoryItemsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Key Value Targeting Expression."]
pub struct KeyValueTargetingExpression {
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Keyword expression being targeted by the ad."]
    pub expression: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about where a user's browser is taken after the user clicks an ad."]
pub struct LandingPage {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this landing page. This is a required field."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this landing page has been archived."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "deepLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links that will direct the user to a mobile app, if installed."]
    pub deep_links: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeepLink>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this landing page. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#landingPage\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this landing page. This is a required field. It must be less than 256 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of this landing page. This is a required field."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a language that can be targeted by ads."]
pub struct Language {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language ID of this language. This is the ID used for targeting and generating reports."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#language\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format of language code is an ISO 639 two-letter language code optionally followed by an underscore followed by an ISO 3166 code. Examples are \"en\" for English or \"zh_CN\" for Simplified Chinese."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this language."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Language Targeting."]
pub struct LanguageTargeting {
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Languages that this ad targets. For each language only languageId is required. The other fields are populated automatically when the ad is inserted or updated."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Language>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Language List Response"]
pub struct LanguagesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#languagesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language collection."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Language>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Modification timestamp."]
pub struct LastModifiedInfo {
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the last change in milliseconds since epoch."]
    pub time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A group clause made up of list population terms representing constraints joined by ORs."]
pub struct ListPopulationClause {
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Terms of this list population clause. Each clause is made up of list population terms representing constraints and are joined by ORs."]
    pub terms: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ListPopulationTerm>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remarketing List Population Rule."]
pub struct ListPopulationRule {
    #[serde(rename = "floodlightActivityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity ID associated with this rule. This field can be left blank."]
    pub floodlight_activity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of floodlight activity associated with this rule. This is a read-only, auto-generated field."]
    pub floodlight_activity_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listPopulationClauses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Clauses that make up this list population rule. Clauses are joined by ANDs, and the clauses themselves are made up of list population terms which are joined by ORs."]
    pub list_population_clauses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ListPopulationClause>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remarketing List Population Rule Term."]
pub struct ListPopulationTerm {
    #[serde(rename = "contains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Will be true if the term should check if the user is in the list and false if the term should check if the user is not in the list. This field is only relevant when type is set to LIST_MEMBERSHIP_TERM. False by default."]
    pub contains: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "negation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to negate the comparison result of this term during rule evaluation. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM."]
    pub negation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comparison operator of this term. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM."]
    pub operator: ::std::option::Option<ListPopulationTermOperatorEnum>,
    #[serde(rename = "remarketingListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the list in question. This field is only relevant when type is set to LIST_MEMBERSHIP_TERM."]
    pub remarketing_list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List population term type determines the applicable fields in this object. If left unset or set to CUSTOM_VARIABLE_TERM, then variableName, variableFriendlyName, operator, value, and negation are applicable. If set to LIST_MEMBERSHIP_TERM then remarketingListId and contains are applicable. If set to REFERRER_TERM then operator, value, and negation are applicable."]
    pub _type: ::std::option::Option<ListPopulationTermTypeEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Literal to compare the variable to. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableFriendlyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Friendly name of this term's variable. This is a read-only, auto-generated field. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM."]
    pub variable_friendly_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the variable (U1, U2, etc.) being compared in this term. This field is only relevant when type is set to null, CUSTOM_VARIABLE_TERM or REFERRER_TERM."]
    pub variable_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Comparison operator of this term. This field is only relevant when type is left unset or set to CUSTOM_VARIABLE_TERM or REFERRER_TERM."]
pub enum ListPopulationTermOperatorEnum {
    #[serde(rename = "NUM_EQUALS")]
    #[doc = ""]
    NumEquals,
    #[serde(rename = "NUM_LESS_THAN")]
    #[doc = ""]
    NumLessThan,
    #[serde(rename = "NUM_LESS_THAN_EQUAL")]
    #[doc = ""]
    NumLessThanEqual,
    #[serde(rename = "NUM_GREATER_THAN")]
    #[doc = ""]
    NumGreaterThan,
    #[serde(rename = "NUM_GREATER_THAN_EQUAL")]
    #[doc = ""]
    NumGreaterThanEqual,
    #[serde(rename = "STRING_EQUALS")]
    #[doc = ""]
    StringEquals,
    #[serde(rename = "STRING_CONTAINS")]
    #[doc = ""]
    StringContains,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "List population term type determines the applicable fields in this object. If left unset or set to CUSTOM_VARIABLE_TERM, then variableName, variableFriendlyName, operator, value, and negation are applicable. If set to LIST_MEMBERSHIP_TERM then remarketingListId and contains are applicable. If set to REFERRER_TERM then operator, value, and negation are applicable."]
pub enum ListPopulationTermTypeEnum {
    #[serde(rename = "CUSTOM_VARIABLE_TERM")]
    #[doc = ""]
    CustomVariableTerm,
    #[serde(rename = "LIST_MEMBERSHIP_TERM")]
    #[doc = ""]
    ListMembershipTerm,
    #[serde(rename = "REFERRER_TERM")]
    #[doc = ""]
    ReferrerTerm,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remarketing List Targeting Expression."]
pub struct ListTargetingExpression {
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expression describing which lists are being targeted by the ad."]
    pub expression: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lookback configuration settings."]
pub struct LookbackConfiguration {
    #[serde(rename = "clickDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive."]
    pub click_duration: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "postImpressionActivitiesDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive."]
    pub post_impression_activities_duration: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a metric."]
pub struct Metric {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#metric."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metric name, e.g. dfa:impressions"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a metro region that can be targeted by ads."]
pub struct Metro {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code of the country to which this metro region belongs."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the country to which this metro region belongs."]
    pub country_dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this metro region."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dmaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DMA ID of this metro region. This is the ID used for targeting and generating reports, and is equivalent to metro_code."]
    pub dma_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#metro\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metroCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metro code of this metro region. This is equivalent to dma_id."]
    pub metro_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this metro region."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metro List Response"]
pub struct MetrosListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#metrosListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metro collection."]
    pub metros: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metro>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a mobile app. Used as a landing page deep link."]
pub struct MobileApp {
    #[serde(rename = "directory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mobile app directory."]
    pub directory: ::std::option::Option<MobileAppDirectoryEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this mobile app."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#mobileApp\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher name."]
    pub publisher_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this mobile app."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mobile app directory."]
pub enum MobileAppDirectoryEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = ""]
    Unknown,
    #[serde(rename = "APPLE_APP_STORE")]
    #[doc = ""]
    AppleAppStore,
    #[serde(rename = "GOOGLE_PLAY_STORE")]
    #[doc = ""]
    GooglePlayStore,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mobile app List Response"]
pub struct MobileAppsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#mobileAppsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mobile apps collection."]
    pub mobile_apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileApp>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a mobile carrier that can be targeted by ads."]
pub struct MobileCarrier {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code of the country to which this mobile carrier belongs."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the country to which this mobile carrier belongs."]
    pub country_dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this mobile carrier."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#mobileCarrier\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this mobile carrier."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mobile Carrier List Response"]
pub struct MobileCarriersListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#mobileCarriersListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileCarriers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mobile carrier collection."]
    pub mobile_carriers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileCarrier>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Object Filter."]
pub struct ObjectFilter {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#objectFilter\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Applicable when status is ASSIGNED. The user has access to objects with these object IDs."]
    pub object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list."]
    pub status: ::std::option::Option<ObjectFilterStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list."]
pub enum ObjectFilterStatusEnum {
    #[serde(rename = "NONE")]
    #[doc = ""]
    None,
    #[serde(rename = "ASSIGNED")]
    #[doc = ""]
    Assigned,
    #[serde(rename = "ALL")]
    #[doc = ""]
    All,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Offset Position."]
pub struct OffsetPosition {
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset distance from left side of an asset or a window."]
    pub left: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset distance from top side of an asset or a window."]
    pub top: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Omniture Integration Settings."]
pub struct OmnitureSettings {
    #[serde(rename = "omnitureCostDataEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether placement cost data will be sent to Omniture. This property can be enabled only if omnitureIntegrationEnabled is true."]
    pub omniture_cost_data_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "omnitureIntegrationEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Omniture integration is enabled. This property can be enabled only when the \"Advanced Ad Serving\" account setting is enabled."]
    pub omniture_integration_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about an operating system that can be targeted by ads."]
pub struct OperatingSystem {
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this operating system. This is the ID used for targeting."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desktop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this operating system is for desktop."]
    pub desktop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#operatingSystem\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this operating system is for mobile."]
    pub mobile: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this operating system."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a particular version of an operating system that can be targeted by ads."]
pub struct OperatingSystemVersion {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this operating system version."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#operatingSystemVersion\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "majorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Major version (leftmost number) of this operating system version."]
    pub major_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minor version (number after the first dot) of this operating system version."]
    pub minor_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this operating system version."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system of this operating system version."]
    pub operating_system: ::std::option::Option<::std::boxed::Box<OperatingSystem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Operating System Version List Response"]
pub struct OperatingSystemVersionsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#operatingSystemVersionsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystemVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system version collection."]
    pub operating_system_versions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperatingSystemVersion>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Operating System List Response"]
pub struct OperatingSystemsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#operatingSystemsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatingSystems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system collection."]
    pub operating_systems:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperatingSystem>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creative optimization activity."]
pub struct OptimizationActivity {
    #[serde(rename = "floodlightActivityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity ID of this optimization activity. This is a required field."]
    pub floodlight_activity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightActivityIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the floodlight activity. This is a read-only, auto-generated field."]
    pub floodlight_activity_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Weight associated with this optimization. The weight assigned will be understood in proportion to the weights assigned to the other optimization activities. Value must be greater than or equal to 1."]
    pub weight: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes properties of a Planning order."]
pub struct Order {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this order."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this order."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "approverUserProfileIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs for users that have to approve documents created for this order."]
    pub approver_user_profile_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "buyerInvoiceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Buyer invoice ID associated with this order."]
    pub buyer_invoice_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "buyerOrganizationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the buyer organization."]
    pub buyer_organization_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments in this order."]
    pub comments: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contacts for this order."]
    pub contacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderContact>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this order. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#order\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this order."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this order."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notes of this order."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "planningTermId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the terms and conditions template used in this order."]
    pub planning_term_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID of this order."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellerOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Seller order ID associated with this order."]
    pub seller_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellerOrganizationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the seller organization."]
    pub seller_organization_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site IDs this order is associated with."]
    pub site_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "siteNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form site names this order is associated with."]
    pub site_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this order."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "termsAndConditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Terms and conditions of this order."]
    pub terms_and_conditions: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contact of an order."]
pub struct OrderContact {
    #[serde(rename = "contactInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form information about this contact. It could be any information related to this contact in addition to type, title, name, and signature user profile ID."]
    pub contact_info: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this contact."]
    pub contact_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this contact."]
    pub contact_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this contact."]
    pub contact_type: ::std::option::Option<OrderContactContactTypeEnum>,
    #[serde(rename = "signatureUserProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user profile containing the signature that will be embedded into order documents."]
    pub signature_user_profile_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of this contact."]
pub enum OrderContactContactTypeEnum {
    #[serde(rename = "PLANNING_ORDER_CONTACT_BUYER_CONTACT")]
    #[doc = ""]
    PlanningOrderContactBuyerContact,
    #[serde(rename = "PLANNING_ORDER_CONTACT_BUYER_BILLING_CONTACT")]
    #[doc = ""]
    PlanningOrderContactBuyerBillingContact,
    #[serde(rename = "PLANNING_ORDER_CONTACT_SELLER_CONTACT")]
    #[doc = ""]
    PlanningOrderContactSellerContact,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Planning order document."]
pub struct OrderDocument {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this order document."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this order document."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "amendedOrderDocumentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amended order document ID of this order document. An order document can be created by optionally amending another order document so that the change history can be preserved."]
    pub amended_order_document_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "approvedByUserProfileIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of users who have approved this order document."]
    pub approved_by_user_profile_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "cancelled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this order document is cancelled."]
    pub cancelled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createdInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the creation of this order document."]
    pub created_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "effectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effective_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this order document."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#orderDocument\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastSentRecipients")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of email addresses that received the last sent document."]
    pub last_sent_recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastSentTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_sent_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the order from which this order document is created."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID of this order document."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this order document has been signed."]
    pub signed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this order document."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this order document."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this order document"]
    pub _type: ::std::option::Option<OrderDocumentTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of this order document"]
pub enum OrderDocumentTypeEnum {
    #[serde(rename = "PLANNING_ORDER_TYPE_INSERTION_ORDER")]
    #[doc = ""]
    PlanningOrderTypeInsertionOrder,
    #[serde(rename = "PLANNING_ORDER_TYPE_CHANGE_ORDER")]
    #[doc = ""]
    PlanningOrderTypeChangeOrder,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Order document List Response"]
pub struct OrderDocumentsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#orderDocumentsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderDocuments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Order document collection"]
    pub order_documents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderDocument>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Order List Response"]
pub struct OrdersListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#ordersListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Order collection."]
    pub orders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Order>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fields that are compatible to be selected for a report of type \"PATH_TO_CONVERSION\"."]
pub struct PathToConversionReportCompatibleFields {
    #[serde(rename = "conversionDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conversion dimensions which are compatible to be selected in the \"conversionDimensions\" section of the report."]
    pub conversion_dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "customFloodlightVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom floodlight variables which are compatible to be selected in the \"customFloodlightVariables\" section of the report."]
    pub custom_floodlight_variables:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#pathToConversionReportCompatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"metricNames\" section of the report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "perInteractionDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Per-interaction dimensions which are compatible to be selected in the \"perInteractionDimensions\" section of the report."]
    pub per_interaction_dimensions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a placement."]
pub struct Placement {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this placement. This field can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adBlockingOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement opts out of ad blocking. When true, ad blocking is disabled for this placement. When false, the campaign and site settings take effect."]
    pub ad_blocking_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "additionalSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional sizes associated with this placement. When inserting or updating a placement, only the size ID field is used."]
    pub additional_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this placement. This field can be left blank."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement is archived."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign ID of this placement. This field is a required field on insertion."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the campaign. This is a read-only, auto-generated field."]
    pub campaign_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments for this placement."]
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "compatibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering on desktop, on mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are no longer allowed for new placement insertions. Instead, use DISPLAY or DISPLAY_INTERSTITIAL. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. This field is required on insertion."]
    pub compatibility: ::std::option::Option<PlacementCompatibilityEnum>,
    #[serde(rename = "contentCategoryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the content category assigned to this placement."]
    pub content_category_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the creation of this placement. This is a read-only field."]
    pub create_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "directorySiteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site ID of this placement. On insert, you must set either this field or the siteId field to specify the site associated with this placement. This is a required field that is read-only after insertion."]
    pub directory_site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directorySiteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the directory site. This is a read-only, auto-generated field."]
    pub directory_site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External ID for this placement."]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this placement. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this placement. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "keyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key name of this placement. This is a read-only, auto-generated field."]
    pub key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placement\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this placement. This is a read-only field."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "lookbackConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lookback window settings for this placement."]
    pub lookback_configuration: ::std::option::Option<::std::boxed::Box<LookbackConfiguration>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this placement.This is a required field and must be less than or equal to 256 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paymentApproved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether payment was approved for this placement. This is a read-only field relevant only to publisher-paid placements."]
    pub payment_approved: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paymentSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payment source for this placement. This is a required field that is read-only after insertion."]
    pub payment_source: ::std::option::Option<PlacementPaymentSourceEnum>,
    #[serde(rename = "placementGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this placement's group, if applicable."]
    pub placement_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementGroupIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the placement group. This is a read-only, auto-generated field."]
    pub placement_group_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "placementStrategyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the placement strategy assigned to this placement."]
    pub placement_strategy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingSchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pricing schedule of this placement. This field is required on insertion, specifically subfields startDate, endDate and pricingType."]
    pub pricing_schedule: ::std::option::Option<::std::boxed::Box<PricingSchedule>>,
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement is the primary placement of a roadblock (placement group). You cannot change this field from true to false. Setting this field to true will automatically set the primary field on the original primary placement of the roadblock to false, and it will automatically set the roadblock's primaryPlacementId field to the ID of this placement."]
    pub primary: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "publisherUpdateInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the last publisher update. This is a read-only field."]
    pub publisher_update_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site ID associated with this placement. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement. This is a required field that is read-only after insertion."]
    pub site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the site. This is a read-only, auto-generated field."]
    pub site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size associated with this placement. When inserting or updating a placement, only the size ID field is used. This field is required on insertion."]
    pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "sslRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether creatives assigned to this placement must be SSL-compliant."]
    pub ssl_required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party placement status."]
    pub status: ::std::option::Option<PlacementStatusEnum>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this placement. This field can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tagFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag formats to generate for this placement. This field is required on insertion. Acceptable values are: - \"PLACEMENT_TAG_STANDARD\" - \"PLACEMENT_TAG_IFRAME_JAVASCRIPT\" - \"PLACEMENT_TAG_IFRAME_ILAYER\" - \"PLACEMENT_TAG_INTERNAL_REDIRECT\" - \"PLACEMENT_TAG_JAVASCRIPT\" - \"PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT\" - \"PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT\" - \"PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT\" - \"PLACEMENT_TAG_CLICK_COMMANDS\" - \"PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH\" - \"PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3\" - \"PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4\" - \"PLACEMENT_TAG_TRACKING\" - \"PLACEMENT_TAG_TRACKING_IFRAME\" - \"PLACEMENT_TAG_TRACKING_JAVASCRIPT\" "]
    pub tag_formats: ::std::option::Option<::std::vec::Vec<PlacementTagFormatsEnum>>,
    #[serde(rename = "tagSetting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag settings for this placement."]
    pub tag_setting: ::std::option::Option<::std::boxed::Box<TagSetting>>,
    #[serde(rename = "videoActiveViewOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Verification and ActiveView are disabled for in-stream video creatives for this placement. The same setting videoActiveViewOptOut exists on the site level -- the opt out occurs if either of these settings are true. These settings are distinct from DirectorySites.settings.activeViewOptOut or Sites.siteSettings.activeViewOptOut which only apply to display ads. However, Accounts.activeViewOptOut opts out both video traffic, as well as display ads, from Verification and ActiveView."]
    pub video_active_view_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "videoSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of settings which affect video creatives served through this placement. Applicable to placements with IN_STREAM_VIDEO compatibility."]
    pub video_settings: ::std::option::Option<::std::boxed::Box<VideoSettings>>,
    #[serde(rename = "vpaidAdapterChoice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "VPAID adapter setting for this placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to this placement. *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH."]
    pub vpaid_adapter_choice: ::std::option::Option<PlacementVpaidAdapterChoiceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Placement compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering on desktop, on mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are no longer allowed for new placement insertions. Instead, use DISPLAY or DISPLAY_INTERSTITIAL. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. This field is required on insertion."]
pub enum PlacementCompatibilityEnum {
    #[serde(rename = "DISPLAY")]
    #[doc = ""]
    Display,
    #[serde(rename = "DISPLAY_INTERSTITIAL")]
    #[doc = ""]
    DisplayInterstitial,
    #[serde(rename = "APP")]
    #[doc = ""]
    App,
    #[serde(rename = "APP_INTERSTITIAL")]
    #[doc = ""]
    AppInterstitial,
    #[serde(rename = "IN_STREAM_VIDEO")]
    #[doc = ""]
    InStreamVideo,
    #[serde(rename = "IN_STREAM_AUDIO")]
    #[doc = ""]
    InStreamAudio,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Payment source for this placement. This is a required field that is read-only after insertion."]
pub enum PlacementPaymentSourceEnum {
    #[serde(rename = "PLACEMENT_AGENCY_PAID")]
    #[doc = ""]
    PlacementAgencyPaid,
    #[serde(rename = "PLACEMENT_PUBLISHER_PAID")]
    #[doc = ""]
    PlacementPublisherPaid,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Third-party placement status."]
pub enum PlacementStatusEnum {
    #[serde(rename = "PENDING_REVIEW")]
    #[doc = ""]
    PendingReview,
    #[serde(rename = "PAYMENT_ACCEPTED")]
    #[doc = ""]
    PaymentAccepted,
    #[serde(rename = "PAYMENT_REJECTED")]
    #[doc = ""]
    PaymentRejected,
    #[serde(rename = "ACKNOWLEDGE_REJECTION")]
    #[doc = ""]
    AcknowledgeRejection,
    #[serde(rename = "ACKNOWLEDGE_ACCEPTANCE")]
    #[doc = ""]
    AcknowledgeAcceptance,
    #[serde(rename = "DRAFT")]
    #[doc = ""]
    Draft,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum PlacementTagFormatsEnum {
    #[serde(rename = "PLACEMENT_TAG_STANDARD")]
    #[doc = ""]
    PlacementTagStandard,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagIframeJavascript,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_ILAYER")]
    #[doc = ""]
    PlacementTagIframeIlayer,
    #[serde(rename = "PLACEMENT_TAG_INTERNAL_REDIRECT")]
    #[doc = ""]
    PlacementTagInternalRedirect,
    #[serde(rename = "PLACEMENT_TAG_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagJavascript,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagInterstitialIframeJavascript,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT")]
    #[doc = ""]
    PlacementTagInterstitialInternalRedirect,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagInterstitialJavascript,
    #[serde(rename = "PLACEMENT_TAG_CLICK_COMMANDS")]
    #[doc = ""]
    PlacementTagClickCommands,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetch,
    #[serde(rename = "PLACEMENT_TAG_TRACKING")]
    #[doc = ""]
    PlacementTagTracking,
    #[serde(rename = "PLACEMENT_TAG_TRACKING_IFRAME")]
    #[doc = ""]
    PlacementTagTrackingIframe,
    #[serde(rename = "PLACEMENT_TAG_TRACKING_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagTrackingJavascript,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetchVast3,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagIframeJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagInterstitialIframeJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagInterstitialJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetchVast4,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "VPAID adapter setting for this placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to this placement. *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH."]
pub enum PlacementVpaidAdapterChoiceEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = ""]
    Default,
    #[serde(rename = "FLASH")]
    #[doc = ""]
    Flash,
    #[serde(rename = "HTML5")]
    #[doc = ""]
    Html5,
    #[serde(rename = "BOTH")]
    #[doc = ""]
    Both,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement Assignment."]
pub struct PlacementAssignment {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement assignment is active. When true, the placement will be included in the ad's rotation."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "placementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the placement to be assigned. This is a required field."]
    pub placement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the placement. This is a read-only, auto-generated field."]
    pub placement_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "sslRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the placement to be assigned requires SSL. This is a read-only field that is auto-generated when the ad is inserted or updated."]
    pub ssl_required: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a package or roadblock."]
pub struct PlacementGroup {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this placement group. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this placement group. This is a required field on insertion."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement group is archived."]
    pub archived: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Campaign ID of this placement group. This field is required on insertion."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the campaign. This is a read-only, auto-generated field."]
    pub campaign_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "childPlacementIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of placements which are assigned to this placement group. This is a read-only, auto-generated field."]
    pub child_placement_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments for this placement group."]
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentCategoryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the content category assigned to this placement group."]
    pub content_category_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the creation of this placement group. This is a read-only field."]
    pub create_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "directorySiteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site ID associated with this placement group. On insert, you must set either this field or the site_id field to specify the site associated with this placement group. This is a required field that is read-only after insertion."]
    pub directory_site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directorySiteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the directory site. This is a read-only, auto-generated field."]
    pub directory_site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External ID for this placement."]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this placement group. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this placement group. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this placement group. This is a read-only field."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this placement group. This is a required field and must be less than 256 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementGroupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this placement group. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting. This field is required on insertion."]
    pub placement_group_type: ::std::option::Option<PlacementGroupPlacementGroupTypeEnum>,
    #[serde(rename = "placementStrategyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the placement strategy assigned to this placement group."]
    pub placement_strategy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingSchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pricing schedule of this placement group. This field is required on insertion."]
    pub pricing_schedule: ::std::option::Option<::std::boxed::Box<PricingSchedule>>,
    #[serde(rename = "primaryPlacementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the primary placement, used to calculate the media cost of a roadblock (placement group). Modifying this field will automatically modify the primary field on all affected roadblock child placements."]
    pub primary_placement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "primaryPlacementIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the primary placement. This is a read-only, auto-generated field."]
    pub primary_placement_id_dimension_value:
        ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site ID associated with this placement group. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement group. This is a required field that is read-only after insertion."]
    pub site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the site. This is a read-only, auto-generated field."]
    pub site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this placement group. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of this placement group. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting. This field is required on insertion."]
pub enum PlacementGroupPlacementGroupTypeEnum {
    #[serde(rename = "PLACEMENT_PACKAGE")]
    #[doc = ""]
    PlacementPackage,
    #[serde(rename = "PLACEMENT_ROADBLOCK")]
    #[doc = ""]
    PlacementRoadblock,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement Group List Response"]
pub struct PlacementGroupsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement group collection."]
    pub placement_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlacementGroup>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement Strategy List Response"]
pub struct PlacementStrategiesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementStrategiesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementStrategies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement strategy collection."]
    pub placement_strategies:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlacementStrategy>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a placement strategy."]
pub struct PlacementStrategy {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this placement strategy.This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this placement strategy. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementStrategy\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this placement strategy. This is a required field. It must be less than 256 characters long and unique among placement strategies of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement Tag"]
pub struct PlacementTag {
    #[serde(rename = "placementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement ID"]
    pub placement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tagDatas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags generated for this placement."]
    pub tag_datas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TagData>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement GenerateTags Response"]
pub struct PlacementsGenerateTagsResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementsGenerateTagsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placementTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of generated tags for the specified placements."]
    pub placement_tags: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlacementTag>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement List Response"]
pub struct PlacementsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#placementsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "placements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement collection."]
    pub placements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Placement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a platform type that can be targeted by ads."]
pub struct PlatformType {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this platform type."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#platformType\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this platform type."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Platform Type List Response"]
pub struct PlatformTypesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#platformTypesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platformTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform type collection."]
    pub platform_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlatformType>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Popup Window Properties."]
pub struct PopupWindowProperties {
    #[serde(rename = "dimension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Popup dimension for a creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA and all VPAID"]
    pub dimension: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper-left corner coordinates of the popup window. Applicable if positionType is COORDINATES."]
    pub offset: ::std::option::Option<::std::boxed::Box<OffsetPosition>>,
    #[serde(rename = "positionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Popup window position either centered or at specific coordinate."]
    pub position_type: ::std::option::Option<PopupWindowPropertiesPositionTypeEnum>,
    #[serde(rename = "showAddressBar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to display the browser address bar."]
    pub show_address_bar: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "showMenuBar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to display the browser menu bar."]
    pub show_menu_bar: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "showScrollBar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to display the browser scroll bar."]
    pub show_scroll_bar: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "showStatusBar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to display the browser status bar."]
    pub show_status_bar: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "showToolBar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to display the browser tool bar."]
    pub show_tool_bar: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of popup window."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Popup window position either centered or at specific coordinate."]
pub enum PopupWindowPropertiesPositionTypeEnum {
    #[serde(rename = "CENTER")]
    #[doc = ""]
    Center,
    #[serde(rename = "COORDINATES")]
    #[doc = ""]
    Coordinates,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a postal code that can be targeted by ads."]
pub struct PostalCode {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal code. This is equivalent to the id field."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code of the country to which this postal code belongs."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the country to which this postal code belongs."]
    pub country_dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this postal code."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#postalCode\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Postal Code List Response"]
pub struct PostalCodesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#postalCodesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal code collection."]
    pub postal_codes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCode>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pricing Information"]
pub struct Pricing {
    #[serde(rename = "capCostType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cap cost type of this inventory item."]
    pub cap_cost_type: ::std::option::Option<PricingCapCostTypeEnum>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flights of this inventory item. A flight (a.k.a. pricing period) represents the inventory item pricing information for a specific period of time."]
    pub flights: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Flight>>>,
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Group type of this inventory item if it represents a placement group. Is null otherwise. There are two type of placement groups: PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE is a simple group of inventory items that acts as a single pricing point for a group of tags. PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK is a group of inventory items that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned inventory items to be marked as primary."]
    pub group_type: ::std::option::Option<PricingGroupTypeEnum>,
    #[serde(rename = "pricingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pricing type of this inventory item."]
    pub pricing_type: ::std::option::Option<PricingPricingTypeEnum>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Cap cost type of this inventory item."]
pub enum PricingCapCostTypeEnum {
    #[serde(rename = "PLANNING_PLACEMENT_CAP_COST_TYPE_NONE")]
    #[doc = ""]
    PlanningPlacementCapCostTypeNone,
    #[serde(rename = "PLANNING_PLACEMENT_CAP_COST_TYPE_MONTHLY")]
    #[doc = ""]
    PlanningPlacementCapCostTypeMonthly,
    #[serde(rename = "PLANNING_PLACEMENT_CAP_COST_TYPE_CUMULATIVE")]
    #[doc = ""]
    PlanningPlacementCapCostTypeCumulative,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Group type of this inventory item if it represents a placement group. Is null otherwise. There are two type of placement groups: PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE is a simple group of inventory items that acts as a single pricing point for a group of tags. PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK is a group of inventory items that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned inventory items to be marked as primary."]
pub enum PricingGroupTypeEnum {
    #[serde(rename = "PLANNING_PLACEMENT_GROUP_TYPE_PACKAGE")]
    #[doc = ""]
    PlanningPlacementGroupTypePackage,
    #[serde(rename = "PLANNING_PLACEMENT_GROUP_TYPE_ROADBLOCK")]
    #[doc = ""]
    PlanningPlacementGroupTypeRoadblock,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Pricing type of this inventory item."]
pub enum PricingPricingTypeEnum {
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_IMPRESSIONS")]
    #[doc = ""]
    PlanningPlacementPricingTypeImpressions,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_CPM")]
    #[doc = ""]
    PlanningPlacementPricingTypeCpm,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_CLICKS")]
    #[doc = ""]
    PlanningPlacementPricingTypeClicks,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_CPC")]
    #[doc = ""]
    PlanningPlacementPricingTypeCpc,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_CPA")]
    #[doc = ""]
    PlanningPlacementPricingTypeCpa,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    #[doc = ""]
    PlanningPlacementPricingTypeFlatRateImpressions,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_FLAT_RATE_CLICKS")]
    #[doc = ""]
    PlanningPlacementPricingTypeFlatRateClicks,
    #[serde(rename = "PLANNING_PLACEMENT_PRICING_TYPE_CPM_ACTIVEVIEW")]
    #[doc = ""]
    PlanningPlacementPricingTypeCpmActiveview,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pricing Schedule"]
pub struct PricingSchedule {
    #[serde(rename = "capCostOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement cap cost option."]
    pub cap_cost_option: ::std::option::Option<PricingScheduleCapCostOptionEnum>,
    #[serde(rename = "disregardOverdelivery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether cap costs are ignored by ad serving."]
    pub disregard_overdelivery: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flighted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this placement is flighted. If true, pricing periods will be computed automatically."]
    pub flighted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "floodlightActivityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Floodlight activity ID associated with this placement. This field should be set when placement pricing type is set to PRICING_TYPE_CPA."]
    pub floodlight_activity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingPeriods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pricing periods for this placement."]
    pub pricing_periods:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricingSchedulePricingPeriod>>>,
    #[serde(rename = "pricingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Placement pricing type. This field is required on insertion."]
    pub pricing_type: ::std::option::Option<PricingSchedulePricingTypeEnum>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testingStartDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub testing_start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Placement cap cost option."]
pub enum PricingScheduleCapCostOptionEnum {
    #[serde(rename = "CAP_COST_NONE")]
    #[doc = ""]
    CapCostNone,
    #[serde(rename = "CAP_COST_MONTHLY")]
    #[doc = ""]
    CapCostMonthly,
    #[serde(rename = "CAP_COST_CUMULATIVE")]
    #[doc = ""]
    CapCostCumulative,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Placement pricing type. This field is required on insertion."]
pub enum PricingSchedulePricingTypeEnum {
    #[serde(rename = "PRICING_TYPE_CPM")]
    #[doc = ""]
    PricingTypeCpm,
    #[serde(rename = "PRICING_TYPE_CPC")]
    #[doc = ""]
    PricingTypeCpc,
    #[serde(rename = "PRICING_TYPE_CPA")]
    #[doc = ""]
    PricingTypeCpa,
    #[serde(rename = "PRICING_TYPE_FLAT_RATE_IMPRESSIONS")]
    #[doc = ""]
    PricingTypeFlatRateImpressions,
    #[serde(rename = "PRICING_TYPE_FLAT_RATE_CLICKS")]
    #[doc = ""]
    PricingTypeFlatRateClicks,
    #[serde(rename = "PRICING_TYPE_CPM_ACTIVEVIEW")]
    #[doc = ""]
    PricingTypeCpmActiveview,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pricing Period"]
pub struct PricingSchedulePricingPeriod {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingComment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comments for this pricing period."]
    pub pricing_comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rateOrCostNanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate or cost of this pricing period in nanos (i.e., multipled by 1000000000). Acceptable values are 0 to 1000000000000000000, inclusive."]
    pub rate_or_cost_nanos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Units of this pricing period. Acceptable values are 0 to 10000000000, inclusive."]
    pub units: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Planning project."]
pub struct Project {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this project."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this project."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "audienceAgeGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience age group of this project."]
    pub audience_age_group: ::std::option::Option<ProjectAudienceAgeGroupEnum>,
    #[serde(rename = "audienceGender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience gender of this project."]
    pub audience_gender: ::std::option::Option<ProjectAudienceGenderEnum>,
    #[serde(rename = "budget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Budget of this project in the currency specified by the current account. The value stored in this field represents only the non-fractional amount. For example, for USD, the smallest value that can be represented by this field is 1 US dollar."]
    pub budget: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientBillingCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client billing code of this project."]
    pub client_billing_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the project client."]
    pub client_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this project. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#project\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the most recent modification of this project."]
    pub last_modified_info: ::std::option::Option<::std::boxed::Box<LastModifiedInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this project."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overview of this project."]
    pub overview: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this project."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetClicks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of clicks that the advertiser is targeting."]
    pub target_clicks: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of conversions that the advertiser is targeting."]
    pub target_conversions: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCpaNanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CPA that the advertiser is targeting."]
    pub target_cpa_nanos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCpcNanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CPC that the advertiser is targeting."]
    pub target_cpc_nanos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCpmActiveViewNanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "vCPM from Active View that the advertiser is targeting."]
    pub target_cpm_active_view_nanos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCpmNanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CPM that the advertiser is targeting."]
    pub target_cpm_nanos: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of impressions that the advertiser is targeting."]
    pub target_impressions: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Audience age group of this project."]
pub enum ProjectAudienceAgeGroupEnum {
    #[serde(rename = "PLANNING_AUDIENCE_AGE_18_24")]
    #[doc = ""]
    PlanningAudienceAge1824,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_25_34")]
    #[doc = ""]
    PlanningAudienceAge2534,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_35_44")]
    #[doc = ""]
    PlanningAudienceAge3544,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_45_54")]
    #[doc = ""]
    PlanningAudienceAge4554,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_55_64")]
    #[doc = ""]
    PlanningAudienceAge5564,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_65_OR_MORE")]
    #[doc = ""]
    PlanningAudienceAge65OrMore,
    #[serde(rename = "PLANNING_AUDIENCE_AGE_UNKNOWN")]
    #[doc = ""]
    PlanningAudienceAgeUnknown,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Audience gender of this project."]
pub enum ProjectAudienceGenderEnum {
    #[serde(rename = "PLANNING_AUDIENCE_GENDER_MALE")]
    #[doc = ""]
    PlanningAudienceGenderMale,
    #[serde(rename = "PLANNING_AUDIENCE_GENDER_FEMALE")]
    #[doc = ""]
    PlanningAudienceGenderFemale,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Project List Response"]
pub struct ProjectsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#projectsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project collection."]
    pub projects: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Project>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fields that are compatible to be selected for a report of type \"REACH\"."]
pub struct ReachReportCompatibleFields {
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensionFilters\" section of the report."]
    pub dimension_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensions\" section of the report."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#reachReportCompatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"metricNames\" section of the report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "pivotedActivityMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected as activity metrics to pivot on in the \"activities\" section of the report."]
    pub pivoted_activity_metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "reachByFrequencyMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"reachByFrequencyMetricNames\" section of the report."]
    pub reach_by_frequency_metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a recipient."]
pub struct Recipient {
    #[serde(rename = "deliveryType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delivery type for the recipient."]
    pub delivery_type: ::std::option::Option<RecipientDeliveryTypeEnum>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the recipient."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#recipient."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The delivery type for the recipient."]
pub enum RecipientDeliveryTypeEnum {
    #[serde(rename = "LINK")]
    #[doc = ""]
    Link,
    #[serde(rename = "ATTACHMENT")]
    #[doc = ""]
    Attachment,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about a region that can be targeted by ads."]
pub struct Region {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code of the country to which this region belongs."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryDartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of the country to which this region belongs."]
    pub country_dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dartId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DART ID of this region."]
    pub dart_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#region\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this region."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Region code."]
    pub region_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Region List Response"]
pub struct RegionsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#regionsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Region collection."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Region>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a remarketing list. Remarketing enables you to create lists of users who have performed specific actions on a site, then target ads to members of those lists. This resource can be used to manage remarketing lists that are owned by your advertisers. To see all remarketing lists that are visible to your advertisers, including those that are shared to your advertiser or account, use the TargetableRemarketingLists resource."]
pub struct RemarketingList {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this remarketing list is active."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the advertiser ID that owns this remarketing list. This is a required field."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list ID. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#remarketingList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lifeSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of days that a user should remain in the remarketing list without an impression. Acceptable values are 1 to 540, inclusive."]
    pub life_span: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listPopulationRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rule used to populate the remarketing list with users."]
    pub list_population_rule: ::std::option::Option<::std::boxed::Box<ListPopulationRule>>,
    #[serde(rename = "listSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of users currently in the list. This is a read-only field."]
    pub list_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product from which this remarketing list was originated."]
    pub list_source: ::std::option::Option<RemarketingListListSourceEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the remarketing list. This is a required field. Must be no greater than 128 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Product from which this remarketing list was originated."]
pub enum RemarketingListListSourceEnum {
    #[serde(rename = "REMARKETING_LIST_SOURCE_OTHER")]
    #[doc = ""]
    RemarketingListSourceOther,
    #[serde(rename = "REMARKETING_LIST_SOURCE_ADX")]
    #[doc = ""]
    RemarketingListSourceAdx,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DFP")]
    #[doc = ""]
    RemarketingListSourceDfp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_XFP")]
    #[doc = ""]
    RemarketingListSourceXfp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DFA")]
    #[doc = ""]
    RemarketingListSourceDfa,
    #[serde(rename = "REMARKETING_LIST_SOURCE_GA")]
    #[doc = ""]
    RemarketingListSourceGa,
    #[serde(rename = "REMARKETING_LIST_SOURCE_YOUTUBE")]
    #[doc = ""]
    RemarketingListSourceYoutube,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DBM")]
    #[doc = ""]
    RemarketingListSourceDbm,
    #[serde(rename = "REMARKETING_LIST_SOURCE_GPLUS")]
    #[doc = ""]
    RemarketingListSourceGplus,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DMP")]
    #[doc = ""]
    RemarketingListSourceDmp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_PLAY_STORE")]
    #[doc = ""]
    RemarketingListSourcePlayStore,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a remarketing list's sharing information. Sharing allows other accounts or advertisers to target to your remarketing lists. This resource can be used to manage remarketing list sharing to other accounts and advertisers."]
pub struct RemarketingListShare {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#remarketingListShare\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remarketingListId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list ID. This is a read-only, auto-generated field."]
    pub remarketing_list_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharedAccountIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Accounts that the remarketing list is shared with."]
    pub shared_account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sharedAdvertiserIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertisers that the remarketing list is shared with."]
    pub shared_advertiser_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remarketing list response"]
pub struct RemarketingListsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#remarketingListsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remarketingLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list collection."]
    pub remarketing_lists:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RemarketingList>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Report resource."]
pub struct Report {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID to which this report belongs."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "criteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report criteria for a report of type \"STANDARD\"."]
    pub criteria: ::std::option::Option<ReportCriteria>,
    #[serde(rename = "crossDimensionReachCriteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report criteria for a report of type \"CROSS_DIMENSION_REACH\"."]
    pub cross_dimension_reach_criteria: ::std::option::Option<ReportCrossDimensionReachCriteria>,
    #[serde(rename = "delivery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report's email delivery settings."]
    pub delivery: ::std::option::Option<ReportDelivery>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The eTag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filename used when generating report files for this report."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightCriteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report criteria for a report of type \"FLOODLIGHT\"."]
    pub floodlight_criteria: ::std::option::Option<ReportFloodlightCriteria>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output format of the report. If not specified, default format is \"CSV\". Note that the actual format in the completed report file might differ if for instance the report's size exceeds the format's capabilities. \"CSV\" will then be the fallback format."]
    pub format: ::std::option::Option<ReportFormatEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID identifying this report resource."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#report."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp (in milliseconds since epoch) of when this report was last modified."]
    pub last_modified_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the report."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user profile id of the owner of this report."]
    pub owner_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pathToConversionCriteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report criteria for a report of type \"PATH_TO_CONVERSION\"."]
    pub path_to_conversion_criteria: ::std::option::Option<ReportPathToConversionCriteria>,
    #[serde(rename = "reachCriteria")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report criteria for a report of type \"REACH\"."]
    pub reach_criteria: ::std::option::Option<ReportReachCriteria>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The report's schedule. Can only be set if the report's 'dateRange' is a relative date range and the relative date range is not \"TODAY\"."]
    pub schedule: ::std::option::Option<ReportSchedule>,
    #[serde(rename = "subAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subaccount ID to which this report belongs if applicable."]
    pub sub_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the report."]
    pub _type: ::std::option::Option<ReportTypeEnum>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report criteria for a report of type \"STANDARD\"."]
pub struct ReportCriteria {
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activity group."]
    pub activities: ::std::option::Option<::std::boxed::Box<Activities>>,
    #[serde(rename = "customRichMediaEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom Rich Media Events group."]
    pub custom_rich_media_events: ::std::option::Option<::std::boxed::Box<CustomRichMediaEvents>>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range for which this report should be run."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters on which dimensions are filtered. Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed."]
    pub dimension_filters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of standard dimensions the report should include."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of metrics the report should include."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report criteria for a report of type \"CROSS_DIMENSION_REACH\"."]
pub struct ReportCrossDimensionReachCriteria {
    #[serde(rename = "breakdown")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of dimensions the report should include."]
    pub breakdown: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range this report should be run for."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "dimension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimension option."]
    pub dimension: ::std::option::Option<ReportCrossDimensionReachCriteriaDimensionEnum>,
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters on which dimensions are filtered."]
    pub dimension_filters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of metrics the report should include."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "overlapMetricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of overlap metrics the report should include."]
    pub overlap_metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "pivoted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the report is pivoted or not. Defaults to true."]
    pub pivoted: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The dimension option."]
pub enum ReportCrossDimensionReachCriteriaDimensionEnum {
    #[serde(rename = "ADVERTISER")]
    #[doc = ""]
    Advertiser,
    #[serde(rename = "CAMPAIGN")]
    #[doc = ""]
    Campaign,
    #[serde(rename = "SITE_BY_ADVERTISER")]
    #[doc = ""]
    SiteByAdvertiser,
    #[serde(rename = "SITE_BY_CAMPAIGN")]
    #[doc = ""]
    SiteByCampaign,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report's email delivery settings."]
pub struct ReportDelivery {
    #[serde(rename = "emailOwner")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the report should be emailed to the report owner."]
    pub email_owner: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "emailOwnerDeliveryType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delivery for the owner to receive, if enabled."]
    pub email_owner_delivery_type: ::std::option::Option<ReportDeliveryEmailOwnerDeliveryTypeEnum>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message to be sent with each email."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recipients")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of recipients to which to email the report."]
    pub recipients: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Recipient>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of delivery for the owner to receive, if enabled."]
pub enum ReportDeliveryEmailOwnerDeliveryTypeEnum {
    #[serde(rename = "LINK")]
    #[doc = ""]
    Link,
    #[serde(rename = "ATTACHMENT")]
    #[doc = ""]
    Attachment,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report criteria for a report of type \"FLOODLIGHT\"."]
pub struct ReportFloodlightCriteria {
    #[serde(rename = "customRichMediaEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom rich media events to include."]
    pub custom_rich_media_events:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range this report should be run for."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters on which dimensions are filtered. Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed."]
    pub dimension_filters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of dimensions the report should include."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "floodlightConfigId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The floodlight ID for which to show data in this report. All advertisers associated with that ID will automatically be added. The dimension of the value needs to be 'dfa:floodlightConfigId'."]
    pub floodlight_config_id: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of metrics the report should include."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "reportProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of the report."]
    pub report_properties: ::std::option::Option<ReportFloodlightCriteriaReportProperties>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of the report."]
pub struct ReportFloodlightCriteriaReportProperties {
    #[serde(rename = "includeAttributedIPConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include conversions that have no cookie, but do have an exposure path."]
    pub include_attributed_ip_conversions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeUnattributedCookieConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window."]
    pub include_unattributed_cookie_conversions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeUnattributedIPConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion."]
    pub include_unattributed_ip_conversions: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The output format of the report. If not specified, default format is \"CSV\". Note that the actual format in the completed report file might differ if for instance the report's size exceeds the format's capabilities. \"CSV\" will then be the fallback format."]
pub enum ReportFormatEnum {
    #[serde(rename = "CSV")]
    #[doc = ""]
    Csv,
    #[serde(rename = "EXCEL")]
    #[doc = ""]
    Excel,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report criteria for a report of type \"PATH_TO_CONVERSION\"."]
pub struct ReportPathToConversionCriteria {
    #[serde(rename = "activityFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of 'dfa:activity' values to filter on."]
    pub activity_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "conversionDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of conversion dimensions the report should include."]
    pub conversion_dimensions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "customFloodlightVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom floodlight variables the report should include."]
    pub custom_floodlight_variables:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "customRichMediaEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom rich media events to include."]
    pub custom_rich_media_events:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range this report should be run for."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "floodlightConfigId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The floodlight ID for which to show data in this report. All advertisers associated with that ID will automatically be added. The dimension of the value needs to be 'dfa:floodlightConfigId'."]
    pub floodlight_config_id: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of metrics the report should include."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "perInteractionDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of per interaction dimensions the report should include."]
    pub per_interaction_dimensions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "reportProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of the report."]
    pub report_properties: ::std::option::Option<ReportPathToConversionCriteriaReportProperties>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties of the report."]
pub struct ReportPathToConversionCriteriaReportProperties {
    #[serde(rename = "clicksLookbackWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CM360 checks to see if a click interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90."]
    pub clicks_lookback_window: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "impressionsLookbackWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CM360 checks to see if an impression interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90."]
    pub impressions_lookback_window: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "includeAttributedIPConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated: has no effect."]
    pub include_attributed_ip_conversions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeUnattributedCookieConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window."]
    pub include_unattributed_cookie_conversions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeUnattributedIPConversions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion."]
    pub include_unattributed_ip_conversions: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maximumClickInteractions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report."]
    pub maximum_click_interactions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maximumImpressionInteractions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report."]
    pub maximum_impression_interactions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maximumInteractionGap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum amount of time that can take place between interactions (clicks or impressions) by the same user. Valid values: 1-90."]
    pub maximum_interaction_gap: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pivotOnInteractionPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable pivoting on interaction path."]
    pub pivot_on_interaction_path: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report criteria for a report of type \"REACH\"."]
pub struct ReportReachCriteria {
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activity group."]
    pub activities: ::std::option::Option<::std::boxed::Box<Activities>>,
    #[serde(rename = "customRichMediaEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom Rich Media Events group."]
    pub custom_rich_media_events: ::std::option::Option<::std::boxed::Box<CustomRichMediaEvents>>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date range this report should be run for."]
    pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters on which dimensions are filtered. Filters for different dimensions are ANDed, filters for the same dimension are grouped together and ORed."]
    pub dimension_filters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of dimensions the report should include."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SortedDimension>>>,
    #[serde(rename = "enableAllDimensionCombinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable all reach dimension combinations in the report. Defaults to false. If enabled, the date range of the report should be within the last 42 days."]
    pub enable_all_dimension_combinations: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of metrics the report should include."]
    pub metric_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "reachByFrequencyMetricNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of names of Reach By Frequency metrics the report should include."]
    pub reach_by_frequency_metric_names:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The report's schedule. Can only be set if the report's 'dateRange' is a relative date range and the relative date range is not \"TODAY\"."]
pub struct ReportSchedule {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the schedule is active or not. Must be set to either true or false."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "every")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines every how many days, weeks or months the report should be run. Needs to be set when \"repeats\" is either \"DAILY\", \"WEEKLY\" or \"MONTHLY\"."]
    pub every: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expiration_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repeats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The interval for which the report is repeated. Note: - \"DAILY\" also requires field \"every\" to be set. - \"WEEKLY\" also requires fields \"every\" and \"repeatsOnWeekDays\" to be set. - \"MONTHLY\" also requires fields \"every\" and \"runsOnDayOfMonth\" to be set. "]
    pub repeats: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repeatsOnWeekDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of week days \"WEEKLY\" on which scheduled reports should run."]
    pub repeats_on_week_days:
        ::std::option::Option<::std::vec::Vec<ReportScheduleRepeatsOnWeekDaysEnum>>,
    #[serde(rename = "runsOnDayOfMonth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enum to define for \"MONTHLY\" scheduled reports whether reports should be repeated on the same day of the month as \"startDate\" or the same day of the week of the month. Example: If 'startDate' is Monday, April 2nd 2012 (2012-04-02), \"DAY_OF_MONTH\" would run subsequent reports on the 2nd of every Month, and \"WEEK_OF_MONTH\" would run subsequent reports on the first Monday of the month."]
    pub runs_on_day_of_month: ::std::option::Option<ReportScheduleRunsOnDayOfMonthEnum>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ReportScheduleRepeatsOnWeekDaysEnum {
    #[serde(rename = "SUNDAY")]
    #[doc = ""]
    Sunday,
    #[serde(rename = "MONDAY")]
    #[doc = ""]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = ""]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = ""]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = ""]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = ""]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = ""]
    Saturday,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Enum to define for \"MONTHLY\" scheduled reports whether reports should be repeated on the same day of the month as \"startDate\" or the same day of the week of the month. Example: If 'startDate' is Monday, April 2nd 2012 (2012-04-02), \"DAY_OF_MONTH\" would run subsequent reports on the 2nd of every Month, and \"WEEK_OF_MONTH\" would run subsequent reports on the first Monday of the month."]
pub enum ReportScheduleRunsOnDayOfMonthEnum {
    #[serde(rename = "DAY_OF_MONTH")]
    #[doc = ""]
    DayOfMonth,
    #[serde(rename = "WEEK_OF_MONTH")]
    #[doc = ""]
    WeekOfMonth,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the report."]
pub enum ReportTypeEnum {
    #[serde(rename = "STANDARD")]
    #[doc = ""]
    Standard,
    #[serde(rename = "REACH")]
    #[doc = ""]
    Reach,
    #[serde(rename = "PATH_TO_CONVERSION")]
    #[doc = ""]
    PathToConversion,
    #[serde(rename = "CROSS_DIMENSION_REACH")]
    #[doc = ""]
    CrossDimensionReach,
    #[serde(rename = "FLOODLIGHT")]
    #[doc = ""]
    Floodlight,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents fields that are compatible to be selected for a report of type \"STANDARD\"."]
pub struct ReportCompatibleFields {
    #[serde(rename = "dimensionFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensionFilters\" section of the report."]
    pub dimension_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions which are compatible to be selected in the \"dimensions\" section of the report."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#reportCompatibleFields."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected in the \"metricNames\" section of the report."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "pivotedActivityMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics which are compatible to be selected as activity metrics to pivot on in the \"activities\" section of the report."]
    pub pivoted_activity_metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the list of reports."]
pub struct ReportList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The eTag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reports returned in this response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Report>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of list this is, in this case dfareporting#reportList."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through reports. To retrieve the next page of results, set the next request's \"pageToken\" to the value of this field. The page token is only valid for a limited amount of time and should not be persisted."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reporting Configuration"]
pub struct ReportsConfiguration {
    #[serde(rename = "exposureToConversionEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the exposure to conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting."]
    pub exposure_to_conversion_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lookbackConfiguration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default lookback windows for new advertisers in this account."]
    pub lookback_configuration: ::std::option::Option<::std::boxed::Box<LookbackConfiguration>>,
    #[serde(rename = "reportGenerationTimeZoneId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Report generation time zone ID of this account. This is a required field that can only be changed by a superuser. Acceptable values are: - \"1\" for \"America/New_York\" - \"2\" for \"Europe/London\" - \"3\" for \"Europe/Paris\" - \"4\" for \"Africa/Johannesburg\" - \"5\" for \"Asia/Jerusalem\" - \"6\" for \"Asia/Shanghai\" - \"7\" for \"Asia/Hong_Kong\" - \"8\" for \"Asia/Tokyo\" - \"9\" for \"Australia/Sydney\" - \"10\" for \"Asia/Dubai\" - \"11\" for \"America/Los_Angeles\" - \"12\" for \"Pacific/Auckland\" - \"13\" for \"America/Sao_Paulo\" - \"16\" for \"America/Asuncion\" - \"17\" for \"America/Chicago\" - \"18\" for \"America/Denver\" - \"19\" for \"America/St_Johns\" - \"20\" for \"Asia/Dhaka\" - \"21\" for \"Asia/Jakarta\" - \"22\" for \"Asia/Kabul\" - \"23\" for \"Asia/Karachi\" - \"24\" for \"Asia/Calcutta\" - \"25\" for \"Asia/Pyongyang\" - \"26\" for \"Asia/Rangoon\" - \"27\" for \"Atlantic/Cape_Verde\" - \"28\" for \"Atlantic/South_Georgia\" - \"29\" for \"Australia/Adelaide\" - \"30\" for \"Australia/Lord_Howe\" - \"31\" for \"Europe/Moscow\" - \"32\" for \"Pacific/Kiritimati\" - \"35\" for \"Pacific/Norfolk\" - \"36\" for \"Pacific/Tongatapu\" "]
    pub report_generation_time_zone_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rich Media Exit Override."]
pub struct RichMediaExitOverride {
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Click-through URL of this rich media exit override. Applicable if the enabled field is set to true."]
    pub click_through_url: ::std::option::Option<::std::boxed::Box<ClickThroughUrl>>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to use the clickThroughUrl. If false, the creative-level exit will be used."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "exitId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID for the override to refer to a specific exit in the creative."]
    pub exit_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A rule associates an asset with a targeting template for asset-level targeting. Applicable to INSTREAM_VIDEO creatives."]
pub struct Rule {
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A creativeAssets[].id. This should refer to one of the parent assets in this creative. This is a required field."]
    pub asset_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A user-friendly name for this rule. This is a required field."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingTemplateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A targeting template ID. The targeting from the targeting template will be used to determine whether this asset should be served. This is a required field."]
    pub targeting_template_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a site."]
pub struct Site {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this site. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "approved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this site is approved."]
    pub approved: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "directorySiteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory site associated with this site. This is a required field that is read-only after insertion."]
    pub directory_site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directorySiteIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the directory site. This is a read-only, auto-generated field."]
    pub directory_site_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this site. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of this site. This is a read-only, auto-generated field."]
    pub id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "keyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key name of this site. This is a read-only, auto-generated field."]
    pub key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#site\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this site.This is a required field. Must be less than 128 characters long. If this site is under a subaccount, the name must be unique among sites of the same subaccount. Otherwise, this site is a top-level site, and the name must be unique among top-level sites of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site contacts."]
    pub site_contacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SiteContact>>>,
    #[serde(rename = "siteSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site-wide settings."]
    pub site_settings: ::std::option::Option<::std::boxed::Box<SiteSettings>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this site. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default video settings for new placements created under this site. This value will be used to populate the placements.videoSettings field, when no value is specified for the new placement."]
    pub video_settings: ::std::option::Option<::std::boxed::Box<SiteVideoSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Companion Settings"]
pub struct SiteCompanionSetting {
    #[serde(rename = "companionsDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether companions are disabled for this site template."]
    pub companions_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabledSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowlist of companion sizes to be served via this site template. Set this list to null or empty to serve all companion sizes."]
    pub enabled_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
    #[serde(rename = "imageOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to serve only static images as companions."]
    pub image_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#siteCompanionSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Site Contact"]
pub struct SiteContact {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Address of this site contact."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site contact type."]
    pub contact_type: ::std::option::Option<SiteContactContactTypeEnum>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of this site contact. This is a required field."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "First name of this site contact."]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this site contact. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last name of this site contact."]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Primary phone number of this site contact."]
    pub phone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title or designation of this site contact."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Site contact type."]
pub enum SiteContactContactTypeEnum {
    #[serde(rename = "SALES_PERSON")]
    #[doc = ""]
    SalesPerson,
    #[serde(rename = "TRAFFICKER")]
    #[doc = ""]
    Trafficker,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Site Settings"]
pub struct SiteSettings {
    #[serde(rename = "activeViewOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether active view creatives are disabled for this site."]
    pub active_view_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "adBlockingOptOut")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this site opts out of ad blocking. When true, ad blocking is disabled for all placements under the site, regardless of the individual placement settings. When false, the campaign and placement settings take effect."]
    pub ad_blocking_opt_out: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "disableNewCookie")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether new cookies are disabled for this site."]
    pub disable_new_cookie: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tagSetting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration settings for dynamic and image floodlight tags."]
    pub tag_setting: ::std::option::Option<::std::boxed::Box<TagSetting>>,
    #[serde(rename = "videoActiveViewOptOutTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Verification and ActiveView for in-stream video creatives are disabled by default for new placements created under this site. This value will be used to populate the placement.videoActiveViewOptOut field, when no value is specified for the new placement."]
    pub video_active_view_opt_out_template: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "vpaidAdapterChoiceTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default VPAID adapter setting for new placements created under this site. This value will be used to populate the placements.vpaidAdapterChoice field, when no value is specified for the new placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to the placement. The publisher's specifications will typically determine this setting. For VPAID creatives, the adapter format will match the VPAID format (HTML5 VPAID creatives use the HTML5 adapter). *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH."]
    pub vpaid_adapter_choice_template:
        ::std::option::Option<SiteSettingsVpaidAdapterChoiceTemplateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Default VPAID adapter setting for new placements created under this site. This value will be used to populate the placements.vpaidAdapterChoice field, when no value is specified for the new placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to the placement. The publisher's specifications will typically determine this setting. For VPAID creatives, the adapter format will match the VPAID format (HTML5 VPAID creatives use the HTML5 adapter). *Note:* Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH."]
pub enum SiteSettingsVpaidAdapterChoiceTemplateEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = ""]
    Default,
    #[serde(rename = "FLASH")]
    #[doc = ""]
    Flash,
    #[serde(rename = "HTML5")]
    #[doc = ""]
    Html5,
    #[serde(rename = "BOTH")]
    #[doc = ""]
    Both,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Skippable Settings"]
pub struct SiteSkippableSetting {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#siteSkippableSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play videos served to this site template before counting a view. Applicable when skippable is true."]
    pub progress_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "skipOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play videos served to this site before the skip button should appear. Applicable when skippable is true."]
    pub skip_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "skippable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can skip creatives served to this site. This will act as default for new placements created under this site."]
    pub skippable: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Transcode Settings"]
pub struct SiteTranscodeSetting {
    #[serde(rename = "enabledVideoFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowlist of video formats to be served to this site template. Set this list to null or empty to serve all video formats."]
    pub enabled_video_formats: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#siteTranscodeSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video Settings"]
pub struct SiteVideoSettings {
    #[serde(rename = "companionSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the companion creatives of video creatives served to this site."]
    pub companion_settings: ::std::option::Option<::std::boxed::Box<SiteCompanionSetting>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#siteVideoSettings\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Orientation of a site template used for video. This will act as default for new placements created under this site."]
    pub orientation: ::std::option::Option<SiteVideoSettingsOrientationEnum>,
    #[serde(rename = "skippableSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the skippability of video creatives served to this site. This will act as default for new placements created under this site."]
    pub skippable_settings: ::std::option::Option<::std::boxed::Box<SiteSkippableSetting>>,
    #[serde(rename = "transcodeSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the transcodes of video creatives served to this site. This will act as default for new placements created under this site."]
    pub transcode_settings: ::std::option::Option<::std::boxed::Box<SiteTranscodeSetting>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Orientation of a site template used for video. This will act as default for new placements created under this site."]
pub enum SiteVideoSettingsOrientationEnum {
    #[serde(rename = "ANY")]
    #[doc = ""]
    Any,
    #[serde(rename = "LANDSCAPE")]
    #[doc = ""]
    Landscape,
    #[serde(rename = "PORTRAIT")]
    #[doc = ""]
    Portrait,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Site List Response"]
pub struct SitesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#sitesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Site collection."]
    pub sites: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Site>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the dimensions of ads, placements, creatives, or creative assets."]
pub struct Size {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height of this size. Acceptable values are 0 to 32767, inclusive."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "iab")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IAB standard size. This is a read-only, auto-generated field."]
    pub iab: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this size. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#size\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of this size. Acceptable values are 0 to 32767, inclusive."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Size List Response"]
pub struct SizesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#sizesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size collection."]
    pub sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Skippable Settings"]
pub struct SkippableSetting {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#skippableSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play videos served to this placement before counting a view. Applicable when skippable is true."]
    pub progress_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "skipOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of time to play videos served to this placement before the skip button should appear. Applicable when skippable is true."]
    pub skip_offset: ::std::option::Option<::std::boxed::Box<VideoOffset>>,
    #[serde(rename = "skippable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the user can skip creatives served to this placement."]
    pub skippable: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a sorted dimension."]
pub struct SortedDimension {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource this is, in this case dfareporting#sortedDimension."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the dimension."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional sort order for the dimension column."]
    pub sort_order: ::std::option::Option<SortedDimensionSortOrderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "An optional sort order for the dimension column."]
pub enum SortedDimensionSortOrderEnum {
    #[serde(rename = "ASCENDING")]
    #[doc = ""]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = ""]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a Campaign Manager subaccount."]
pub struct Subaccount {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the account that contains this subaccount. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "availablePermissionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of the available user role permissions for this subaccount."]
    pub available_permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this subaccount. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#subaccount\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this subaccount. This is a required field. Must be less than 128 characters long and be unique among subaccounts of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Subaccount List Response"]
pub struct SubaccountsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#subaccountsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount collection."]
    pub subaccounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subaccount>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placement Tag Data"]
pub struct TagData {
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad associated with this placement tag. Applicable only when format is PLACEMENT_TAG_TRACKING."]
    pub ad_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag string to record a click."]
    pub click_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative associated with this placement tag. Applicable only when format is PLACEMENT_TAG_TRACKING."]
    pub creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "TagData tag format of this tag."]
    pub format: ::std::option::Option<TagDataFormatEnum>,
    #[serde(rename = "impressionTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag string for serving an ad."]
    pub impression_tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "TagData tag format of this tag."]
pub enum TagDataFormatEnum {
    #[serde(rename = "PLACEMENT_TAG_STANDARD")]
    #[doc = ""]
    PlacementTagStandard,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagIframeJavascript,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_ILAYER")]
    #[doc = ""]
    PlacementTagIframeIlayer,
    #[serde(rename = "PLACEMENT_TAG_INTERNAL_REDIRECT")]
    #[doc = ""]
    PlacementTagInternalRedirect,
    #[serde(rename = "PLACEMENT_TAG_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagJavascript,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagInterstitialIframeJavascript,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT")]
    #[doc = ""]
    PlacementTagInterstitialInternalRedirect,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagInterstitialJavascript,
    #[serde(rename = "PLACEMENT_TAG_CLICK_COMMANDS")]
    #[doc = ""]
    PlacementTagClickCommands,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetch,
    #[serde(rename = "PLACEMENT_TAG_TRACKING")]
    #[doc = ""]
    PlacementTagTracking,
    #[serde(rename = "PLACEMENT_TAG_TRACKING_IFRAME")]
    #[doc = ""]
    PlacementTagTrackingIframe,
    #[serde(rename = "PLACEMENT_TAG_TRACKING_JAVASCRIPT")]
    #[doc = ""]
    PlacementTagTrackingJavascript,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetchVast3,
    #[serde(rename = "PLACEMENT_TAG_IFRAME_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagIframeJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagInterstitialIframeJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT_LEGACY")]
    #[doc = ""]
    PlacementTagInterstitialJavascriptLegacy,
    #[serde(rename = "PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4")]
    #[doc = ""]
    PlacementTagInstreamVideoPrefetchVast4,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tag Settings"]
pub struct TagSetting {
    #[serde(rename = "additionalKeyValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional key-values to be included in tags. Each key-value pair must be of the form key=value, and pairs must be separated by a semicolon (;). Keys and values must not contain commas. For example, id=2;color=red is a valid value for this field."]
    pub additional_key_values: ::std::option::Option<::std::string::String>,
    #[serde(rename = "includeClickThroughUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether static landing page URLs should be included in the tags. This setting applies only to placements."]
    pub include_click_through_urls: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeClickTracking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether click-tracking string should be included in the tags."]
    pub include_click_tracking: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keywordOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders."]
    pub keyword_option: ::std::option::Option<TagSettingKeywordOptionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders."]
pub enum TagSettingKeywordOptionEnum {
    #[serde(rename = "PLACEHOLDER_WITH_LIST_OF_KEYWORDS")]
    #[doc = ""]
    PlaceholderWithListOfKeywords,
    #[serde(rename = "IGNORE")]
    #[doc = ""]
    Ignore,
    #[serde(rename = "GENERATE_SEPARATE_TAG_FOR_EACH_KEYWORD")]
    #[doc = ""]
    GenerateSeparateTagForEachKeyword,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dynamic and Image Tag Settings."]
pub struct TagSettings {
    #[serde(rename = "dynamicTagEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether dynamic floodlight tags are enabled."]
    pub dynamic_tag_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "imageTagEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether image tags are enabled."]
    pub image_tag_enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Target Window."]
pub struct TargetWindow {
    #[serde(rename = "customHtml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-entered value."]
    pub custom_html: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetWindowOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of browser window for which the backup image of the flash creative can be displayed."]
    pub target_window_option: ::std::option::Option<TargetWindowTargetWindowOptionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of browser window for which the backup image of the flash creative can be displayed."]
pub enum TargetWindowTargetWindowOptionEnum {
    #[serde(rename = "NEW_WINDOW")]
    #[doc = ""]
    NewWindow,
    #[serde(rename = "CURRENT_WINDOW")]
    #[doc = ""]
    CurrentWindow,
    #[serde(rename = "CUSTOM")]
    #[doc = ""]
    Custom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a targetable remarketing list. Remarketing enables you to create lists of users who have performed specific actions on a site, then target ads to members of those lists. This resource is a read-only view of a remarketing list to be used to faciliate targeting ads to specific lists. Remarketing lists that are owned by your advertisers and those that are shared to your advertisers or account are accessible via this resource. To manage remarketing lists that are owned by your advertisers, use the RemarketingLists resource."]
pub struct TargetableRemarketingList {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this targetable remarketing list is active."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the advertiser ID that owns this targetable remarketing list."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targetable remarketing list description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targetable remarketing list ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#targetableRemarketingList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lifeSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of days that a user should remain in the targetable remarketing list without an impression."]
    pub life_span: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of users currently in the list. This is a read-only field."]
    pub list_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product from which this targetable remarketing list was originated."]
    pub list_source: ::std::option::Option<TargetableRemarketingListListSourceEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the targetable remarketing list. Is no greater than 128 characters long."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Product from which this targetable remarketing list was originated."]
pub enum TargetableRemarketingListListSourceEnum {
    #[serde(rename = "REMARKETING_LIST_SOURCE_OTHER")]
    #[doc = ""]
    RemarketingListSourceOther,
    #[serde(rename = "REMARKETING_LIST_SOURCE_ADX")]
    #[doc = ""]
    RemarketingListSourceAdx,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DFP")]
    #[doc = ""]
    RemarketingListSourceDfp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_XFP")]
    #[doc = ""]
    RemarketingListSourceXfp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DFA")]
    #[doc = ""]
    RemarketingListSourceDfa,
    #[serde(rename = "REMARKETING_LIST_SOURCE_GA")]
    #[doc = ""]
    RemarketingListSourceGa,
    #[serde(rename = "REMARKETING_LIST_SOURCE_YOUTUBE")]
    #[doc = ""]
    RemarketingListSourceYoutube,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DBM")]
    #[doc = ""]
    RemarketingListSourceDbm,
    #[serde(rename = "REMARKETING_LIST_SOURCE_GPLUS")]
    #[doc = ""]
    RemarketingListSourceGplus,
    #[serde(rename = "REMARKETING_LIST_SOURCE_DMP")]
    #[doc = ""]
    RemarketingListSourceDmp,
    #[serde(rename = "REMARKETING_LIST_SOURCE_PLAY_STORE")]
    #[doc = ""]
    RemarketingListSourcePlayStore,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targetable remarketing list response"]
pub struct TargetableRemarketingListsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#targetableRemarketingListsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetableRemarketingLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targetable remarketing list collection."]
    pub targetable_remarketing_lists:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetableRemarketingList>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a targeting template. A targeting template encapsulates targeting information which can be reused across multiple ads."]
pub struct TargetingTemplate {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser ID of this targeting template. This is a required field on insert and is read-only after insert."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserIdDimensionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimension value for the ID of the advertiser. This is a read-only, auto-generated field."]
    pub advertiser_id_dimension_value: ::std::option::Option<::std::boxed::Box<DimensionValue>>,
    #[serde(rename = "dayPartTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time and day targeting criteria."]
    pub day_part_targeting: ::std::option::Option<::std::boxed::Box<DayPartTargeting>>,
    #[serde(rename = "geoTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographical targeting criteria."]
    pub geo_targeting: ::std::option::Option<::std::boxed::Box<GeoTargeting>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this targeting template. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyValueTargetingExpression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key-value targeting criteria."]
    pub key_value_targeting_expression:
        ::std::option::Option<::std::boxed::Box<KeyValueTargetingExpression>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#targetingTemplate\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language targeting criteria."]
    pub language_targeting: ::std::option::Option<::std::boxed::Box<LanguageTargeting>>,
    #[serde(rename = "listTargetingExpression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remarketing list targeting criteria."]
    pub list_targeting_expression:
        ::std::option::Option<::std::boxed::Box<ListTargetingExpression>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this targeting template. This field is required. It must be less than 256 characters long and unique within an advertiser."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "technologyTargeting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Technology platform targeting criteria."]
    pub technology_targeting: ::std::option::Option<::std::boxed::Box<TechnologyTargeting>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targeting Template List Response"]
pub struct TargetingTemplatesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#targetingTemplatesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingTemplates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targeting template collection."]
    pub targeting_templates:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingTemplate>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Technology Targeting."]
pub struct TechnologyTargeting {
    #[serde(rename = "browsers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Browsers that this ad targets. For each browser either set browserVersionId or dartId along with the version numbers. If both are specified, only browserVersionId will be used. The other fields are populated automatically when the ad is inserted or updated."]
    pub browsers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Browser>>>,
    #[serde(rename = "connectionTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Connection types that this ad targets. For each connection type only id is required. The other fields are populated automatically when the ad is inserted or updated."]
    pub connection_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectionType>>>,
    #[serde(rename = "mobileCarriers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mobile carriers that this ad targets. For each mobile carrier only id is required, and the other fields are populated automatically when the ad is inserted or updated. If targeting a mobile carrier, do not set targeting for any zip codes."]
    pub mobile_carriers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileCarrier>>>,
    #[serde(rename = "operatingSystemVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating system versions that this ad targets. To target all versions, use operatingSystems. For each operating system version, only id is required. The other fields are populated automatically when the ad is inserted or updated. If targeting an operating system version, do not set targeting for the corresponding operating system in operatingSystems."]
    pub operating_system_versions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperatingSystemVersion>>>,
    #[serde(rename = "operatingSystems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operating systems that this ad targets. To target specific versions, use operatingSystemVersions. For each operating system only dartId is required. The other fields are populated automatically when the ad is inserted or updated. If targeting an operating system, do not set targeting for operating system versions for the same operating system."]
    pub operating_systems:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperatingSystem>>>,
    #[serde(rename = "platformTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform types that this ad targets. For example, desktop, mobile, or tablet. For each platform type, only id is required, and the other fields are populated automatically when the ad is inserted or updated."]
    pub platform_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlatformType>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Third Party Authentication Token"]
pub struct ThirdPartyAuthenticationToken {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the third-party authentication token."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the third-party authentication token. This is a read-only, auto-generated field."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Third-party Tracking URL."]
pub struct ThirdPartyTrackingUrl {
    #[serde(rename = "thirdPartyUrlType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party URL type for in-stream video and in-stream audio creatives."]
    pub third_party_url_type: ::std::option::Option<ThirdPartyTrackingUrlThirdPartyUrlTypeEnum>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for the specified third-party URL type."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Third-party URL type for in-stream video and in-stream audio creatives."]
pub enum ThirdPartyTrackingUrlThirdPartyUrlTypeEnum {
    #[serde(rename = "IMPRESSION")]
    #[doc = ""]
    Impression,
    #[serde(rename = "CLICK_TRACKING")]
    #[doc = ""]
    ClickTracking,
    #[serde(rename = "VIDEO_START")]
    #[doc = ""]
    VideoStart,
    #[serde(rename = "VIDEO_FIRST_QUARTILE")]
    #[doc = ""]
    VideoFirstQuartile,
    #[serde(rename = "VIDEO_MIDPOINT")]
    #[doc = ""]
    VideoMidpoint,
    #[serde(rename = "VIDEO_THIRD_QUARTILE")]
    #[doc = ""]
    VideoThirdQuartile,
    #[serde(rename = "VIDEO_COMPLETE")]
    #[doc = ""]
    VideoComplete,
    #[serde(rename = "VIDEO_MUTE")]
    #[doc = ""]
    VideoMute,
    #[serde(rename = "VIDEO_PAUSE")]
    #[doc = ""]
    VideoPause,
    #[serde(rename = "VIDEO_REWIND")]
    #[doc = ""]
    VideoRewind,
    #[serde(rename = "VIDEO_FULLSCREEN")]
    #[doc = ""]
    VideoFullscreen,
    #[serde(rename = "VIDEO_STOP")]
    #[doc = ""]
    VideoStop,
    #[serde(rename = "VIDEO_CUSTOM")]
    #[doc = ""]
    VideoCustom,
    #[serde(rename = "SURVEY")]
    #[doc = ""]
    Survey,
    #[serde(rename = "RICH_MEDIA_IMPRESSION")]
    #[doc = ""]
    RichMediaImpression,
    #[serde(rename = "RICH_MEDIA_RM_IMPRESSION")]
    #[doc = ""]
    RichMediaRmImpression,
    #[serde(rename = "RICH_MEDIA_BACKUP_IMPRESSION")]
    #[doc = ""]
    RichMediaBackupImpression,
    #[serde(rename = "VIDEO_SKIP")]
    #[doc = ""]
    VideoSkip,
    #[serde(rename = "VIDEO_PROGRESS")]
    #[doc = ""]
    VideoProgress,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Transcode Settings"]
pub struct TranscodeSetting {
    #[serde(rename = "enabledVideoFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowlist of video formats to be served to this placement. Set this list to null or empty to serve all video formats."]
    pub enabled_video_formats: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#transcodeSetting\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Universal Ad ID as per the VAST 4.0 spec. Applicable to the following creative types: INSTREAM_AUDIO, INSTREAM_VIDEO and VPAID."]
pub struct UniversalAdId {
    #[serde(rename = "registry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Registry used for the Ad ID value."]
    pub registry: ::std::option::Option<UniversalAdIdRegistryEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID value for this creative. Only alphanumeric characters and the following symbols are valid: \"_/\\-\". Maximum length is 64 characters. Read only when registry is DCM."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Registry used for the Ad ID value."]
pub enum UniversalAdIdRegistryEnum {
    #[serde(rename = "OTHER")]
    #[doc = ""]
    Other,
    #[serde(rename = "AD_ID.ORG")]
    #[doc = ""]
    AdIdOrg,
    #[serde(rename = "CLEARCAST")]
    #[doc = ""]
    Clearcast,
    #[serde(rename = "DCM")]
    #[doc = ""]
    Dcm,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User Defined Variable configuration."]
pub struct UserDefinedVariableConfiguration {
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data type for the variable. This is a required field."]
    pub data_type: ::std::option::Option<UserDefinedVariableConfigurationDataTypeEnum>,
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-friendly name for the variable which will appear in reports. This is a required field, must be less than 64 characters long, and cannot contain the following characters: \"\"<>\"."]
    pub report_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Variable name in the tag. This is a required field."]
    pub variable_type: ::std::option::Option<UserDefinedVariableConfigurationVariableTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data type for the variable. This is a required field."]
pub enum UserDefinedVariableConfigurationDataTypeEnum {
    #[serde(rename = "STRING")]
    #[doc = ""]
    String,
    #[serde(rename = "NUMBER")]
    #[doc = ""]
    Number,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Variable name in the tag. This is a required field."]
pub enum UserDefinedVariableConfigurationVariableTypeEnum {
    #[serde(rename = "U1")]
    #[doc = ""]
    U1,
    #[serde(rename = "U2")]
    #[doc = ""]
    U2,
    #[serde(rename = "U3")]
    #[doc = ""]
    U3,
    #[serde(rename = "U4")]
    #[doc = ""]
    U4,
    #[serde(rename = "U5")]
    #[doc = ""]
    U5,
    #[serde(rename = "U6")]
    #[doc = ""]
    U6,
    #[serde(rename = "U7")]
    #[doc = ""]
    U7,
    #[serde(rename = "U8")]
    #[doc = ""]
    U8,
    #[serde(rename = "U9")]
    #[doc = ""]
    U9,
    #[serde(rename = "U10")]
    #[doc = ""]
    U10,
    #[serde(rename = "U11")]
    #[doc = ""]
    U11,
    #[serde(rename = "U12")]
    #[doc = ""]
    U12,
    #[serde(rename = "U13")]
    #[doc = ""]
    U13,
    #[serde(rename = "U14")]
    #[doc = ""]
    U14,
    #[serde(rename = "U15")]
    #[doc = ""]
    U15,
    #[serde(rename = "U16")]
    #[doc = ""]
    U16,
    #[serde(rename = "U17")]
    #[doc = ""]
    U17,
    #[serde(rename = "U18")]
    #[doc = ""]
    U18,
    #[serde(rename = "U19")]
    #[doc = ""]
    U19,
    #[serde(rename = "U20")]
    #[doc = ""]
    U20,
    #[serde(rename = "U21")]
    #[doc = ""]
    U21,
    #[serde(rename = "U22")]
    #[doc = ""]
    U22,
    #[serde(rename = "U23")]
    #[doc = ""]
    U23,
    #[serde(rename = "U24")]
    #[doc = ""]
    U24,
    #[serde(rename = "U25")]
    #[doc = ""]
    U25,
    #[serde(rename = "U26")]
    #[doc = ""]
    U26,
    #[serde(rename = "U27")]
    #[doc = ""]
    U27,
    #[serde(rename = "U28")]
    #[doc = ""]
    U28,
    #[serde(rename = "U29")]
    #[doc = ""]
    U29,
    #[serde(rename = "U30")]
    #[doc = ""]
    U30,
    #[serde(rename = "U31")]
    #[doc = ""]
    U31,
    #[serde(rename = "U32")]
    #[doc = ""]
    U32,
    #[serde(rename = "U33")]
    #[doc = ""]
    U33,
    #[serde(rename = "U34")]
    #[doc = ""]
    U34,
    #[serde(rename = "U35")]
    #[doc = ""]
    U35,
    #[serde(rename = "U36")]
    #[doc = ""]
    U36,
    #[serde(rename = "U37")]
    #[doc = ""]
    U37,
    #[serde(rename = "U38")]
    #[doc = ""]
    U38,
    #[serde(rename = "U39")]
    #[doc = ""]
    U39,
    #[serde(rename = "U40")]
    #[doc = ""]
    U40,
    #[serde(rename = "U41")]
    #[doc = ""]
    U41,
    #[serde(rename = "U42")]
    #[doc = ""]
    U42,
    #[serde(rename = "U43")]
    #[doc = ""]
    U43,
    #[serde(rename = "U44")]
    #[doc = ""]
    U44,
    #[serde(rename = "U45")]
    #[doc = ""]
    U45,
    #[serde(rename = "U46")]
    #[doc = ""]
    U46,
    #[serde(rename = "U47")]
    #[doc = ""]
    U47,
    #[serde(rename = "U48")]
    #[doc = ""]
    U48,
    #[serde(rename = "U49")]
    #[doc = ""]
    U49,
    #[serde(rename = "U50")]
    #[doc = ""]
    U50,
    #[serde(rename = "U51")]
    #[doc = ""]
    U51,
    #[serde(rename = "U52")]
    #[doc = ""]
    U52,
    #[serde(rename = "U53")]
    #[doc = ""]
    U53,
    #[serde(rename = "U54")]
    #[doc = ""]
    U54,
    #[serde(rename = "U55")]
    #[doc = ""]
    U55,
    #[serde(rename = "U56")]
    #[doc = ""]
    U56,
    #[serde(rename = "U57")]
    #[doc = ""]
    U57,
    #[serde(rename = "U58")]
    #[doc = ""]
    U58,
    #[serde(rename = "U59")]
    #[doc = ""]
    U59,
    #[serde(rename = "U60")]
    #[doc = ""]
    U60,
    #[serde(rename = "U61")]
    #[doc = ""]
    U61,
    #[serde(rename = "U62")]
    #[doc = ""]
    U62,
    #[serde(rename = "U63")]
    #[doc = ""]
    U63,
    #[serde(rename = "U64")]
    #[doc = ""]
    U64,
    #[serde(rename = "U65")]
    #[doc = ""]
    U65,
    #[serde(rename = "U66")]
    #[doc = ""]
    U66,
    #[serde(rename = "U67")]
    #[doc = ""]
    U67,
    #[serde(rename = "U68")]
    #[doc = ""]
    U68,
    #[serde(rename = "U69")]
    #[doc = ""]
    U69,
    #[serde(rename = "U70")]
    #[doc = ""]
    U70,
    #[serde(rename = "U71")]
    #[doc = ""]
    U71,
    #[serde(rename = "U72")]
    #[doc = ""]
    U72,
    #[serde(rename = "U73")]
    #[doc = ""]
    U73,
    #[serde(rename = "U74")]
    #[doc = ""]
    U74,
    #[serde(rename = "U75")]
    #[doc = ""]
    U75,
    #[serde(rename = "U76")]
    #[doc = ""]
    U76,
    #[serde(rename = "U77")]
    #[doc = ""]
    U77,
    #[serde(rename = "U78")]
    #[doc = ""]
    U78,
    #[serde(rename = "U79")]
    #[doc = ""]
    U79,
    #[serde(rename = "U80")]
    #[doc = ""]
    U80,
    #[serde(rename = "U81")]
    #[doc = ""]
    U81,
    #[serde(rename = "U82")]
    #[doc = ""]
    U82,
    #[serde(rename = "U83")]
    #[doc = ""]
    U83,
    #[serde(rename = "U84")]
    #[doc = ""]
    U84,
    #[serde(rename = "U85")]
    #[doc = ""]
    U85,
    #[serde(rename = "U86")]
    #[doc = ""]
    U86,
    #[serde(rename = "U87")]
    #[doc = ""]
    U87,
    #[serde(rename = "U88")]
    #[doc = ""]
    U88,
    #[serde(rename = "U89")]
    #[doc = ""]
    U89,
    #[serde(rename = "U90")]
    #[doc = ""]
    U90,
    #[serde(rename = "U91")]
    #[doc = ""]
    U91,
    #[serde(rename = "U92")]
    #[doc = ""]
    U92,
    #[serde(rename = "U93")]
    #[doc = ""]
    U93,
    #[serde(rename = "U94")]
    #[doc = ""]
    U94,
    #[serde(rename = "U95")]
    #[doc = ""]
    U95,
    #[serde(rename = "U96")]
    #[doc = ""]
    U96,
    #[serde(rename = "U97")]
    #[doc = ""]
    U97,
    #[serde(rename = "U98")]
    #[doc = ""]
    U98,
    #[serde(rename = "U99")]
    #[doc = ""]
    U99,
    #[serde(rename = "U100")]
    #[doc = ""]
    U100,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A UserProfile resource lets you list all DFA user profiles that are associated with a Google user account. The profile_id needs to be specified in other API requests. "]
pub struct UserProfile {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID to which this profile belongs."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account name this profile belongs to."]
    pub account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of this resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userProfile\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of the user profile."]
    pub profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub account ID this profile belongs to if applicable."]
    pub sub_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subAccountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub account name this profile belongs to if applicable."]
    pub sub_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user name."]
    pub user_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the list of user profiles."]
pub struct UserProfileList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of this resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user profiles returned in this response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserProfile>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userProfileList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of auser role, which is used to manage user access."]
pub struct UserRole {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account ID of this user role. This is a read-only field that can be left blank."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultUserRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this is a default user role. Default user roles are created by the system for the account/subaccount and cannot be modified or deleted. Each default user role comes with a basic set of preassigned permissions."]
    pub default_user_role: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this user role. This is a read-only, auto-generated field."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRole\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this user role. This is a required field. Must be less than 256 characters long. If this user role is under a subaccount, the name must be unique among sites of the same subaccount. Otherwise, this user role is a top-level user role, and the name must be unique among top-level user roles of the same account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentUserRoleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the user role that this user role is based on or copied from. This is a required field."]
    pub parent_user_role_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of permissions associated with this user role."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserRolePermission>>>,
    #[serde(rename = "subaccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subaccount ID of this user role. This is a read-only field that can be left blank."]
    pub subaccount_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains properties of a user role permission."]
pub struct UserRolePermission {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Levels of availability for a user role permission."]
    pub availability: ::std::option::Option<UserRolePermissionAvailabilityEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this user role permission."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRolePermission\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this user role permission."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissionGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the permission group that this user role permission belongs to."]
    pub permission_group_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Levels of availability for a user role permission."]
pub enum UserRolePermissionAvailabilityEnum {
    #[serde(rename = "NOT_AVAILABLE_BY_DEFAULT")]
    #[doc = ""]
    NotAvailableByDefault,
    #[serde(rename = "ACCOUNT_BY_DEFAULT")]
    #[doc = ""]
    AccountByDefault,
    #[serde(rename = "SUBACCOUNT_AND_ACCOUNT_BY_DEFAULT")]
    #[doc = ""]
    SubaccountAndAccountByDefault,
    #[serde(rename = "ACCOUNT_ALWAYS")]
    #[doc = ""]
    AccountAlways,
    #[serde(rename = "SUBACCOUNT_AND_ACCOUNT_ALWAYS")]
    #[doc = ""]
    SubaccountAndAccountAlways,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a grouping of related user role permissions."]
pub struct UserRolePermissionGroup {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this user role permission."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRolePermissionGroup\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this user role permission group."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User Role Permission Group List Response"]
pub struct UserRolePermissionGroupsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRolePermissionGroupsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userRolePermissionGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User role permission group collection."]
    pub user_role_permission_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserRolePermissionGroup>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User Role Permission List Response"]
pub struct UserRolePermissionsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRolePermissionsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userRolePermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User role permission collection."]
    pub user_role_permissions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserRolePermission>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User Role List Response"]
pub struct UserRolesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#userRolesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to be used for the next list operation."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User role collection."]
    pub user_roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserRole>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about supported video formats."]
pub struct VideoFormat {
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File type of the video format."]
    pub file_type: ::std::option::Option<VideoFormatFileTypeEnum>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the video format."]
    pub id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#videoFormat\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resolution of this video format."]
    pub resolution: ::std::option::Option<::std::boxed::Box<Size>>,
    #[serde(rename = "targetBitRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target bit rate of this video format."]
    pub target_bit_rate: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "File type of the video format."]
pub enum VideoFormatFileTypeEnum {
    #[serde(rename = "FLV")]
    #[doc = ""]
    Flv,
    #[serde(rename = "THREEGPP")]
    #[doc = ""]
    Threegpp,
    #[serde(rename = "MP4")]
    #[doc = ""]
    Mp4,
    #[serde(rename = "WEBM")]
    #[doc = ""]
    Webm,
    #[serde(rename = "M3U8")]
    #[doc = ""]
    M3U8,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video Format List Response"]
pub struct VideoFormatsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#videoFormatsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video format collection."]
    pub video_formats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoFormat>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video Offset"]
pub struct VideoOffset {
    #[serde(rename = "offsetPercentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive."]
    pub offset_percentage: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "offsetSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive."]
    pub offset_seconds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video Settings"]
pub struct VideoSettings {
    #[serde(rename = "companionSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the companion creatives of video creatives served to this placement."]
    pub companion_settings: ::std::option::Option<::std::boxed::Box<CompanionSetting>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dfareporting#videoSettings\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Orientation of a video placement. If this value is set, placement will return assets matching the specified orientation."]
    pub orientation: ::std::option::Option<VideoSettingsOrientationEnum>,
    #[serde(rename = "skippableSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the skippability of video creatives served to this placement. If this object is provided, the creative-level skippable settings will be overridden."]
    pub skippable_settings: ::std::option::Option<::std::boxed::Box<SkippableSetting>>,
    #[serde(rename = "transcodeSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for the transcodes of video creatives served to this placement. If this object is provided, the creative-level transcode settings will be overridden."]
    pub transcode_settings: ::std::option::Option<::std::boxed::Box<TranscodeSetting>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Orientation of a video placement. If this value is set, placement will return assets matching the specified orientation."]
pub enum VideoSettingsOrientationEnum {
    #[serde(rename = "ANY")]
    #[doc = ""]
    Any,
    #[serde(rename = "LANDSCAPE")]
    #[doc = ""]
    Landscape,
    #[serde(rename = "PORTRAIT")]
    #[doc = ""]
    Portrait,
}
