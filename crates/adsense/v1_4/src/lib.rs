#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    #[serde(rename = "creation_time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this account."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "account_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#account."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "premium")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this account is premium."]
    pub premium: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub accounts of the this account."]
    pub sub_accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "AdSense timezone of this account."]
    pub timezone: ::std::option::Option<::std::string::String>,
}
mod account_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#account")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accounts {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accounts returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
    #[serde(rename = "kind")]
    #[serde(default = "accounts_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#accounts."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through accounts. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod accounts_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#accounts")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdClient {
    #[serde(rename = "arcOptIn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad client is opted in to ARC."]
    pub arc_opt_in: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this ad client."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_client_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#adClient."]
    pub kind: ::std::string::String,
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This ad client's product code, which corresponds to the PRODUCT_CODE report dimension."]
    pub product_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportsReporting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this ad client supports being reported on."]
    pub supports_reporting: ::std::option::Option<::std::primitive::bool>,
}
mod ad_client_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adClient")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdClients {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ad clients returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdClient>>>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_clients_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#adClients."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod ad_clients_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adClients")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdCode {
    #[serde(rename = "adCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Auto ad code snippet. The ad code snippet."]
    pub ad_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ampBody")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AMP Auto ad code snippet that goes in the body of an AMP page."]
    pub amp_body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ampHead")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AMP Auto ad code snippet that goes in the head of an AMP page."]
    pub amp_head: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_code_defaults :: kind")]
    #[doc = "Kind this is, in this case adsense#adCode."]
    pub kind: ::std::string::String,
}
mod ad_code_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adCode")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdStyle {
    #[serde(rename = "colors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
    pub colors: ::std::option::Option<AdStyleColors>,
    #[serde(rename = "corners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The style of the corners in the ad (deprecated: never populated, ignored)."]
    pub corners: ::std::option::Option<::std::string::String>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The font which is included in the style."]
    pub font: ::std::option::Option<AdStyleFont>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_style_defaults :: kind")]
    #[doc = "Kind this is, in this case adsense#adStyle."]
    pub kind: ::std::string::String,
}
mod ad_style_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adStyle")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
pub struct AdStyleColors {
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the ad background."]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(rename = "border")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the ad border."]
    pub border: ::std::option::Option<::std::string::String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the ad text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the ad title."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the ad url."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The font which is included in the style."]
pub struct AdStyleFont {
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The family of the font."]
    pub family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the font."]
    pub size: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdUnit {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identity code of this ad unit, not necessarily unique across ad clients."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentAdsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated)."]
    pub content_ads_settings: ::std::option::Option<AdUnitContentAdsSettings>,
    #[serde(rename = "customStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom style information specific to this ad unit."]
    pub custom_style: ::std::option::Option<::std::boxed::Box<AdStyle>>,
    #[serde(rename = "feedAdsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings specific to feed ads (AFF) - deprecated."]
    pub feed_ads_settings: ::std::option::Option<AdUnitFeedAdsSettings>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_unit_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#adUnit."]
    pub kind: ::std::string::String,
    #[serde(rename = "mobileContentAdsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings specific to WAP mobile content ads (AFMC) - deprecated."]
    pub mobile_content_ads_settings: ::std::option::Option<AdUnitMobileContentAdsSettings>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this ad unit."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "savedStyleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the saved ad style which holds this ad unit's style information."]
    pub saved_style_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this ad unit. Possible values are:\nNEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.\n\nACTIVE: Indicates that there has been activity on this ad unit in the last seven days.\n\nINACTIVE: Indicates that there has been no activity on this ad unit in the last seven days."]
    pub status: ::std::option::Option<::std::string::String>,
}
mod ad_unit_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adUnit")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated)."]
pub struct AdUnitContentAdsSettings {
    #[serde(rename = "backupOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The backup option to be used in instances where no ad is available."]
    pub backup_option: ::std::option::Option<AdUnitContentAdsSettingsBackupOption>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of this ad unit."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this ad unit."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The backup option to be used in instances where no ad is available."]
pub struct AdUnitContentAdsSettingsBackupOption {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Color to use when type is set to COLOR."]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the backup option. Possible values are BLANK, COLOR and URL."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to use when type is set to URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings specific to feed ads (AFF) - deprecated."]
pub struct AdUnitFeedAdsSettings {
    #[serde(rename = "adPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The position of the ads relative to the feed entries."]
    pub ad_position: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The frequency at which ads should appear in the feed (i.e. every N entries)."]
    pub frequency: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minimumWordCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum length an entry should be in order to have attached ads."]
    pub minimum_word_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of ads which should appear."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings specific to WAP mobile content ads (AFMC) - deprecated."]
pub struct AdUnitMobileContentAdsSettings {
    #[serde(rename = "markupLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The markup language to use for this ad unit."]
    pub markup_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scriptingLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scripting language to use for this ad unit."]
    pub scripting_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of this ad unit."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this ad unit."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdUnits {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ad units returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdUnit>>>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_units_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#adUnits."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod ad_units_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#adUnits")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdsenseReportsGenerateResponse {
    #[serde(rename = "averages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
    pub averages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested end date in yyyy-mm-dd format."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."]
    pub headers: ::std::option::Option<::std::vec::Vec<AdsenseReportsGenerateResponseHeaders>>,
    #[serde(rename = "kind")]
    #[serde(default = "adsense_reports_generate_response_defaults :: kind")]
    #[doc = "Kind this is, in this case adsense#report."]
    pub kind: ::std::string::String,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested start date in yyyy-mm-dd format."]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalMatchedRows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit."]
    pub total_matched_rows: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
    pub totals: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any warnings associated with generation of the report."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod adsense_reports_generate_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#report")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdsenseReportsGenerateResponseHeaders {
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency of this column. Only present if the header type is METRIC_CURRENCY."]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the header."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the header; one of DIMENSION, METRIC_TALLY, METRIC_RATIO, or METRIC_CURRENCY."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alert {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this alert. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isDismissible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this alert can be dismissed."]
    pub is_dismissible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "alert_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#alert."]
    pub kind: ::std::string::String,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized alert message."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Severity of this alert. Possible values: INFO, WARNING, SEVERE."]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this alert. Possible values: SELF_HOLD, MIGRATED_TO_BILLING3, ADDRESS_PIN_VERIFICATION, PHONE_PIN_VERIFICATION, CORPORATE_ENTITY, GRAYLISTED_PUBLISHER, API_HOLD."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod alert_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#alert")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alerts {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alerts returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Alert>>>,
    #[serde(rename = "kind")]
    #[serde(default = "alerts_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#alerts."]
    pub kind: ::std::string::String,
}
mod alerts_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#alerts")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomChannel {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code of this custom channel, not necessarily unique across ad clients."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "custom_channel_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#customChannel."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this custom channel."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The targeting information of this custom channel, if activated."]
    pub targeting_info: ::std::option::Option<CustomChannelTargetingInfo>,
}
mod custom_channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#customChannel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The targeting information of this custom channel, if activated."]
pub struct CustomChannelTargetingInfo {
    #[serde(rename = "adsAppearOn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name used to describe this channel externally."]
    pub ads_appear_on: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The external description of the channel."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locations in which ads appear. (Only valid for content and mobile content ads (deprecated)). Acceptable values for content ads are: TOP_LEFT, TOP_CENTER, TOP_RIGHT, MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT, MULTIPLE_LOCATIONS. Acceptable values for mobile content ads (deprecated) are: TOP, MIDDLE, BOTTOM, MULTIPLE_LOCATIONS."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the sites ads will be displayed on."]
    pub site_language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomChannels {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom channels returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomChannel>>>,
    #[serde(rename = "kind")]
    #[serde(default = "custom_channels_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#customChannels."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through custom channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod custom_channels_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#customChannels")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportingMetadataEntry>>>,
    #[serde(rename = "kind")]
    #[serde(default = "metadata_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#metadata."]
    pub kind: ::std::string::String,
}
mod metadata_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#metadata")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Payment {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this Payment."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "payment_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#payment."]
    pub kind: ::std::string::String,
    #[serde(rename = "paymentAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount to be paid."]
    pub payment_amount: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paymentAmountCurrencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code for the amount to be paid."]
    pub payment_amount_currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paymentDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date this payment was/will be credited to the user, or none if the payment threshold has not been met."]
    pub payment_date: ::std::option::Option<::std::string::String>,
}
mod payment_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#payment")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Payments {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Payments for the account. One or both of a) the account's most recent payment; and b) the account's upcoming payment."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Payment>>>,
    #[serde(rename = "kind")]
    #[serde(default = "payments_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#payments."]
    pub kind: ::std::string::String,
}
mod payments_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#payments")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportingMetadataEntry {
    #[serde(rename = "compatibleDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For metrics this is a list of dimension IDs which the metric is compatible with, for dimensions it is a list of compatibility groups the dimension belongs to."]
    pub compatible_dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "compatibleMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the metrics the dimension or metric this reporting metadata entry describes is compatible with."]
    pub compatible_metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this reporting metadata entry, corresponding to the name of the appropriate dimension or metric."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "reporting_metadata_entry_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#reportingMetadataEntry."]
    pub kind: ::std::string::String,
    #[serde(rename = "requiredDimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the dimensions which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
    pub required_dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "requiredMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the metrics which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
    pub required_metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "supportedProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The codes of the projects supported by the dimension or metric this reporting metadata entry describes."]
    pub supported_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod reporting_metadata_entry_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#reportingMetadataEntry")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedAdStyle {
    #[serde(rename = "adStyle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AdStyle itself."]
    pub ad_style: ::std::option::Option<::std::boxed::Box<AdStyle>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "saved_ad_style_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#savedAdStyle."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user selected name of this SavedAdStyle."]
    pub name: ::std::option::Option<::std::string::String>,
}
mod saved_ad_style_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#savedAdStyle")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedAdStyles {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The saved ad styles returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedAdStyle>>>,
    #[serde(rename = "kind")]
    #[serde(default = "saved_ad_styles_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#savedAdStyles."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod saved_ad_styles_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#savedAdStyles")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedReport {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this saved report."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "saved_report_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#savedReport."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This saved report's name."]
    pub name: ::std::option::Option<::std::string::String>,
}
mod saved_report_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#savedReport")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedReports {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The saved reports returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedReport>>>,
    #[serde(rename = "kind")]
    #[serde(default = "saved_reports_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#savedReports."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through saved reports. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod saved_reports_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#savedReports")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlChannel {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "url_channel_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsense#urlChannel."]
    pub kind: ::std::string::String,
    #[serde(rename = "urlPattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL Pattern of this URL channel. Does not include \"http://\" or \"https://\". Example: www.example.com/home"]
    pub url_pattern: ::std::option::Option<::std::string::String>,
}
mod url_channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#urlChannel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlChannels {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of this response for caching purposes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL channels returned in this list response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UrlChannel>>>,
    #[serde(rename = "kind")]
    #[serde(default = "url_channels_defaults :: kind")]
    #[doc = "Kind of list this is, in this case adsense#urlChannels."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod url_channels_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsense#urlChannels")
    }
}
