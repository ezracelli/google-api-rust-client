#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this account."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "account_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsensehost#account."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approval status of this account. One of: PENDING, APPROVED, DISABLED."]
    pub status: ::std::option::Option<::std::string::String>,
}
mod account_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#account")
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
    #[doc = "Kind of list this is, in this case adsensehost#accounts."]
    pub kind: ::std::string::String,
}
mod accounts_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#accounts")
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
    #[doc = "Kind of resource this is, in this case adsensehost#adClient."]
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
        String::from("adsensehost#adClient")
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
    #[doc = "Kind of list this is, in this case adsensehost#adClients."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod ad_clients_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#adClients")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdCode {
    #[serde(rename = "adCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ad code snippet."]
    pub ad_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_code_defaults :: kind")]
    #[doc = "Kind this is, in this case adsensehost#adCode."]
    pub kind: ::std::string::String,
}
mod ad_code_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#adCode")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdStyle {
    #[serde(rename = "colors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
    #[doc = "Kind this is, in this case adsensehost#adStyle."]
    pub kind: ::std::string::String,
}
mod ad_style_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#adStyle")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
    #[doc = "The family of the font. Possible values are: ACCOUNT_DEFAULT_FAMILY, ADSENSE_DEFAULT_FAMILY, ARIAL, TIMES and VERDANA."]
    pub family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the font. Possible values are: ACCOUNT_DEFAULT_SIZE, ADSENSE_DEFAULT_SIZE, SMALL, MEDIUM and LARGE."]
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
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "ad_unit_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsensehost#adUnit."]
    pub kind: ::std::string::String,
    #[serde(rename = "mobileContentAdsSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings specific to WAP mobile content ads (AFMC - deprecated)."]
    pub mobile_content_ads_settings: ::std::option::Option<AdUnitMobileContentAdsSettings>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this ad unit."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of this ad unit. Possible values are:\nNEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.\n\nACTIVE: Indicates that there has been activity on this ad unit in the last seven days.\n\nINACTIVE: Indicates that there has been no activity on this ad unit in the last seven days."]
    pub status: ::std::option::Option<::std::string::String>,
}
mod ad_unit_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#adUnit")
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
    #[doc = "Size of this ad unit. Size values are in the form SIZE_{width}_{height}."]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this ad unit. Possible values are TEXT, TEXT_IMAGE, IMAGE and LINK."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The backup option to be used in instances where no ad is available."]
pub struct AdUnitContentAdsSettingsBackupOption {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Color to use when type is set to COLOR. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
#[doc = "Settings specific to WAP mobile content ads (AFMC - deprecated)."]
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
    #[doc = "Kind of list this is, in this case adsensehost#adUnits."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod ad_units_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#adUnits")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssociationSession {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hosted account id of the associated publisher after association. Present if status is ACCEPTED."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this association session."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "association_session_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsensehost#associationSession."]
    pub kind: ::std::string::String,
    #[serde(rename = "productCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated)"]
    pub product_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "redirectUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Redirect URL of this association session. Used to redirect users into the AdSense association flow."]
    pub redirect_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userLocale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The preferred locale of the user themselves when going through the AdSense association flow."]
    pub user_locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "websiteLocale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locale of the user's hosted website."]
    pub website_locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "websiteUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the user's hosted website."]
    pub website_url: ::std::option::Option<::std::string::String>,
}
mod association_session_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#associationSession")
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
    #[doc = "Kind of resource this is, in this case adsensehost#customChannel."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of this custom channel."]
    pub name: ::std::option::Option<::std::string::String>,
}
mod custom_channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#customChannel")
    }
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
    #[doc = "Kind of list this is, in this case adsensehost#customChannels."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through custom channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod custom_channels_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#customChannels")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    #[serde(rename = "averages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
    pub averages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."]
    pub headers: ::std::option::Option<::std::vec::Vec<ReportHeaders>>,
    #[serde(rename = "kind")]
    #[serde(default = "report_defaults :: kind")]
    #[doc = "Kind this is, in this case adsensehost#report."]
    pub kind: ::std::string::String,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
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
mod report_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#report")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportHeaders {
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
pub struct UrlChannel {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "url_channel_defaults :: kind")]
    #[doc = "Kind of resource this is, in this case adsensehost#urlChannel."]
    pub kind: ::std::string::String,
    #[serde(rename = "urlPattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL Pattern of this URL channel. Does not include \"http://\" or \"https://\". Example: www.example.com/home"]
    pub url_pattern: ::std::option::Option<::std::string::String>,
}
mod url_channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#urlChannel")
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
    #[doc = "Kind of list this is, in this case adsensehost#urlChannels."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod url_channels_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adsensehost#urlChannels")
    }
}
