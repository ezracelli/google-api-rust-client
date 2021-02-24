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
        serde_json::from_str(&"json").unwrap()
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
                    #[doc = " A URL of a site's sitemap index. For example: `http://www.example.com/sitemapindex.xml`."]
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
    #[doc = "A filter test to be applied to each row in the data set, where a match can return the row. Filters are string comparisons, and values and dimension names are not case-sensitive. Individual filters are either AND'ed or OR'ed within their parent filter group, according to the group's group type. You do not need to group by a specified dimension to filter against it."]
    pub struct ApiDimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dimension: ::std::option::Option<ApiDimensionFilterDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub operator: ::std::option::Option<ApiDimensionFilterOperatorEnum>,
    }
    impl ApiDimensionFilter {
        pub fn builder() -> ApiDimensionFilterBuilder {
            ApiDimensionFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ApiDimensionFilterDimensionEnum {
        #[serde(rename = "QUERY")]
        #[doc = ""]
        Query,
        #[serde(rename = "PAGE")]
        #[doc = ""]
        Page,
        #[serde(rename = "COUNTRY")]
        #[doc = ""]
        Country,
        #[serde(rename = "DEVICE")]
        #[doc = ""]
        Device,
        #[serde(rename = "SEARCH_APPEARANCE")]
        #[doc = ""]
        SearchAppearance,
    }
    impl ::std::default::Default for ApiDimensionFilterDimensionEnum {
        fn default() -> Self {
            Self::Query
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ApiDimensionFilterOperatorEnum {
        #[serde(rename = "EQUALS")]
        #[doc = ""]
        Equals,
        #[serde(rename = "NOT_EQUALS")]
        #[doc = ""]
        NotEquals,
        #[serde(rename = "CONTAINS")]
        #[doc = ""]
        Contains,
        #[serde(rename = "NOT_CONTAINS")]
        #[doc = ""]
        NotContains,
    }
    impl ::std::default::Default for ApiDimensionFilterOperatorEnum {
        fn default() -> Self {
            Self::Equals
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of dimension value filters to test against each row. Only rows that pass all filter groups will be returned. All results within a filter group are either AND'ed or OR'ed together, depending on the group type selected. All filter groups are AND'ed together."]
    pub struct ApiDimensionFilterGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiDimensionFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group_type: ::std::option::Option<ApiDimensionFilterGroupGroupTypeEnum>,
    }
    impl ApiDimensionFilterGroup {
        pub fn builder() -> ApiDimensionFilterGroupBuilder {
            ApiDimensionFilterGroupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ApiDimensionFilterGroupGroupTypeEnum {
        #[serde(rename = "AND")]
        #[doc = ""]
        And,
    }
    impl ::std::default::Default for ApiDimensionFilterGroupGroupTypeEnum {
        fn default() -> Self {
            Self::And
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Blocked resource."]
    pub struct BlockedResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the blocked resource."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl BlockedResource {
        pub fn builder() -> BlockedResourceBuilder {
            BlockedResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describe image data."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image data in format determined by the mime type. Currently, the format will always be \"image/png\", but this might change in the future."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mime-type of the image data."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mobile-friendly issue."]
    pub struct MobileFriendlyIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rule violated."]
        pub rule: ::std::option::Option<MobileFriendlyIssueRuleEnum>,
    }
    impl MobileFriendlyIssue {
        pub fn builder() -> MobileFriendlyIssueBuilder {
            MobileFriendlyIssueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rule violated."]
    pub enum MobileFriendlyIssueRuleEnum {
        #[serde(rename = "MOBILE_FRIENDLY_RULE_UNSPECIFIED")]
        #[doc = "Unknown rule. Sorry, we don't have any description for the rule that was broken."]
        MobileFriendlyRuleUnspecified,
        #[serde(rename = "USES_INCOMPATIBLE_PLUGINS")]
        #[doc = "Plugins incompatible with mobile devices are being used. [Learn more] (https://support.google.com/webmasters/answer/6352293#flash_usage)."]
        UsesIncompatiblePlugins,
        #[serde(rename = "CONFIGURE_VIEWPORT")]
        #[doc = "Viewsport is not specified using the meta viewport tag. [Learn more] (https://support.google.com/webmasters/answer/6352293#viewport_not_configured)."]
        ConfigureViewport,
        #[serde(rename = "FIXED_WIDTH_VIEWPORT")]
        #[doc = "Viewport defined to a fixed width. [Learn more] (https://support.google.com/webmasters/answer/6352293#fixed-width_viewport)."]
        FixedWidthViewport,
        #[serde(rename = "SIZE_CONTENT_TO_VIEWPORT")]
        #[doc = "Content not sized to viewport. [Learn more] (https://support.google.com/webmasters/answer/6352293#content_not_sized_to_viewport)."]
        SizeContentToViewport,
        #[serde(rename = "USE_LEGIBLE_FONT_SIZES")]
        #[doc = "Font size is too small for easy reading on a small screen. [Learn More] (https://support.google.com/webmasters/answer/6352293#small_font_size)."]
        UseLegibleFontSizes,
        #[serde(rename = "TAP_TARGETS_TOO_CLOSE")]
        #[doc = "Touch elements are too close to each other. [Learn more] (https://support.google.com/webmasters/answer/6352293#touch_elements_too_close)."]
        TapTargetsTooClose,
    }
    impl ::std::default::Default for MobileFriendlyIssueRuleEnum {
        fn default() -> Self {
            Self::MobileFriendlyRuleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a resource with issue."]
    pub struct ResourceIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockedResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes a blocked resource issue."]
        pub blocked_resource: ::std::option::Option<::std::boxed::Box<BlockedResource>>,
    }
    impl ResourceIssue {
        pub fn builder() -> ResourceIssueBuilder {
            ResourceIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mobile-friendly test request."]
    pub struct RunMobileFriendlyTestRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestScreenshot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not screenshot is requested. Default is false."]
        pub request_screenshot: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for inspection."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl RunMobileFriendlyTestRequest {
        pub fn builder() -> RunMobileFriendlyTestRequestBuilder {
            RunMobileFriendlyTestRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mobile-friendly test response, including mobile-friendly issues and resource issues."]
    pub struct RunMobileFriendlyTestResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileFriendliness")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Test verdict, whether the page is mobile friendly or not."]
        pub mobile_friendliness:
            ::std::option::Option<RunMobileFriendlyTestResponseMobileFriendlinessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileFriendlyIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of mobile-usability issues."]
        pub mobile_friendly_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileFriendlyIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about embedded resources issues."]
        pub resource_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenshot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Screenshot of the requested URL."]
        pub screenshot: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Final state of the test, can be either complete or an error."]
        pub test_status: ::std::option::Option<::std::boxed::Box<TestStatus>>,
    }
    impl RunMobileFriendlyTestResponse {
        pub fn builder() -> RunMobileFriendlyTestResponseBuilder {
            RunMobileFriendlyTestResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Test verdict, whether the page is mobile friendly or not."]
    pub enum RunMobileFriendlyTestResponseMobileFriendlinessEnum {
        #[serde(rename = "MOBILE_FRIENDLY_TEST_RESULT_UNSPECIFIED")]
        #[doc = "Internal error when running this test. Please try running the test again."]
        MobileFriendlyTestResultUnspecified,
        #[serde(rename = "MOBILE_FRIENDLY")]
        #[doc = "The page is mobile friendly."]
        MobileFriendly,
        #[serde(rename = "NOT_MOBILE_FRIENDLY")]
        #[doc = "The page is not mobile friendly."]
        NotMobileFriendly,
    }
    impl ::std::default::Default for RunMobileFriendlyTestResponseMobileFriendlinessEnum {
        fn default() -> Self {
            Self::MobileFriendlyTestResultUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SearchAnalyticsQueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is \\\"auto\\\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid."]
        pub aggregation_type: ::std::option::Option<SearchAnalyticsQueryRequestAggregationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data state to be fetched, can be full or all, the latter including full and partial data."]
        pub data_state: ::std::option::Option<SearchAnalyticsQueryRequestDataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilterGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains \\\"buy\\\"' to see only data where the query string contains the substring \\\"buy\\\" (not case-sensitive). You can filter by a dimension without grouping by it."]
        pub dimension_filter_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiDimensionFilterGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions."]
        pub dimensions:
            ::std::option::Option<::std::vec::Vec<SearchAnalyticsQueryRequestDimensionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 25,000 (inclusive)."]
        pub row_limit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional; Default is \\\"web\\\"] The search type to filter for."]
        pub search_type: ::std::option::Option<SearchAnalyticsQueryRequestSearchTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " [Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range."]
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
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Optional; Default is \\\"auto\\\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid."]
    pub enum SearchAnalyticsQueryRequestAggregationTypeEnum {
        #[serde(rename = "AUTO")]
        #[doc = ""]
        Auto,
        #[serde(rename = "BY_PROPERTY")]
        #[doc = ""]
        ByProperty,
        #[serde(rename = "BY_PAGE")]
        #[doc = ""]
        ByPage,
    }
    impl ::std::default::Default for SearchAnalyticsQueryRequestAggregationTypeEnum {
        fn default() -> Self {
            Self::Auto
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The data state to be fetched, can be full or all, the latter including full and partial data."]
    pub enum SearchAnalyticsQueryRequestDataStateEnum {
        #[serde(rename = "DATA_STATE_UNSPECIFIED")]
        #[doc = "Default value, should not be used."]
        DataStateUnspecified,
        #[serde(rename = "FINAL")]
        #[doc = "Include full final data only, without partial."]
        Final,
        #[serde(rename = "ALL")]
        #[doc = "Include all data, full and partial."]
        All,
    }
    impl ::std::default::Default for SearchAnalyticsQueryRequestDataStateEnum {
        fn default() -> Self {
            Self::DataStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum SearchAnalyticsQueryRequestDimensionsEnum {
        #[serde(rename = "DATE")]
        #[doc = ""]
        Date,
        #[serde(rename = "QUERY")]
        #[doc = ""]
        Query,
        #[serde(rename = "PAGE")]
        #[doc = ""]
        Page,
        #[serde(rename = "COUNTRY")]
        #[doc = ""]
        Country,
        #[serde(rename = "DEVICE")]
        #[doc = ""]
        Device,
        #[serde(rename = "SEARCH_APPEARANCE")]
        #[doc = ""]
        SearchAppearance,
    }
    impl ::std::default::Default for SearchAnalyticsQueryRequestDimensionsEnum {
        fn default() -> Self {
            Self::Date
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Optional; Default is \\\"web\\\"] The search type to filter for."]
    pub enum SearchAnalyticsQueryRequestSearchTypeEnum {
        #[serde(rename = "WEB")]
        #[doc = ""]
        Web,
        #[serde(rename = "IMAGE")]
        #[doc = ""]
        Image,
        #[serde(rename = "VIDEO")]
        #[doc = ""]
        Video,
        #[serde(rename = "NEWS")]
        #[doc = ""]
        News,
    }
    impl ::std::default::Default for SearchAnalyticsQueryRequestSearchTypeEnum {
        fn default() -> Self {
            Self::Web
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
        pub response_aggregation_type:
            ::std::option::Option<SearchAnalyticsQueryResponseResponseAggregationTypeEnum>,
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
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the results were aggregated."]
    pub enum SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
        #[serde(rename = "AUTO")]
        #[doc = ""]
        Auto,
        #[serde(rename = "BY_PROPERTY")]
        #[doc = ""]
        ByProperty,
        #[serde(rename = "BY_PAGE")]
        #[doc = ""]
        ByPage,
    }
    impl ::std::default::Default for SearchAnalyticsQueryResponseResponseAggregationTypeEnum {
        fn default() -> Self {
            Self::Auto
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
        #[doc = "Contains detailed information about a specific URL submitted as a [sitemap](https://support.google.com/webmasters/answer/156184)."]
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
        #[doc = "Contains permission level information about a Search Console site. For more information, see [Permissions in Search Console](https://support.google.com/webmasters/answer/2451999)."]
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
    #[doc = "Final state of the test, including error details if necessary."]
    pub struct TestStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error details if applicable."]
        pub details: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the test."]
        pub status: ::std::option::Option<TestStatusStatusEnum>,
    }
    impl TestStatus {
        pub fn builder() -> TestStatusBuilder {
            TestStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the test."]
    pub enum TestStatusStatusEnum {
        #[serde(rename = "TEST_STATUS_UNSPECIFIED")]
        #[doc = "Internal error when running this test. Please try running the test again."]
        TestStatusUnspecified,
        #[serde(rename = "COMPLETE")]
        #[doc = "Inspection has completed without errors."]
        Complete,
        #[serde(rename = "INTERNAL_ERROR")]
        #[doc = "Inspection terminated in an error state. This indicates a problem in Google's infrastructure, not a user error. Please try again later."]
        InternalError,
        #[serde(rename = "PAGE_UNREACHABLE")]
        #[doc = "Google can not access the URL because of a user error such as a robots.txt blockage, a 403 or 500 code etc. Please make sure that the URL provided is accessible by Googlebot and is not password protected."]
        PageUnreachable,
    }
    impl ::std::default::Default for TestStatusStatusEnum {
        fn default() -> Self {
            Self::TestStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains permission level information about a Search Console site. For more information, see [Permissions in Search Console](https://support.google.com/webmasters/answer/2451999)."]
    pub struct WmxSite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's permission level for the site."]
        pub permission_level: ::std::option::Option<WmxSitePermissionLevelEnum>,
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
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The user's permission level for the site."]
    pub enum WmxSitePermissionLevelEnum {
        #[serde(rename = "SITE_PERMISSION_LEVEL_UNSPECIFIED")]
        #[doc = ""]
        SitePermissionLevelUnspecified,
        #[serde(rename = "SITE_OWNER")]
        #[doc = "Owner has complete access to the site."]
        SiteOwner,
        #[serde(rename = "SITE_FULL_USER")]
        #[doc = "Full users can access all data, and perform most of the operations."]
        SiteFullUser,
        #[serde(rename = "SITE_RESTRICTED_USER")]
        #[doc = "Restricted users can access most of the data, and perform some operations."]
        SiteRestrictedUser,
        #[serde(rename = "SITE_UNVERIFIED_USER")]
        #[doc = "Unverified user has no access to site's data."]
        SiteUnverifiedUser,
    }
    impl ::std::default::Default for WmxSitePermissionLevelEnum {
        fn default() -> Self {
            Self::SitePermissionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains detailed information about a specific URL submitted as a [sitemap](https://support.google.com/webmasters/answer/156184)."]
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
        #[doc = "The type of the sitemap. For example: `rssFeed`."]
        pub _type: ::std::option::Option<WmxSitemapTypeEnum>,
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
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the sitemap. For example: `rssFeed`."]
    pub enum WmxSitemapTypeEnum {
        #[serde(rename = "NOT_SITEMAP")]
        #[doc = ""]
        NotSitemap,
        #[serde(rename = "URL_LIST")]
        #[doc = ""]
        UrlList,
        #[serde(rename = "SITEMAP")]
        #[doc = ""]
        Sitemap,
        #[serde(rename = "RSS_FEED")]
        #[doc = ""]
        RssFeed,
        #[serde(rename = "ATOM_FEED")]
        #[doc = ""]
        AtomFeed,
        #[serde(rename = "PATTERN_SITEMAP")]
        #[doc = "Unsupported sitemap types."]
        PatternSitemap,
        #[serde(rename = "OCEANFRONT")]
        #[doc = ""]
        Oceanfront,
    }
    impl ::std::default::Default for WmxSitemapTypeEnum {
        fn default() -> Self {
            Self::NotSitemap
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
        #[doc = "The specific type of content in this sitemap. For example: `web`."]
        pub _type: ::std::option::Option<WmxSitemapContentTypeEnum>,
    }
    impl WmxSitemapContent {
        pub fn builder() -> WmxSitemapContentBuilder {
            WmxSitemapContentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The specific type of content in this sitemap. For example: `web`."]
    pub enum WmxSitemapContentTypeEnum {
        #[serde(rename = "WEB")]
        #[doc = ""]
        Web,
        #[serde(rename = "IMAGE")]
        #[doc = ""]
        Image,
        #[serde(rename = "VIDEO")]
        #[doc = ""]
        Video,
        #[serde(rename = "NEWS")]
        #[doc = ""]
        News,
        #[serde(rename = "MOBILE")]
        #[doc = ""]
        Mobile,
        #[serde(rename = "ANDROID_APP")]
        #[doc = ""]
        AndroidApp,
        #[serde(rename = "PATTERN")]
        #[doc = "Unsupported content type."]
        Pattern,
        #[serde(rename = "IOS_APP")]
        #[doc = ""]
        IosApp,
        #[serde(rename = "DATA_FEED_ELEMENT")]
        #[doc = "Unsupported content type."]
        DataFeedElement,
    }
    impl ::std::default::Default for WmxSitemapContentTypeEnum {
        fn default() -> Self {
            Self::Web
        }
    }
}
