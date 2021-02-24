#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
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
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod sitemaps {
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
                    #[serde(rename = "sitemapIndex")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A URL of a site's sitemap index. For example: http://www.example.com/sitemapindex.xml"]
                    pub sitemap_index: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
    pub struct ApiDataRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clicks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub clicks: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ctr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ctr: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub impressions: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<::std::primitive::f64>,
    }
    impl ApiDataRow {
        pub fn builder() -> ApiDataRowBuilder {
            ApiDataRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ApiDimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dimension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub operator: ::std::option::Option<::std::string::String>,
    }
    impl ApiDimensionFilter {
        pub fn builder() -> ApiDimensionFilterBuilder {
            ApiDimensionFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ApiDimensionFilterGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiDimensionFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group_type: ::std::option::Option<::std::string::String>,
    }
    impl ApiDimensionFilterGroup {
        pub fn builder() -> ApiDimensionFilterGroupBuilder {
            ApiDimensionFilterGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SearchAnalyticsQueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is \"auto\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see  the help documentation to learn how data is calculated differently by site versus by page.\n\nNote: If you group or filter by page, you cannot aggregate by property.\n\nIf you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid."]
        pub aggregation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If \"all\" (case-insensitive), data will include fresh data. If \"final\" (case-insensitive) or if this parameter is omitted, the returned data will include only finalized data."]
        pub data_state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilterGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains \"buy\"' to see only data where the query string contains the substring \"buy\" (not case-sensitive). You can filter by a dimension without grouping by it."]
        pub dimension_filter_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiDimensionFilterGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 5,000 (inclusive)."]
        pub row_limit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is \"web\"] The search type to filter for."]
        pub search_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startRow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number."]
        pub start_row: ::std::option::Option<::std::primitive::i64>,
    }
    impl SearchAnalyticsQueryRequest {
        pub fn builder() -> SearchAnalyticsQueryRequestBuilder {
            SearchAnalyticsQueryRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of rows, one per result, grouped by key. Metrics in each row are aggregated for all data grouped by that key either by page or property, as specified by the aggregation type parameter."]
    pub struct SearchAnalyticsQueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseAggregationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the results were aggregated."]
        pub response_aggregation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of rows grouped by the key values in the order given in the query."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiDataRow>>>,
    }
    impl SearchAnalyticsQueryResponse {
        pub fn builder() -> SearchAnalyticsQueryResponseBuilder {
            SearchAnalyticsQueryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of sitemaps."]
    pub struct SitemapsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sitemap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains detailed information about a specific URL submitted as a sitemap."]
        pub sitemap: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WmxSitemap>>>,
    }
    impl SitemapsListResponse {
        pub fn builder() -> SitemapsListResponseBuilder {
            SitemapsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of sites with access level information."]
    pub struct SitesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteEntry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains permission level information about a Search Console site. For more information, see Permissions in Search Console."]
        pub site_entry: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WmxSite>>>,
    }
    impl SitesListResponse {
        pub fn builder() -> SitesListResponseBuilder {
            SitesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains permission level information about a Search Console site. For more information, see  Permissions in Search Console."]
    pub struct WmxSite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's permission level for the site."]
        pub permission_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the site."]
        pub site_url: ::std::option::Option<::std::string::String>,
    }
    impl WmxSite {
        pub fn builder() -> WmxSiteBuilder {
            WmxSiteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains detailed information about a specific URL submitted as a sitemap."]
    pub struct WmxSitemap {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The various content types in the sitemap."]
        pub contents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WmxSitemapContent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of errors in the sitemap. These are issues with the sitemap itself that need to be fixed before it can be processed correctly."]
        pub errors: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the sitemap has not been processed."]
        pub is_pending: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSitemapsIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the sitemap is a collection of sitemaps."]
        pub is_sitemaps_index: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastDownloaded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date & time in which this sitemap was last downloaded. Date format is in RFC 3339 format (yyyy-mm-dd)."]
        pub last_downloaded: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSubmitted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date & time in which this sitemap was submitted. Date format is in RFC 3339 format (yyyy-mm-dd)."]
        pub last_submitted: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url of the sitemap."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the sitemap. For example: rssFeed."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of warnings for the sitemap. These are generally non-critical issues with URLs in the sitemaps."]
        pub warnings: ::std::option::Option<::std::string::String>,
    }
    impl WmxSitemap {
        pub fn builder() -> WmxSitemapBuilder {
            WmxSitemapBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the various content types in the sitemap."]
    pub struct WmxSitemapContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of URLs from the sitemap that were indexed (of the content type)."]
        pub indexed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submitted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of URLs in the sitemap (of the content type)."]
        pub submitted: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specific type of content in this sitemap. For example: web."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl WmxSitemapContent {
        pub fn builder() -> WmxSitemapContentBuilder {
            WmxSitemapContentBuilder::default()
        }
    }
}
