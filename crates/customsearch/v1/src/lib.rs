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
        serde_json::from_str(&"\"json\"").unwrap()
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
    pub mod cse {
        pub mod methods {
            pub mod list {
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
                    #[serde(rename = "c2coff")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)"]
                    pub c2coff: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "cr")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections) page for a list of valid values for this parameter."]
                    pub cr: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "cx")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The Programmable Search Engine ID to use for this request."]
                    pub cx: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "dateRestrict")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
                    pub date_restrict: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "exactTerms")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Identifies a phrase that all documents in the search results must contain."]
                    pub exact_terms: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "excludeTerms")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
                    pub exclude_terms: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fileType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287)."]
                    pub file_type: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "gl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States."]
                    pub gl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "googlehost")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search."]
                    pub googlehost: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "highRange")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                    pub high_range: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and (Supported Interface Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages] for a list of supported languages."]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hq")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Appends the specified query terms to the query, as if they were combined with a logical AND operator."]
                    pub hq: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "imgColorType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
                    pub img_color_type: ::std::option::Option<QueryParametersImgColorTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "imgDominantColor")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
                    pub img_dominant_color:
                        ::std::option::Option<QueryParametersImgDominantColorEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "imgSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
                    pub img_size: ::std::option::Option<QueryParametersImgSizeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "imgType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
                    pub img_type: ::std::option::Option<QueryParametersImgTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "linkSite")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies that all search results should contain a link to a particular URL."]
                    pub link_site: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "lowRange")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                    pub low_range: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "lr")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `\"lang_ar\"`: Arabic * `\"lang_bg\"`: Bulgarian * `\"lang_ca\"`: Catalan * `\"lang_cs\"`: Czech * `\"lang_da\"`: Danish * `\"lang_de\"`: German * `\"lang_el\"`: Greek * `\"lang_en\"`: English * `\"lang_es\"`: Spanish * `\"lang_et\"`: Estonian * `\"lang_fi\"`: Finnish * `\"lang_fr\"`: French * `\"lang_hr\"`: Croatian * `\"lang_hu\"`: Hungarian * `\"lang_id\"`: Indonesian * `\"lang_is\"`: Icelandic * `\"lang_it\"`: Italian * `\"lang_iw\"`: Hebrew * `\"lang_ja\"`: Japanese * `\"lang_ko\"`: Korean * `\"lang_lt\"`: Lithuanian * `\"lang_lv\"`: Latvian * `\"lang_nl\"`: Dutch * `\"lang_no\"`: Norwegian * `\"lang_pl\"`: Polish * `\"lang_pt\"`: Portuguese * `\"lang_ro\"`: Romanian * `\"lang_ru\"`: Russian * `\"lang_sk\"`: Slovak * `\"lang_sl\"`: Slovenian * `\"lang_sr\"`: Serbian * `\"lang_sv\"`: Swedish * `\"lang_tr\"`: Turkish * `\"lang_zh-CN\"`: Chinese (Simplified) * `\"lang_zh-TW\"`: Chinese (Traditional)"]
                    pub lr: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "num")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Number of search results to return. * Valid values are integers between 1 and 10, inclusive."]
                    pub num: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orTerms")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms."]
                    pub or_terms: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query"]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "relatedSite")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies that all search results should be pages that are related to the specified URL."]
                    pub related_site: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "rights")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
                    pub rights: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "safe")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
                    pub safe: ::std::option::Option<QueryParametersSafeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "searchType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
                    pub search_type: ::std::option::Option<QueryParametersSearchTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "siteSearch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below)."]
                    pub site_search: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "siteSearchFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
                    pub site_search_filter:
                        ::std::option::Option<QueryParametersSiteSearchFilterEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sort")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute)."]
                    pub sort: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "start")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10."]
                    pub start: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
                pub enum QueryParametersImgColorTypeEnum {
                    #[serde(rename = "imgColorTypeUndefined")]
                    #[doc = "No image color type specified."]
                    ImgColorTypeUndefined,
                    #[serde(rename = "mono")]
                    #[doc = "Black and white images only."]
                    Mono,
                    #[serde(rename = "gray")]
                    #[doc = "Grayscale images only."]
                    Gray,
                    #[serde(rename = "color")]
                    #[doc = "Color images only."]
                    Color,
                    #[serde(rename = "trans")]
                    #[doc = "Images with transparent background"]
                    Trans,
                }
                impl ::std::default::Default for QueryParametersImgColorTypeEnum {
                    fn default() -> Self {
                        Self::ImgColorTypeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
                pub enum QueryParametersImgDominantColorEnum {
                    #[serde(rename = "imgDominantColorUndefined")]
                    #[doc = "No dominant color specified."]
                    ImgDominantColorUndefined,
                    #[serde(rename = "black")]
                    #[doc = "Predominantly black images only."]
                    Black,
                    #[serde(rename = "blue")]
                    #[doc = "Predominantly blue images only."]
                    Blue,
                    #[serde(rename = "brown")]
                    #[doc = "Predominantly brown images only."]
                    Brown,
                    #[serde(rename = "gray")]
                    #[doc = "Predominantly gray images only."]
                    Gray,
                    #[serde(rename = "green")]
                    #[doc = "Predominantly green images only."]
                    Green,
                    #[serde(rename = "orange")]
                    #[doc = "Predominantly orange images only."]
                    Orange,
                    #[serde(rename = "pink")]
                    #[doc = "Predominantly pink images only."]
                    Pink,
                    #[serde(rename = "purple")]
                    #[doc = "Predominantly purple images only."]
                    Purple,
                    #[serde(rename = "red")]
                    #[doc = "Predominantly red images only."]
                    Red,
                    #[serde(rename = "teal")]
                    #[doc = "Predominantly teal images only."]
                    Teal,
                    #[serde(rename = "white")]
                    #[doc = "Predominantly white images only."]
                    White,
                    #[serde(rename = "yellow")]
                    #[doc = "Predominantly yellow images only."]
                    Yellow,
                }
                impl ::std::default::Default for QueryParametersImgDominantColorEnum {
                    fn default() -> Self {
                        Self::ImgDominantColorUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
                pub enum QueryParametersImgSizeEnum {
                    #[serde(rename = "imgSizeUndefined")]
                    #[doc = "No image size specified."]
                    ImgSizeUndefined,
                    #[serde(rename = "HUGE")]
                    #[doc = "Only the largest possible images."]
                    Huge,
                    #[serde(rename = "ICON")]
                    #[doc = "Only very small icon-sized images."]
                    Icon,
                    #[serde(rename = "LARGE")]
                    #[doc = "Only large images."]
                    Large,
                    #[serde(rename = "MEDIUM")]
                    #[doc = "Only medium images."]
                    Medium,
                    #[serde(rename = "SMALL")]
                    #[doc = "Only small images."]
                    Small,
                    #[serde(rename = "XLARGE")]
                    #[doc = "Only very large images."]
                    Xlarge,
                    #[serde(rename = "XXLARGE")]
                    #[doc = "Only extremely large images."]
                    Xxlarge,
                }
                impl ::std::default::Default for QueryParametersImgSizeEnum {
                    fn default() -> Self {
                        Self::ImgSizeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
                pub enum QueryParametersImgTypeEnum {
                    #[serde(rename = "imgTypeUndefined")]
                    #[doc = "No image type specified."]
                    ImgTypeUndefined,
                    #[serde(rename = "clipart")]
                    #[doc = "Clipart-style images only."]
                    Clipart,
                    #[serde(rename = "face")]
                    #[doc = "Images of faces only."]
                    Face,
                    #[serde(rename = "lineart")]
                    #[doc = "Line art images only."]
                    Lineart,
                    #[serde(rename = "stock")]
                    #[doc = "Stock images only."]
                    Stock,
                    #[serde(rename = "photo")]
                    #[doc = "Photo images only."]
                    Photo,
                    #[serde(rename = "animated")]
                    #[doc = "Animated images only."]
                    Animated,
                }
                impl ::std::default::Default for QueryParametersImgTypeEnum {
                    fn default() -> Self {
                        Self::ImgTypeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
                pub enum QueryParametersSafeEnum {
                    #[serde(rename = "safeUndefined")]
                    #[doc = "SafeSearch mode unspecified. (Falls back to engine's configuration.)"]
                    SafeUndefined,
                    #[serde(rename = "active")]
                    #[doc = "Turn SafeSearch on."]
                    Active,
                    #[serde(rename = "high")]
                    #[doc = "Deprecated, equivalent to \"active\"."]
                    High,
                    #[serde(rename = "medium")]
                    #[doc = "Deprecated, equivalent to \"active\"."]
                    Medium,
                    #[serde(rename = "off")]
                    #[doc = "Turn SafeSearch off."]
                    Off,
                }
                impl ::std::default::Default for QueryParametersSafeEnum {
                    fn default() -> Self {
                        Self::SafeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
                pub enum QueryParametersSearchTypeEnum {
                    #[serde(rename = "searchTypeUndefined")]
                    #[doc = "Search type unspecified (defaults to web search)."]
                    SearchTypeUndefined,
                    #[serde(rename = "image")]
                    #[doc = "Image search."]
                    Image,
                }
                impl ::std::default::Default for QueryParametersSearchTypeEnum {
                    fn default() -> Self {
                        Self::SearchTypeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
                pub enum QueryParametersSiteSearchFilterEnum {
                    #[serde(rename = "siteSearchFilterUndefined")]
                    #[doc = "Filter mode unspecified."]
                    SiteSearchFilterUndefined,
                    #[serde(rename = "e")]
                    #[doc = "Exclude results from the listed sites."]
                    E,
                    #[serde(rename = "i")]
                    #[doc = "Include only results from the listed sites."]
                    I,
                }
                impl ::std::default::Default for QueryParametersSiteSearchFilterEnum {
                    fn default() -> Self {
                        Self::SiteSearchFilterUndefined
                    }
                }
            }
        }
        pub mod resources {
            pub mod siterestrict {
                pub mod methods {
                    pub mod list {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "c2coff")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)"]
                            pub c2coff: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "cr")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections) page for a list of valid values for this parameter."]
                            pub cr: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "cx")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The Programmable Search Engine ID to use for this request."]
                            pub cx: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "dateRestrict")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
                            pub date_restrict: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "exactTerms")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Identifies a phrase that all documents in the search results must contain."]
                            pub exact_terms: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "excludeTerms")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
                            pub exclude_terms: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "fileType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287)."]
                            pub file_type: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "gl")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States."]
                            pub gl: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "googlehost")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search."]
                            pub googlehost: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "highRange")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                            pub high_range: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "hl")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and (Supported Interface Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages] for a list of supported languages."]
                            pub hl: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "hq")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Appends the specified query terms to the query, as if they were combined with a logical AND operator."]
                            pub hq: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "imgColorType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
                            pub img_color_type:
                                ::std::option::Option<QueryParametersImgColorTypeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "imgDominantColor")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
                            pub img_dominant_color:
                                ::std::option::Option<QueryParametersImgDominantColorEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "imgSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
                            pub img_size: ::std::option::Option<QueryParametersImgSizeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "imgType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
                            pub img_type: ::std::option::Option<QueryParametersImgTypeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "linkSite")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies that all search results should contain a link to a particular URL."]
                            pub link_site: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "lowRange")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                            pub low_range: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "lr")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `\"lang_ar\"`: Arabic * `\"lang_bg\"`: Bulgarian * `\"lang_ca\"`: Catalan * `\"lang_cs\"`: Czech * `\"lang_da\"`: Danish * `\"lang_de\"`: German * `\"lang_el\"`: Greek * `\"lang_en\"`: English * `\"lang_es\"`: Spanish * `\"lang_et\"`: Estonian * `\"lang_fi\"`: Finnish * `\"lang_fr\"`: French * `\"lang_hr\"`: Croatian * `\"lang_hu\"`: Hungarian * `\"lang_id\"`: Indonesian * `\"lang_is\"`: Icelandic * `\"lang_it\"`: Italian * `\"lang_iw\"`: Hebrew * `\"lang_ja\"`: Japanese * `\"lang_ko\"`: Korean * `\"lang_lt\"`: Lithuanian * `\"lang_lv\"`: Latvian * `\"lang_nl\"`: Dutch * `\"lang_no\"`: Norwegian * `\"lang_pl\"`: Polish * `\"lang_pt\"`: Portuguese * `\"lang_ro\"`: Romanian * `\"lang_ru\"`: Russian * `\"lang_sk\"`: Slovak * `\"lang_sl\"`: Slovenian * `\"lang_sr\"`: Serbian * `\"lang_sv\"`: Swedish * `\"lang_tr\"`: Turkish * `\"lang_zh-CN\"`: Chinese (Simplified) * `\"lang_zh-TW\"`: Chinese (Traditional)"]
                            pub lr: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "num")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Number of search results to return. * Valid values are integers between 1 and 10, inclusive."]
                            pub num: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orTerms")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms."]
                            pub or_terms: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "q")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Query"]
                            pub q: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "relatedSite")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies that all search results should be pages that are related to the specified URL."]
                            pub related_site: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "rights")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
                            pub rights: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "safe")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
                            pub safe: ::std::option::Option<QueryParametersSafeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "searchType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
                            pub search_type: ::std::option::Option<QueryParametersSearchTypeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "siteSearch")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below)."]
                            pub site_search: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "siteSearchFilter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
                            pub site_search_filter:
                                ::std::option::Option<QueryParametersSiteSearchFilterEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute)."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10."]
                            pub start: ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
                        pub enum QueryParametersImgColorTypeEnum {
                            #[serde(rename = "imgColorTypeUndefined")]
                            #[doc = "No image color type specified."]
                            ImgColorTypeUndefined,
                            #[serde(rename = "mono")]
                            #[doc = "Black and white images only."]
                            Mono,
                            #[serde(rename = "gray")]
                            #[doc = "Grayscale images only."]
                            Gray,
                            #[serde(rename = "color")]
                            #[doc = "Color images only."]
                            Color,
                            #[serde(rename = "trans")]
                            #[doc = "Images with transparent background"]
                            Trans,
                        }
                        impl ::std::default::Default for QueryParametersImgColorTypeEnum {
                            fn default() -> Self {
                                Self::ImgColorTypeUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
                        pub enum QueryParametersImgDominantColorEnum {
                            #[serde(rename = "imgDominantColorUndefined")]
                            #[doc = "No dominant color specified."]
                            ImgDominantColorUndefined,
                            #[serde(rename = "black")]
                            #[doc = "Predominantly black images only."]
                            Black,
                            #[serde(rename = "blue")]
                            #[doc = "Predominantly blue images only."]
                            Blue,
                            #[serde(rename = "brown")]
                            #[doc = "Predominantly brown images only."]
                            Brown,
                            #[serde(rename = "gray")]
                            #[doc = "Predominantly gray images only."]
                            Gray,
                            #[serde(rename = "green")]
                            #[doc = "Predominantly green images only."]
                            Green,
                            #[serde(rename = "orange")]
                            #[doc = "Predominantly orange images only."]
                            Orange,
                            #[serde(rename = "pink")]
                            #[doc = "Predominantly pink images only."]
                            Pink,
                            #[serde(rename = "purple")]
                            #[doc = "Predominantly purple images only."]
                            Purple,
                            #[serde(rename = "red")]
                            #[doc = "Predominantly red images only."]
                            Red,
                            #[serde(rename = "teal")]
                            #[doc = "Predominantly teal images only."]
                            Teal,
                            #[serde(rename = "white")]
                            #[doc = "Predominantly white images only."]
                            White,
                            #[serde(rename = "yellow")]
                            #[doc = "Predominantly yellow images only."]
                            Yellow,
                        }
                        impl ::std::default::Default for QueryParametersImgDominantColorEnum {
                            fn default() -> Self {
                                Self::ImgDominantColorUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
                        pub enum QueryParametersImgSizeEnum {
                            #[serde(rename = "imgSizeUndefined")]
                            #[doc = "No image size specified."]
                            ImgSizeUndefined,
                            #[serde(rename = "HUGE")]
                            #[doc = "Only the largest possible images."]
                            Huge,
                            #[serde(rename = "ICON")]
                            #[doc = "Only very small icon-sized images."]
                            Icon,
                            #[serde(rename = "LARGE")]
                            #[doc = "Only large images."]
                            Large,
                            #[serde(rename = "MEDIUM")]
                            #[doc = "Only medium images."]
                            Medium,
                            #[serde(rename = "SMALL")]
                            #[doc = "Only small images."]
                            Small,
                            #[serde(rename = "XLARGE")]
                            #[doc = "Only very large images."]
                            Xlarge,
                            #[serde(rename = "XXLARGE")]
                            #[doc = "Only extremely large images."]
                            Xxlarge,
                        }
                        impl ::std::default::Default for QueryParametersImgSizeEnum {
                            fn default() -> Self {
                                Self::ImgSizeUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
                        pub enum QueryParametersImgTypeEnum {
                            #[serde(rename = "imgTypeUndefined")]
                            #[doc = "No image type specified."]
                            ImgTypeUndefined,
                            #[serde(rename = "clipart")]
                            #[doc = "Clipart-style images only."]
                            Clipart,
                            #[serde(rename = "face")]
                            #[doc = "Images of faces only."]
                            Face,
                            #[serde(rename = "lineart")]
                            #[doc = "Line art images only."]
                            Lineart,
                            #[serde(rename = "stock")]
                            #[doc = "Stock images only."]
                            Stock,
                            #[serde(rename = "photo")]
                            #[doc = "Photo images only."]
                            Photo,
                            #[serde(rename = "animated")]
                            #[doc = "Animated images only."]
                            Animated,
                        }
                        impl ::std::default::Default for QueryParametersImgTypeEnum {
                            fn default() -> Self {
                                Self::ImgTypeUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
                        pub enum QueryParametersSafeEnum {
                            #[serde(rename = "safeUndefined")]
                            #[doc = "SafeSearch mode unspecified. (Falls back to engine's configuration.)"]
                            SafeUndefined,
                            #[serde(rename = "active")]
                            #[doc = "Turn SafeSearch on."]
                            Active,
                            #[serde(rename = "high")]
                            #[doc = "Deprecated, equivalent to \"active\"."]
                            High,
                            #[serde(rename = "medium")]
                            #[doc = "Deprecated, equivalent to \"active\"."]
                            Medium,
                            #[serde(rename = "off")]
                            #[doc = "Turn SafeSearch off."]
                            Off,
                        }
                        impl ::std::default::Default for QueryParametersSafeEnum {
                            fn default() -> Self {
                                Self::SafeUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
                        pub enum QueryParametersSearchTypeEnum {
                            #[serde(rename = "searchTypeUndefined")]
                            #[doc = "Search type unspecified (defaults to web search)."]
                            SearchTypeUndefined,
                            #[serde(rename = "image")]
                            #[doc = "Image search."]
                            Image,
                        }
                        impl ::std::default::Default for QueryParametersSearchTypeEnum {
                            fn default() -> Self {
                                Self::SearchTypeUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
                        pub enum QueryParametersSiteSearchFilterEnum {
                            #[serde(rename = "siteSearchFilterUndefined")]
                            #[doc = "Filter mode unspecified."]
                            SiteSearchFilterUndefined,
                            #[serde(rename = "e")]
                            #[doc = "Exclude results from the listed sites."]
                            E,
                            #[serde(rename = "i")]
                            #[doc = "Include only results from the listed sites."]
                            I,
                        }
                        impl ::std::default::Default for QueryParametersSiteSearchFilterEnum {
                            fn default() -> Self {
                                Self::SiteSearchFilterUndefined
                            }
                        }
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
    #[doc = "Promotion result."]
    pub struct Promotion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bodyLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of block objects for this promotion. See [Google WebSearch Protocol reference](https://developers.google.com/custom-search/docs/xml_results) for more information."]
        pub body_lines: ::std::option::Option<::std::vec::Vec<PromotionBodyLines>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An abridged version of this search's result URL, e.g. www.example.com."]
        pub display_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the promotion, in HTML."]
        pub html_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image belonging to a promotion."]
        pub image: ::std::option::Option<PromotionImage>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the promotion."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the promotion."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Promotion {
        pub fn builder() -> PromotionBuilder {
            PromotionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Block object belonging to a promotion."]
    pub struct PromotionBodyLines {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The block object's text in HTML, if it has text."]
        pub html_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The anchor text of the block object's link, if it has a link."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The block object's text, if it has text."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the block object's link, if it has one."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PromotionBodyLines {
        pub fn builder() -> PromotionBodyLinesBuilder {
            PromotionBodyLinesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image belonging to a promotion."]
    pub struct PromotionImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image height in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the image for this promotion link."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image width in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl PromotionImage {
        pub fn builder() -> PromotionImageBuilder {
            PromotionImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom search result."]
    pub struct Result {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the ID of Google's cached version of the search result."]
        pub cache_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An abridged version of this search result’s URL, e.g. www.example.com."]
        pub display_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file format of the search result."]
        pub file_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL displayed after the snippet for each search result."]
        pub formatted_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlFormattedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTML-formatted URL displayed after the snippet for each search result."]
        pub html_formatted_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlSnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet of the search result, in HTML."]
        pub html_snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the search result, in HTML."]
        pub html_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image belonging to a custom search result."]
        pub image: ::std::option::Option<ResultImage>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier for the type of current object. For this API, it is `customsearch#result.`"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encapsulates all information about [refinement labels](https://developers.google.com/custom-search/docs/xml_results)."]
        pub labels: ::std::option::Option<::std::vec::Vec<ResultLabels>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full URL to which the search result is pointing, e.g. http://www.example.com/foo/bar."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the search result."]
        pub mime: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagemap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains [PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps) information for this search result."]
        pub pagemap:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet of the search result, in plain text."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the search result, in plain text."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Result {
        pub fn builder() -> ResultBuilder {
            ResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image belonging to a custom search result."]
    pub struct ResultImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the image, in pixels."]
        pub byte_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL pointing to the webpage hosting the image."]
        pub context_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the image, in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the thumbnail image, in pixels."]
        pub thumbnail_height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL to the thumbnail image."]
        pub thumbnail_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the thumbnail image, in pixels."]
        pub thumbnail_width: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the image, in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl ResultImage {
        pub fn builder() -> ResultImageBuilder {
            ResultImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Refinement label associated with a custom search result."]
    pub struct ResultLabels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of a refinement label. This is the name you should display in your user interface."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label_with_op")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refinement label and the associated refinement operation."]
        pub label_with_op: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of a refinement label, which you can use to refine searches. Don't display this in your user interface; instead, use displayName."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ResultLabels {
        pub fn builder() -> ResultLabelsBuilder {
            ResultLabelsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a custom search request."]
    pub struct Search {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search."]
        pub context:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current set of custom search results."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Result>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the type of current object. For this API, it is customsearch#search."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine's configuration files define any promotions for the given query."]
        pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query metadata for the previous, current, and next pages of results."]
        pub queries: ::std::option::Option<SearchQueries>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchInformation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about a search operation."]
        pub search_information: ::std::option::Option<SearchSearchInformation>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spelling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spell correction information for a query."]
        pub spelling: ::std::option::Option<SearchSpelling>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OpenSearch template and URL."]
        pub url: ::std::option::Option<SearchUrl>,
    }
    impl Search {
        pub fn builder() -> SearchBuilder {
            SearchBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query metadata for the previous, current, and next pages of results."]
    pub struct SearchQueries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata representing the next page of results, if applicable."]
        pub next_page: ::std::option::Option<::std::vec::Vec<SearchQueriesNextPage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata representing the previous page of results, if applicable."]
        pub previous_page: ::std::option::Option<::std::vec::Vec<SearchQueriesPreviousPage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "request")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata representing the current request."]
        pub request: ::std::option::Option<::std::vec::Vec<SearchQueriesRequest>>,
    }
    impl SearchQueries {
        pub fn builder() -> SearchQueriesBuilder {
            SearchQueriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom search request metadata."]
    pub struct SearchQueriesNextPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of search results returned in this set."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
        pub cr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        pub cx: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRestrict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        pub date_restrict: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableCnTwTranslation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exactTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        pub exact_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        pub exclude_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        pub file_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        pub gl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleHost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        pub google_host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub high_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
        pub hl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hq")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        pub hq: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgColorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        pub img_color_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgDominantColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        pub img_dominant_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        pub img_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        pub img_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search requests."]
        pub input_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the search results."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        pub link_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub low_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
        pub or_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search results."]
        pub output_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        pub related_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        pub rights: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        pub safe: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search terms entered by the user."]
        pub search_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        pub search_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs from a specified site."]
        pub site_search: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        pub site_search_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        pub sort: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        pub start_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the query."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated number of total search results. May not be accurate."]
        pub total_results: ::std::option::Option<::std::string::String>,
    }
    impl SearchQueriesNextPage {
        pub fn builder() -> SearchQueriesNextPageBuilder {
            SearchQueriesNextPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom search request metadata."]
    pub struct SearchQueriesPreviousPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of search results returned in this set."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
        pub cr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        pub cx: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRestrict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        pub date_restrict: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableCnTwTranslation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exactTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        pub exact_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        pub exclude_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        pub file_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        pub gl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleHost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        pub google_host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub high_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
        pub hl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hq")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        pub hq: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgColorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        pub img_color_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgDominantColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        pub img_dominant_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        pub img_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        pub img_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search requests."]
        pub input_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the search results."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        pub link_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub low_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
        pub or_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search results."]
        pub output_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        pub related_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        pub rights: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        pub safe: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search terms entered by the user."]
        pub search_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        pub search_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs from a specified site."]
        pub site_search: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        pub site_search_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        pub sort: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        pub start_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the query."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated number of total search results. May not be accurate."]
        pub total_results: ::std::option::Option<::std::string::String>,
    }
    impl SearchQueriesPreviousPage {
        pub fn builder() -> SearchQueriesPreviousPageBuilder {
            SearchQueriesPreviousPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom search request metadata."]
    pub struct SearchQueriesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of search results returned in this set."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections) for a list of valid values for this parameter."]
        pub cr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        pub cx: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRestrict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        pub date_restrict: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableCnTwTranslation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        pub disable_cn_tw_translation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exactTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        pub exact_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        pub exclude_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        pub file_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        pub gl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleHost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        pub google_host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub high_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages) for a list of supported languages."]
        pub hl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hq")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        pub hq: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgColorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        pub img_color_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgDominantColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        pub img_dominant_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        pub img_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imgType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        pub img_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search requests."]
        pub input_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the search results."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        pub link_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        pub low_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt) query term for this type of query."]
        pub or_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The character encoding supported for search results."]
        pub output_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        pub related_site: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        pub rights: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        pub safe: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search terms entered by the user."]
        pub search_terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        pub search_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts results to URLs from a specified site."]
        pub site_search: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        pub site_search_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        pub sort: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        pub start_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the query."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated number of total search results. May not be accurate."]
        pub total_results: ::std::option::Option<::std::string::String>,
    }
    impl SearchQueriesRequest {
        pub fn builder() -> SearchQueriesRequestBuilder {
            SearchQueriesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a search operation."]
    pub struct SearchSearchInformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedSearchTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time taken for the server to return search results, formatted according to locale style."]
        pub formatted_search_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedTotalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of search results, formatted according to locale style."]
        pub formatted_total_results: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time taken for the server to return search results."]
        pub search_time: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of search results returned by the query."]
        pub total_results: ::std::option::Option<::std::string::String>,
    }
    impl SearchSearchInformation {
        pub fn builder() -> SearchSearchInformationBuilder {
            SearchSearchInformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Spell correction information for a query."]
    pub struct SearchSpelling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "correctedQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corrected query."]
        pub corrected_query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlCorrectedQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corrected query, formatted in HTML."]
        pub html_corrected_query: ::std::option::Option<::std::string::String>,
    }
    impl SearchSpelling {
        pub fn builder() -> SearchSpellingBuilder {
            SearchSpellingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "OpenSearch template and URL."]
    pub struct SearchUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual [OpenSearch template](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax) for this API."]
        pub template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the OpenSearch URL template for the Custom Search JSON API."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl SearchUrl {
        pub fn builder() -> SearchUrlBuilder {
            SearchUrlBuilder::default()
        }
    }
}
