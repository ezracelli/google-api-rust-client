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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Activity represents data for an activity of a user. Note that an Activity is different from a hit. A hit might result in multiple Activity's. For example, if a hit includes a transaction and a goal completion, there will be two Activity protos for this hit, one for ECOMMERCE and one for GOAL. Conversely, multiple hits can also construct one Activity. In classic e-commerce, data for one transaction might be sent through multiple hits. These hits will be merged into one ECOMMERCE Activity."]
    pub struct Activity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the activity. If activities for a visit cross midnight and occur in two separate dates, then two sessions (one per date) share the session identifier. For example, say session ID 113472 has activity within 2019-08-20, and session ID 243742 has activity within 2019-08-25 and 2019-08-26. Session ID 113472 is one session, and session ID 243742 is two sessions."]
        pub activity_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this activity."]
        pub activity_type: ::std::option::Option<ActivityActivityTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This will be set if `activity_type` equals `SCREEN_VIEW`."]
        pub appview: ::std::option::Option<::std::boxed::Box<ScreenviewData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "campaign")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For manual campaign tracking, it is the value of the utm_campaign campaign tracking parameter. For AdWords autotagging, it is the name(s) of the online ad campaign(s) you use for the property. If you use neither, its value is (not set)."]
        pub campaign: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelGrouping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Channel Group associated with an end user's session for this View (defined by the View's Channel Groupings)."]
        pub channel_grouping: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customDimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all custom dimensions associated with this activity."]
        pub custom_dimension:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomDimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ecommerce")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This will be set if `activity_type` equals `ECOMMERCE`."]
        pub ecommerce: ::std::option::Option<::std::boxed::Box<EcommerceData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains all the details pertaining to an event and will be set if `activity_type` equals `EVENT`."]
        pub event: ::std::option::Option<::std::boxed::Box<EventData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field contains a list of all the goals that were reached in this activity when `activity_type` equals `GOAL`."]
        pub goals: ::std::option::Option<::std::boxed::Box<GoalSetData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hostname from which the tracking request was made."]
        pub hostname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For manual campaign tracking, it is the value of the utm_term campaign tracking parameter. For AdWords traffic, it contains the best matching targeting criteria. For the display network, where multiple targeting criteria could have caused the ad to show up, it returns the best matching targeting criteria as selected by Ads. This could be display_keyword, site placement, boomuserlist, user_interest, age, or gender. Otherwise its value is (not set)."]
        pub keyword: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landingPagePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first page in users' sessions, or the landing page."]
        pub landing_page_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of referrals. For manual campaign tracking, it is the value of the utm_medium campaign tracking parameter. For AdWords autotagging, it is cpc. If users came from a search engine detected by Google Analytics, it is organic. If the referrer is not a search engine, it is referral. If users came directly to the property and document.referrer is empty, its value is (none)."]
        pub medium: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This will be set if `activity_type` equals `PAGEVIEW`. This field contains all the details about the visitor and the page that was visited."]
        pub pageview: ::std::option::Option<::std::boxed::Box<PageviewData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of referrals. For manual campaign tracking, it is the value of the utm_source campaign tracking parameter. For AdWords autotagging, it is google. If you use neither, it is the domain of the source (e.g., document.referrer) referring the users. It may also contain a port address. If users arrived without a referrer, its value is (direct)."]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl Activity {
        pub fn builder() -> ActivityBuilder {
            ActivityBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of this activity."]
    pub enum ActivityActivityTypeEnum {
        #[serde(rename = "ACTIVITY_TYPE_UNSPECIFIED")]
        #[doc = "ActivityType will never have this value in the response. Using this type in the request will result in an error."]
        ActivityTypeUnspecified,
        #[serde(rename = "PAGEVIEW")]
        #[doc = "Used when the activity resulted out of a visitor viewing a page."]
        Pageview,
        #[serde(rename = "SCREENVIEW")]
        #[doc = "Used when the activity resulted out of a visitor using an application on a mobile device."]
        Screenview,
        #[serde(rename = "GOAL")]
        #[doc = "Used to denote that a goal type activity."]
        Goal,
        #[serde(rename = "ECOMMERCE")]
        #[doc = "An e-commerce transaction was performed by the visitor on the page."]
        Ecommerce,
        #[serde(rename = "EVENT")]
        #[doc = "Used when the activity is an event."]
        Event,
    }
    impl ::std::default::Default for ActivityActivityTypeEnum {
        fn default() -> Self {
            Self::ActivityTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a cohort. A cohort is a group of users who share a common characteristic. For example, all users with the same acquisition date belong to the same cohort."]
    pub struct Cohort {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is used for `FIRST_VISIT_DATE` cohort, the cohort selects users whose first visit date is between start date and end date defined in the DateRange. The date ranges should be aligned for cohort requests. If the request contains `ga:cohortNthDay` it should be exactly one day long, if `ga:cohortNthWeek` it should be aligned to the week boundary (starting at Sunday and ending Saturday), and for `ga:cohortNthMonth` the date range should be aligned to the month (starting at the first and ending on the last day of the month). For LTV requests there are no such restrictions. You do not need to supply a date range for the `reportsRequest.dateRanges` field."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique name for the cohort. If not defined name will be auto-generated with values cohort_[1234...]."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the cohort. The only supported type as of now is `FIRST_VISIT_DATE`. If this field is unspecified the cohort is treated as `FIRST_VISIT_DATE` type cohort."]
        pub _type: ::std::option::Option<CohortTypeEnum>,
    }
    impl Cohort {
        pub fn builder() -> CohortBuilder {
            CohortBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the cohort. The only supported type as of now is `FIRST_VISIT_DATE`. If this field is unspecified the cohort is treated as `FIRST_VISIT_DATE` type cohort."]
    pub enum CohortTypeEnum {
        #[serde(rename = "UNSPECIFIED_COHORT_TYPE")]
        #[doc = "If unspecified it's treated as `FIRST_VISIT_DATE`."]
        UnspecifiedCohortType,
        #[serde(rename = "FIRST_VISIT_DATE")]
        #[doc = "Cohorts that are selected based on first visit date."]
        FirstVisitDate,
    }
    impl ::std::default::Default for CohortTypeEnum {
        fn default() -> Self {
            Self::UnspecifiedCohortType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a cohort group. For example: \"cohortGroup\": { \"cohorts\": [{ \"name\": \"cohort 1\", \"type\": \"FIRST_VISIT_DATE\", \"dateRange\": { \"startDate\": \"2015-08-01\", \"endDate\": \"2015-08-01\" } },{ \"name\": \"cohort 2\" \"type\": \"FIRST_VISIT_DATE\" \"dateRange\": { \"startDate\": \"2015-07-01\", \"endDate\": \"2015-07-01\" } }] }"]
    pub struct CohortGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohorts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The definition for the cohort."]
        pub cohorts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cohort>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lifetimeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable Life Time Value (LTV). LTV measures lifetime value for users acquired through different channels. Please see: [Cohort Analysis](https://support.google.com/analytics/answer/6074676) and [Lifetime Value](https://support.google.com/analytics/answer/6182550) If the value of lifetimeValue is false: - The metric values are similar to the values in the web interface cohort report. - The cohort definition date ranges must be aligned to the calendar week and month. i.e. while requesting `ga:cohortNthWeek` the `startDate` in the cohort definition should be a Sunday and the `endDate` should be the following Saturday, and for `ga:cohortNthMonth`, the `startDate` should be the 1st of the month and `endDate` should be the last day of the month. When the lifetimeValue is true: - The metric values will correspond to the values in the web interface LifeTime value report. - The Lifetime Value report shows you how user value (Revenue) and engagement (Appviews, Goal Completions, Sessions, and Session Duration) grow during the 90 days after a user is acquired. - The metrics are calculated as a cumulative average per user per the time increment. - The cohort definition date ranges need not be aligned to the calendar week and month boundaries. - The `viewId` must be an [app view ID](https://support.google.com/analytics/answer/2649553#WebVersusAppViews)"]
        pub lifetime_value: ::std::option::Option<::std::primitive::bool>,
    }
    impl CohortGroup {
        pub fn builder() -> CohortGroupBuilder {
            CohortGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Column headers."]
    pub struct ColumnHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension names in the response."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metric headers for the metrics in the response."]
        pub metric_header: ::std::option::Option<::std::boxed::Box<MetricHeader>>,
    }
    impl ColumnHeader {
        pub fn builder() -> ColumnHeaderBuilder {
            ColumnHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom dimension."]
    pub struct CustomDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Slot number of custom dimension."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the custom dimension. Default value (i.e. empty string) indicates clearing sesion/visitor scope custom dimension value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CustomDimension {
        pub fn builder() -> CustomDimensionBuilder {
            CustomDimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A contiguous set of days: startDate, startDate + 1 day, ..., endDate. The start and end dates are specified in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) date format `YYYY-MM-DD`."]
    pub struct DateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end date for the query in the format `YYYY-MM-DD`."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start date for the query in the format `YYYY-MM-DD`."]
        pub start_date: ::std::option::Option<::std::string::String>,
    }
    impl DateRange {
        pub fn builder() -> DateRangeBuilder {
            DateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used to return a list of metrics for a single DateRange / dimension combination"]
    pub struct DateRangeValues {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotValueRegions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of each pivot region."]
        pub pivot_value_regions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotValueRegion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each value corresponds to each Metric in the request."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl DateRangeValues {
        pub fn builder() -> DateRangeValuesBuilder {
            DateRangeValuesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Dimensions](https://support.google.com/analytics/answer/1033861) are attributes of your data. For example, the dimension `ga:city` indicates the city, for example, \"Paris\" or \"New York\", from which a session originates."]
    pub struct Dimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, we place dimension values into buckets after string to int64. Dimension values that are not the string representation of an integral value will be converted to zero. The bucket values have to be in increasing order. Each bucket is closed on the lower end, and open on the upper end. The \"first\" bucket includes all values less than the first boundary, the \"last\" bucket includes all values up to infinity. Dimension values that fall in a bucket get transformed to a new dimension value. For example, if one gives a list of \"0, 1, 3, 4, 7\", then we return the following buckets: - bucket #1: values < 0, dimension value \"<0\" - bucket #2: values in [0,1), dimension value \"0\" - bucket #3: values in [1,3), dimension value \"1-2\" - bucket #4: values in [3,4), dimension value \"3\" - bucket #5: values in [4,7), dimension value \"4-6\" - bucket #6: values >= 7, dimension value \"7+\" NOTE: If you are applying histogram mutation on any dimension, and using that dimension in sort, you will want to use the sort type `HISTOGRAM_BUCKET` for that purpose. Without that the dimension values will be sorted according to dictionary (lexicographic) order. For example the ascending dictionary order is: \"<50\", \"1001+\", \"121-1000\", \"50-120\" And the ascending `HISTOGRAM_BUCKET` order is: \"<50\", \"50-120\", \"121-1000\", \"1001+\" The client has to explicitly request `\"orderType\": \"HISTOGRAM_BUCKET\"` for a histogram-mutated dimension."]
        pub histogram_buckets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the dimension to fetch, for example `ga:browser`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Dimension {
        pub fn builder() -> DimensionBuilder {
            DimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimension filter specifies the filtering options on a dimension."]
    pub struct DimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Should the match be case sensitive? Default is false."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension to filter on. A DimensionFilter must contain a dimension."]
        pub dimension_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Strings or regular expression to match against. Only the first value of the list is used for comparison unless the operator is `IN_LIST`. If `IN_LIST` operator, then the entire list is used to filter the dimensions as explained in the description of the `IN_LIST` operator."]
        pub expressions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "not")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logical `NOT` operator. If this boolean is set to true, then the matching dimension values will be excluded in the report. The default is false."]
        pub not: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How to match the dimension to the expression. The default is REGEXP."]
        pub operator: ::std::option::Option<DimensionFilterOperatorEnum>,
    }
    impl DimensionFilter {
        pub fn builder() -> DimensionFilterBuilder {
            DimensionFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How to match the dimension to the expression. The default is REGEXP."]
    pub enum DimensionFilterOperatorEnum {
        #[serde(rename = "OPERATOR_UNSPECIFIED")]
        #[doc = "If the match type is unspecified, it is treated as a `REGEXP`."]
        OperatorUnspecified,
        #[serde(rename = "REGEXP")]
        #[doc = "The match expression is treated as a regular expression. All match types are not treated as regular expressions."]
        Regexp,
        #[serde(rename = "BEGINS_WITH")]
        #[doc = "Matches the value which begin with the match expression provided."]
        BeginsWith,
        #[serde(rename = "ENDS_WITH")]
        #[doc = "Matches the values which end with the match expression provided."]
        EndsWith,
        #[serde(rename = "PARTIAL")]
        #[doc = "Substring match."]
        Partial,
        #[serde(rename = "EXACT")]
        #[doc = "The value should match the match expression entirely."]
        Exact,
        #[serde(rename = "NUMERIC_EQUAL")]
        #[doc = "Integer comparison filters. case sensitivity is ignored for these and the expression is assumed to be a string representing an integer. Failure conditions: - If expression is not a valid int64, the client should expect an error. - Input dimensions that are not valid int64 values will never match the filter."]
        NumericEqual,
        #[serde(rename = "NUMERIC_GREATER_THAN")]
        #[doc = "Checks if the dimension is numerically greater than the match expression. Read the description for `NUMERIC_EQUALS` for restrictions."]
        NumericGreaterThan,
        #[serde(rename = "NUMERIC_LESS_THAN")]
        #[doc = "Checks if the dimension is numerically less than the match expression. Read the description for `NUMERIC_EQUALS` for restrictions."]
        NumericLessThan,
        #[serde(rename = "IN_LIST")]
        #[doc = "This option is used to specify a dimension filter whose expression can take any value from a selected list of values. This helps avoiding evaluating multiple exact match dimension filters which are OR'ed for every single response row. For example: expressions: [\"A\", \"B\", \"C\"] Any response row whose dimension has it is value as A, B or C, matches this DimensionFilter."]
        InList,
    }
    impl ::std::default::Default for DimensionFilterOperatorEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A group of dimension filters. Set the operator value to specify how the filters are logically combined."]
    pub struct DimensionFilterClause {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The repeated set of filters. They are logically combined based on the operator specified."]
        pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator for combining multiple dimension filters. If unspecified, it is treated as an `OR`."]
        pub operator: ::std::option::Option<DimensionFilterClauseOperatorEnum>,
    }
    impl DimensionFilterClause {
        pub fn builder() -> DimensionFilterClauseBuilder {
            DimensionFilterClauseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operator for combining multiple dimension filters. If unspecified, it is treated as an `OR`."]
    pub enum DimensionFilterClauseOperatorEnum {
        #[serde(rename = "OPERATOR_UNSPECIFIED")]
        #[doc = "Unspecified operator. It is treated as an `OR`."]
        OperatorUnspecified,
        #[serde(rename = "OR")]
        #[doc = "The logical `OR` operator."]
        Or,
        #[serde(rename = "AND")]
        #[doc = "The logical `AND` operator."]
        And,
    }
    impl ::std::default::Default for DimensionFilterClauseOperatorEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dynamic segment definition for defining the segment within the request. A segment can select users, sessions or both."]
    pub struct DynamicSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the dynamic segment."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session Segment to select sessions to include in the segment."]
        pub session_segment: ::std::option::Option<::std::boxed::Box<SegmentDefinition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User Segment to select users to include in the segment."]
        pub user_segment: ::std::option::Option<::std::boxed::Box<SegmentDefinition>>,
    }
    impl DynamicSegment {
        pub fn builder() -> DynamicSegmentBuilder {
            DynamicSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "E-commerce details associated with the user activity."]
    pub struct EcommerceData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action associated with this e-commerce action."]
        pub action_type: ::std::option::Option<EcommerceDataActionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ecommerceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this e-commerce activity."]
        pub ecommerce_type: ::std::option::Option<EcommerceDataEcommerceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "products")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the products in this transaction."]
        pub products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transaction details of this e-commerce action."]
        pub transaction: ::std::option::Option<::std::boxed::Box<TransactionData>>,
    }
    impl EcommerceData {
        pub fn builder() -> EcommerceDataBuilder {
            EcommerceDataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Action associated with this e-commerce action."]
    pub enum EcommerceDataActionTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Action type is not known."]
        Unknown,
        #[serde(rename = "CLICK")]
        #[doc = "Click through of product lists."]
        Click,
        #[serde(rename = "DETAILS_VIEW")]
        #[doc = "Product detail views."]
        DetailsView,
        #[serde(rename = "ADD_TO_CART")]
        #[doc = "Add product(s) to cart."]
        AddToCart,
        #[serde(rename = "REMOVE_FROM_CART")]
        #[doc = "Remove product(s) from cart."]
        RemoveFromCart,
        #[serde(rename = "CHECKOUT")]
        #[doc = "Check out."]
        Checkout,
        #[serde(rename = "PAYMENT")]
        #[doc = "Completed purchase."]
        Payment,
        #[serde(rename = "REFUND")]
        #[doc = "Refund of purchase."]
        Refund,
        #[serde(rename = "CHECKOUT_OPTION")]
        #[doc = "Checkout options."]
        CheckoutOption,
    }
    impl ::std::default::Default for EcommerceDataActionTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this e-commerce activity."]
    pub enum EcommerceDataEcommerceTypeEnum {
        #[serde(rename = "ECOMMERCE_TYPE_UNSPECIFIED")]
        #[doc = "Used when the e-commerce activity type is unspecified."]
        EcommerceTypeUnspecified,
        #[serde(rename = "CLASSIC")]
        #[doc = "Used when activity has classic (non-enhanced) e-commerce information."]
        Classic,
        #[serde(rename = "ENHANCED")]
        #[doc = "Used when activity has enhanced e-commerce information."]
        Enhanced,
    }
    impl ::std::default::Default for EcommerceDataEcommerceTypeEnum {
        fn default() -> Self {
            Self::EcommerceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents all the details pertaining to an event."]
    pub struct EventData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of interaction with the object. Eg: 'play'."]
        pub event_action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object on the page that was interacted with. Eg: 'Video'."]
        pub event_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of such events in this activity."]
        pub event_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label attached with the event."]
        pub event_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numeric value associated with the event."]
        pub event_value: ::std::option::Option<::std::string::String>,
    }
    impl EventData {
        pub fn builder() -> EventDataBuilder {
            EventDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The batch request containing multiple report request."]
    pub struct GetReportsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requests, each request will have a separate response. There can be a maximum of 5 requests. All requests should have the same `dateRanges`, `viewId`, `segments`, `samplingLevel`, and `cohortGroup`."]
        pub report_requests:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportRequest>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useResourceQuotas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables [resource based quotas](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4), (defaults to `False`). If this field is set to `True` the per view (profile) quotas are governed by the computational cost of the request. Note that using cost based quotas will higher enable sampling rates. (10 Million for `SMALL`, 100M for `LARGE`. See the [limits and quotas documentation](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4) for details."]
        pub use_resource_quotas: ::std::option::Option<::std::primitive::bool>,
    }
    impl GetReportsRequest {
        pub fn builder() -> GetReportsRequestBuilder {
            GetReportsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The main response class which holds the reports from the Reporting API `batchGet` call."]
    pub struct GetReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of resource quota tokens deducted to execute the query. Includes all responses."]
        pub query_cost: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Responses corresponding to each of the request."]
        pub reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Report>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceQuotasRemaining")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of resource quota remaining for the property."]
        pub resource_quotas_remaining:
            ::std::option::Option<::std::boxed::Box<ResourceQuotasRemaining>>,
    }
    impl GetReportsResponse {
        pub fn builder() -> GetReportsResponseBuilder {
            GetReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents all the details pertaining to a goal."]
    pub struct GoalData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalCompletionLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the page where this goal was completed."]
        pub goal_completion_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalCompletions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of goal completions in this activity."]
        pub goal_completions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This identifies the goal as configured for the profile."]
        pub goal_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the goal."]
        pub goal_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalPreviousStep1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the page one step prior to the goal completion."]
        pub goal_previous_step1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalPreviousStep2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the page two steps prior to the goal completion."]
        pub goal_previous_step2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalPreviousStep3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the page three steps prior to the goal completion."]
        pub goal_previous_step3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goalValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value in this goal."]
        pub goal_value: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoalData {
        pub fn builder() -> GoalDataBuilder {
            GoalDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a set of goals that were reached in an activity."]
    pub struct GoalSetData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the goals that were reached in the current activity."]
        pub goals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoalData>>>,
    }
    impl GoalSetData {
        pub fn builder() -> GoalSetDataBuilder {
            GoalSetDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Metrics](https://support.google.com/analytics/answer/1033861) are the quantitative measurements. For example, the metric `ga:users` indicates the total number of users for the requested time period."]
    pub struct Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alias for the metric expression is an alternate name for the expression. The alias can be used for filtering and sorting. This field is optional and is useful if the expression is not a single metric but a complex expression which cannot be used in filtering and sorting. The alias is also used in the response column header."]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A metric expression in the request. An expression is constructed from one or more metrics and numbers. Accepted operators include: Plus (+), Minus (-), Negation (Unary -), Divided by (/), Multiplied by (*), Parenthesis, Positive cardinal numbers (0-9), can include decimals and is limited to 1024 characters. Example `ga:totalRefunds/ga:users`, in most cases the metric expression is just a single metric name like `ga:users`. Adding mixed `MetricType` (E.g., `CURRENCY` + `PERCENTAGE`) metrics will result in unexpected results."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how the metric expression should be formatted, for example `INTEGER`."]
        pub formatting_type: ::std::option::Option<MetricFormattingTypeEnum>,
    }
    impl Metric {
        pub fn builder() -> MetricBuilder {
            MetricBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies how the metric expression should be formatted, for example `INTEGER`."]
    pub enum MetricFormattingTypeEnum {
        #[serde(rename = "METRIC_TYPE_UNSPECIFIED")]
        #[doc = "Metric type is unspecified."]
        MetricTypeUnspecified,
        #[serde(rename = "INTEGER")]
        #[doc = "Integer metric."]
        Integer,
        #[serde(rename = "FLOAT")]
        #[doc = "Float metric."]
        Float,
        #[serde(rename = "CURRENCY")]
        #[doc = "Currency metric."]
        Currency,
        #[serde(rename = "PERCENT")]
        #[doc = "Percentage metric."]
        Percent,
        #[serde(rename = "TIME")]
        #[doc = "Time metric in `HH:MM:SS` format."]
        Time,
    }
    impl ::std::default::Default for MetricFormattingTypeEnum {
        fn default() -> Self {
            Self::MetricTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "MetricFilter specifies the filter on a metric."]
    pub struct MetricFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to compare against."]
        pub comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric that will be filtered on. A metricFilter must contain a metric name. A metric name can be an alias earlier defined as a metric or it can also be a metric expression."]
        pub metric_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "not")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logical `NOT` operator. If this boolean is set to true, then the matching metric values will be excluded in the report. The default is false."]
        pub not: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is the metric `EQUAL`, `LESS_THAN` or `GREATER_THAN` the comparisonValue, the default is `EQUAL`. If the operator is `IS_MISSING`, checks if the metric is missing and would ignore the comparisonValue."]
        pub operator: ::std::option::Option<MetricFilterOperatorEnum>,
    }
    impl MetricFilter {
        pub fn builder() -> MetricFilterBuilder {
            MetricFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Is the metric `EQUAL`, `LESS_THAN` or `GREATER_THAN` the comparisonValue, the default is `EQUAL`. If the operator is `IS_MISSING`, checks if the metric is missing and would ignore the comparisonValue."]
    pub enum MetricFilterOperatorEnum {
        #[serde(rename = "OPERATOR_UNSPECIFIED")]
        #[doc = "If the operator is not specified, it is treated as `EQUAL`."]
        OperatorUnspecified,
        #[serde(rename = "EQUAL")]
        #[doc = "Should the value of the metric be exactly equal to the comparison value."]
        Equal,
        #[serde(rename = "LESS_THAN")]
        #[doc = "Should the value of the metric be less than to the comparison value."]
        LessThan,
        #[serde(rename = "GREATER_THAN")]
        #[doc = "Should the value of the metric be greater than to the comparison value."]
        GreaterThan,
        #[serde(rename = "IS_MISSING")]
        #[doc = "Validates if the metric is missing. Doesn't take comparisonValue into account."]
        IsMissing,
    }
    impl ::std::default::Default for MetricFilterOperatorEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a group of metric filters. Set the operator value to specify how the filters are logically combined."]
    pub struct MetricFilterClause {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The repeated set of filters. They are logically combined based on the operator specified."]
        pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator for combining multiple metric filters. If unspecified, it is treated as an `OR`."]
        pub operator: ::std::option::Option<MetricFilterClauseOperatorEnum>,
    }
    impl MetricFilterClause {
        pub fn builder() -> MetricFilterClauseBuilder {
            MetricFilterClauseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operator for combining multiple metric filters. If unspecified, it is treated as an `OR`."]
    pub enum MetricFilterClauseOperatorEnum {
        #[serde(rename = "OPERATOR_UNSPECIFIED")]
        #[doc = "Unspecified operator. It is treated as an `OR`."]
        OperatorUnspecified,
        #[serde(rename = "OR")]
        #[doc = "The logical `OR` operator."]
        Or,
        #[serde(rename = "AND")]
        #[doc = "The logical `AND` operator."]
        And,
    }
    impl ::std::default::Default for MetricFilterClauseOperatorEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The headers for the metrics."]
    pub struct MetricHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricHeaderEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headers for the metrics in the response."]
        pub metric_header_entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricHeaderEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headers for the pivots in the response."]
        pub pivot_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotHeader>>>,
    }
    impl MetricHeader {
        pub fn builder() -> MetricHeaderBuilder {
            MetricHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Header for the metrics."]
    pub struct MetricHeaderEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the header."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the metric, for example `INTEGER`."]
        pub _type: ::std::option::Option<MetricHeaderEntryTypeEnum>,
    }
    impl MetricHeaderEntry {
        pub fn builder() -> MetricHeaderEntryBuilder {
            MetricHeaderEntryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the metric, for example `INTEGER`."]
    pub enum MetricHeaderEntryTypeEnum {
        #[serde(rename = "METRIC_TYPE_UNSPECIFIED")]
        #[doc = "Metric type is unspecified."]
        MetricTypeUnspecified,
        #[serde(rename = "INTEGER")]
        #[doc = "Integer metric."]
        Integer,
        #[serde(rename = "FLOAT")]
        #[doc = "Float metric."]
        Float,
        #[serde(rename = "CURRENCY")]
        #[doc = "Currency metric."]
        Currency,
        #[serde(rename = "PERCENT")]
        #[doc = "Percentage metric."]
        Percent,
        #[serde(rename = "TIME")]
        #[doc = "Time metric in `HH:MM:SS` format."]
        Time,
    }
    impl ::std::default::Default for MetricHeaderEntryTypeEnum {
        fn default() -> Self {
            Self::MetricTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of segment filters in the `OR` group are combined with the logical OR operator."]
    pub struct OrFiltersForSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentFilterClauses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of segment filters to be combined with a `OR` operator."]
        pub segment_filter_clauses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SegmentFilterClause>>>,
    }
    impl OrFiltersForSegment {
        pub fn builder() -> OrFiltersForSegmentBuilder {
            OrFiltersForSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the sorting options."]
    pub struct OrderBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field which to sort by. The default sort order is ascending. Example: `ga:browser`. Note, that you can only specify one field for sort here. For example, `ga:browser, ga:city` is not valid."]
        pub field_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order type. The default orderType is `VALUE`."]
        pub order_type: ::std::option::Option<OrderByOrderTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sorting order for the field."]
        pub sort_order: ::std::option::Option<OrderBySortOrderEnum>,
    }
    impl OrderBy {
        pub fn builder() -> OrderByBuilder {
            OrderByBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The order type. The default orderType is `VALUE`."]
    pub enum OrderByOrderTypeEnum {
        #[serde(rename = "ORDER_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified order type will be treated as sort based on value."]
        OrderTypeUnspecified,
        #[serde(rename = "VALUE")]
        #[doc = "The sort order is based on the value of the chosen column; looks only at the first date range."]
        Value,
        #[serde(rename = "DELTA")]
        #[doc = "The sort order is based on the difference of the values of the chosen column between the first two date ranges. Usable only if there are exactly two date ranges."]
        Delta,
        #[serde(rename = "SMART")]
        #[doc = "The sort order is based on weighted value of the chosen column. If column has n/d format, then weighted value of this ratio will be `(n + totals.n)/(d + totals.d)` Usable only for metrics that represent ratios."]
        Smart,
        #[serde(rename = "HISTOGRAM_BUCKET")]
        #[doc = "Histogram order type is applicable only to dimension columns with non-empty histogram-buckets."]
        HistogramBucket,
        #[serde(rename = "DIMENSION_AS_INTEGER")]
        #[doc = "If the dimensions are fixed length numbers, ordinary sort would just work fine. `DIMENSION_AS_INTEGER` can be used if the dimensions are variable length numbers."]
        DimensionAsInteger,
    }
    impl ::std::default::Default for OrderByOrderTypeEnum {
        fn default() -> Self {
            Self::OrderTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The sorting order for the field."]
    pub enum OrderBySortOrderEnum {
        #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
        #[doc = "If the sort order is unspecified, the default is ascending."]
        SortOrderUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "Ascending sort. The field will be sorted in an ascending manner."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "Descending sort. The field will be sorted in a descending manner."]
        Descending,
    }
    impl ::std::default::Default for OrderBySortOrderEnum {
        fn default() -> Self {
            Self::SortOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents details collected when the visitor views a page."]
    pub struct PageviewData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the page that the visitor viewed."]
        pub page_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the page that the visitor viewed."]
        pub page_title: ::std::option::Option<::std::string::String>,
    }
    impl PageviewData {
        pub fn builder() -> PageviewDataBuilder {
            PageviewDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Pivot describes the pivot section in the request. The Pivot helps rearrange the information in the table for certain reports by pivoting your data on a second dimension."]
    pub struct Pivot {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilterClauses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DimensionFilterClauses are logically combined with an `AND` operator: only data that is included by all these DimensionFilterClauses contributes to the values in this pivot region. Dimension filters can be used to restrict the columns shown in the pivot region. For example if you have `ga:browser` as the requested dimension in the pivot region, and you specify key filters to restrict `ga:browser` to only \"IE\" or \"Firefox\", then only those two browsers would show up as columns."]
        pub dimension_filter_clauses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionFilterClause>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of dimensions to show as pivot columns. A Pivot can have a maximum of 4 dimensions. Pivot dimensions are part of the restriction on the total number of dimensions allowed in the request."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxGroupCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the maximum number of groups to return. The default value is 10, also the maximum value is 1,000."]
        pub max_group_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pivot metrics. Pivot metrics are part of the restriction on total number of metrics allowed in the request."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If k metrics were requested, then the response will contain some data-dependent multiple of k columns in the report. E.g., if you pivoted on the dimension `ga:browser` then you'd get k columns for \"Firefox\", k columns for \"IE\", k columns for \"Chrome\", etc. The ordering of the groups of columns is determined by descending order of \"total\" for the first of the k values. Ties are broken by lexicographic ordering of the first pivot dimension, then lexicographic ordering of the second pivot dimension, and so on. E.g., if the totals for the first value for Firefox, IE, and Chrome were 8, 2, 8, respectively, the order of columns would be Chrome, Firefox, IE. The following let you choose which of the groups of k columns are included in the response."]
        pub start_group: ::std::option::Option<::std::primitive::i64>,
    }
    impl Pivot {
        pub fn builder() -> PivotBuilder {
            PivotBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The headers for each of the pivot sections defined in the request."]
    pub struct PivotHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotHeaderEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A single pivot section header."]
        pub pivot_header_entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotHeaderEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPivotGroupsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of groups for this pivot."]
        pub total_pivot_groups_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl PivotHeader {
        pub fn builder() -> PivotHeaderBuilder {
            PivotHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The headers for the each of the metric column corresponding to the metrics requested in the pivots section of the response."]
    pub struct PivotHeaderEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the dimensions in the pivot response."]
        pub dimension_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values for the dimensions in the pivot."]
        pub dimension_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric header for the metric in the pivot."]
        pub metric: ::std::option::Option<::std::boxed::Box<MetricHeaderEntry>>,
    }
    impl PivotHeaderEntry {
        pub fn builder() -> PivotHeaderEntryBuilder {
            PivotHeaderEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metric values in the pivot region."]
    pub struct PivotValueRegion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of the metrics in each of the pivot regions."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PivotValueRegion {
        pub fn builder() -> PivotValueRegionBuilder {
            PivotValueRegionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of the products in an e-commerce transaction."]
    pub struct ProductData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemRevenue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total revenue from purchased product items."]
        pub item_revenue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The product name, supplied by the e-commerce tracking application, for the purchased items."]
        pub product_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productQuantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of this product units in the transaction."]
        pub product_quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSku")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique code that represents the product."]
        pub product_sku: ::std::option::Option<::std::string::String>,
    }
    impl ProductData {
        pub fn builder() -> ProductDataBuilder {
            ProductDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The data response corresponding to the request."]
    pub struct Report {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column headers."]
        pub column_header: ::std::option::Option<::std::boxed::Box<ColumnHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response data."]
        pub data: ::std::option::Option<::std::boxed::Box<ReportData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Report {
        pub fn builder() -> ReportBuilder {
            ReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The data part of the report."]
    pub struct ReportData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataLastRefreshed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the data in the report was refreshed. All the hits received before this timestamp are included in the calculation of the report."]
        pub data_last_refreshed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDataGolden")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if response to this request is golden or not. Data is golden when the exact same request will not produce any new results if asked at a later point in time."]
        pub is_data_golden: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum and maximum values seen over all matching rows. These are both empty when `hideValueRanges` in the request is false, or when rowCount is zero."]
        pub maximums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRangeValues>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum and maximum values seen over all matching rows. These are both empty when `hideValueRanges` in the request is false, or when rowCount is zero."]
        pub minimums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRangeValues>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of matching rows for this query."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "There's one ReportRow for every unique combination of dimensions."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplesReadCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the results are [sampled](https://support.google.com/analytics/answer/2637192), this returns the total number of samples read, one entry per date range. If the results are not sampled this field will not be defined. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details."]
        pub samples_read_counts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingSpaceSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the results are [sampled](https://support.google.com/analytics/answer/2637192), this returns the total number of samples present, one entry per date range. If the results are not sampled this field will not be defined. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details."]
        pub sampling_space_sizes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For each requested date range, for the set of all rows that match the query, every requested value format gets a total. The total for a value format is computed by first totaling the metrics mentioned in the value format and then evaluating the value format as a scalar expression. E.g., The \"totals\" for `3 / (ga:sessions + 2)` we compute `3 / ((sum of all relevant ga:sessions) + 2)`. Totals are computed before pagination."]
        pub totals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRangeValues>>>,
    }
    impl ReportData {
        pub fn builder() -> ReportDataBuilder {
            ReportDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The main request class which specifies the Reporting API request."]
    pub struct ReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohortGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cohort group associated with this request. If there is a cohort group in the request the `ga:cohort` dimension must be present. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `cohortGroup` definition."]
        pub cohort_group: ::std::option::Option<::std::boxed::Box<CohortGroup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date ranges in the request. The request can have a maximum of 2 date ranges. The response will contain a set of metric values for each combination of the dimensions for each date range in the request. So, if there are two date ranges, there will be two set of metric values, one for the original date range and one for the second date range. The `reportRequest.dateRanges` field should not be specified for cohorts or Lifetime value requests. If a date range is not provided, the default date range is (startDate: current date - 7 days, endDate: current date - 1 day). Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `dateRanges` definition."]
        pub date_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilterClauses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension filter clauses for filtering Dimension Values. They are logically combined with the `AND` operator. Note that filtering occurs before any dimensions are aggregated, so that the returned metrics represent the total for only the relevant dimensions."]
        pub dimension_filter_clauses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionFilterClause>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions requested. Requests can have a total of 9 dimensions."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filtersExpression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension or metric filters that restrict the data returned for your request. To use the `filtersExpression`, supply a dimension or metric on which to filter, followed by the filter expression. For example, the following expression selects `ga:browser` dimension which starts with Firefox; `ga:browser=~^Firefox`. For more information on dimensions and metric filters, see [Filters reference](https://developers.google.com/analytics/devguides/reporting/core/v3/reference#filters)."]
        pub filters_expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideTotals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, hides the total of all metrics for all the matching rows, for every date range. The default false and will return the totals."]
        pub hide_totals: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hideValueRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, hides the minimum and maximum across all matching rows. The default is false and the value ranges are returned."]
        pub hide_value_ranges: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeEmptyRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to false, the response does not include rows if all the retrieved metrics are equal to zero. The default is false which will exclude these rows."]
        pub include_empty_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricFilterClauses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric filter clauses. They are logically combined with the `AND` operator. Metric filters look at only the first date range and not the comparing date range. Note that filtering on metrics occurs after the metrics are aggregated."]
        pub metric_filter_clauses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricFilterClause>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics requested. Requests must specify at least one metric. Requests can have a total of 10 metrics."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort order on output rows. To compare two rows, the elements of the following are applied in order until a difference is found. All date ranges in the output get the same row order."]
        pub order_bys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderBy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page size is for paging and specifies the maximum number of returned rows. Page size should be >= 0. A query returns the default of 1,000 rows. The Analytics Core Reporting API returns a maximum of 100,000 rows per request, no matter how many you ask for. It can also return fewer rows than requested, if there aren't as many dimension segments as you expect. For instance, there are fewer than 300 possible values for `ga:country`, so when segmenting only by country, you can't get more than 300 rows, even if you set `pageSize` to a higher value."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to get the next page of the results. Adding this to the request will return the rows after the pageToken. The pageToken should be the value returned in the nextPageToken parameter in the response to the GetReports request."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivots")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pivot definitions. Requests can have a maximum of 2 pivots."]
        pub pivots: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Pivot>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired report [sample](https://support.google.com/analytics/answer/2637192) size. If the the `samplingLevel` field is unspecified the `DEFAULT` sampling level is used. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `samplingLevel` definition. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details."]
        pub sampling_level: ::std::option::Option<ReportRequestSamplingLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segment the data returned for the request. A segment definition helps look at a subset of the segment request. A request can contain up to four segments. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `segments` definition. Requests with segments must have the `ga:segment` dimension."]
        pub segments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Segment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Analytics [view ID](https://support.google.com/analytics/answer/1009618) from which to retrieve data. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `viewId`."]
        pub view_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportRequest {
        pub fn builder() -> ReportRequestBuilder {
            ReportRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The desired report [sample](https://support.google.com/analytics/answer/2637192) size. If the the `samplingLevel` field is unspecified the `DEFAULT` sampling level is used. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `samplingLevel` definition. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details."]
    pub enum ReportRequestSamplingLevelEnum {
        #[serde(rename = "SAMPLING_UNSPECIFIED")]
        #[doc = "If the `samplingLevel` field is unspecified the `DEFAULT` sampling level is used."]
        SamplingUnspecified,
        #[serde(rename = "DEFAULT")]
        #[doc = "Returns response with a sample size that balances speed and accuracy."]
        Default,
        #[serde(rename = "SMALL")]
        #[doc = "It returns a fast response with a smaller sampling size."]
        Small,
        #[serde(rename = "LARGE")]
        #[doc = "Returns a more accurate response using a large sampling size. But this may result in response being slower."]
        Large,
    }
    impl ::std::default::Default for ReportRequestSamplingLevelEnum {
        fn default() -> Self {
            Self::SamplingUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row in the report."]
    pub struct ReportRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of requested dimensions."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of metrics for each requested DateRange."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRangeValues>>>,
    }
    impl ReportRow {
        pub fn builder() -> ReportRowBuilder {
            ReportRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The resource quota tokens remaining for the property after the request is completed."]
    pub struct ResourceQuotasRemaining {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dailyQuotaTokensRemaining")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Daily resource quota remaining remaining."]
        pub daily_quota_tokens_remaining: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hourlyQuotaTokensRemaining")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hourly resource quota tokens remaining."]
        pub hourly_quota_tokens_remaining: ::std::option::Option<::std::primitive::i64>,
    }
    impl ResourceQuotasRemaining {
        pub fn builder() -> ResourceQuotasRemainingBuilder {
            ResourceQuotasRemainingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ScreenviewData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application name."]
        pub app_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileDeviceBranding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile manufacturer or branded name. Eg: \"Google\", \"Apple\" etc."]
        pub mobile_device_branding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileDeviceModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile device model. Eg: \"Pixel\", \"iPhone\" etc."]
        pub mobile_device_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the screen."]
        pub screen_name: ::std::option::Option<::std::string::String>,
    }
    impl ScreenviewData {
        pub fn builder() -> ScreenviewDataBuilder {
            ScreenviewDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to fetch User Report from Reporting API `userActivity:get` call."]
    pub struct SearchUserActivityRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of all activity types being requested. Only acvities matching these types will be returned in the response. If empty, all activies will be returned."]
        pub activity_types:
            ::std::option::Option<::std::vec::Vec<SearchUserActivityRequestActivityTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date range for which to retrieve the user activity. If a date range is not provided, the default date range is (startDate: current date - 7 days, endDate: current date - 1 day)."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page size is for paging and specifies the maximum number of returned rows. Page size should be > 0. If the value is 0 or if the field isn't specified, the request returns the default of 1000 rows per page."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to get the next page of the results. Adding this to the request will return the rows after the pageToken. The pageToken should be the value returned in the nextPageToken parameter in the response to the [SearchUserActivityRequest](#SearchUserActivityRequest) request."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Unique user Id to query for. Every [SearchUserActivityRequest](#SearchUserActivityRequest) must contain this field."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Analytics [view ID](https://support.google.com/analytics/answer/1009618) from which to retrieve data. Every [SearchUserActivityRequest](#SearchUserActivityRequest) must contain the `viewId`."]
        pub view_id: ::std::option::Option<::std::string::String>,
    }
    impl SearchUserActivityRequest {
        pub fn builder() -> SearchUserActivityRequestBuilder {
            SearchUserActivityRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum SearchUserActivityRequestActivityTypesEnum {
        #[serde(rename = "ACTIVITY_TYPE_UNSPECIFIED")]
        #[doc = "ActivityType will never have this value in the response. Using this type in the request will result in an error."]
        ActivityTypeUnspecified,
        #[serde(rename = "PAGEVIEW")]
        #[doc = "Used when the activity resulted out of a visitor viewing a page."]
        Pageview,
        #[serde(rename = "SCREENVIEW")]
        #[doc = "Used when the activity resulted out of a visitor using an application on a mobile device."]
        Screenview,
        #[serde(rename = "GOAL")]
        #[doc = "Used to denote that a goal type activity."]
        Goal,
        #[serde(rename = "ECOMMERCE")]
        #[doc = "An e-commerce transaction was performed by the visitor on the page."]
        Ecommerce,
        #[serde(rename = "EVENT")]
        #[doc = "Used when the activity is an event."]
        Event,
    }
    impl ::std::default::Default for SearchUserActivityRequestActivityTypesEnum {
        fn default() -> Self {
            Self::ActivityTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response from `userActivity:get` call."]
    pub struct SearchUserActivityResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This token should be passed to [SearchUserActivityRequest](#SearchUserActivityRequest) to retrieve the next page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field represents the [sampling rate](https://support.google.com/analytics/answer/2637192) for the given request and is a number between 0.0 to 1.0. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details."]
        pub sample_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each record represents a session (device details, duration, etc)."]
        pub sessions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserActivitySession>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total rows returned by this query (across different pages)."]
        pub total_rows: ::std::option::Option<::std::primitive::i64>,
    }
    impl SearchUserActivityResponse {
        pub fn builder() -> SearchUserActivityResponseBuilder {
            SearchUserActivityResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The segment definition, if the report needs to be segmented. A Segment is a subset of the Analytics data. For example, of the entire set of users, one Segment might be users from a particular country or city."]
    pub struct Segment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A dynamic segment definition in the request."]
        pub dynamic_segment: ::std::option::Option<::std::boxed::Box<DynamicSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The segment ID of a built-in or custom segment, for example `gaid::-3`."]
        pub segment_id: ::std::option::Option<::std::string::String>,
    }
    impl Segment {
        pub fn builder() -> SegmentBuilder {
            SegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SegmentDefinition defines the segment to be a set of SegmentFilters which are combined together with a logical `AND` operation."]
    pub struct SegmentDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A segment is defined by a set of segment filters which are combined together with a logical `AND` operation."]
        pub segment_filters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SegmentFilter>>>,
    }
    impl SegmentDefinition {
        pub fn builder() -> SegmentDefinitionBuilder {
            SegmentDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimension filter specifies the filtering options on a dimension."]
    pub struct SegmentDimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Should the match be case sensitive, ignored for `IN_LIST` operator."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the dimension for which the filter is being applied."]
        pub dimension_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of expressions, only the first element is used for all operators"]
        pub expressions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxComparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum comparison values for `BETWEEN` match type."]
        pub max_comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minComparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum comparison values for `BETWEEN` match type."]
        pub min_comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator to use to match the dimension with the expressions."]
        pub operator: ::std::option::Option<SegmentDimensionFilterOperatorEnum>,
    }
    impl SegmentDimensionFilter {
        pub fn builder() -> SegmentDimensionFilterBuilder {
            SegmentDimensionFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operator to use to match the dimension with the expressions."]
    pub enum SegmentDimensionFilterOperatorEnum {
        #[serde(rename = "OPERATOR_UNSPECIFIED")]
        #[doc = "If the match type is unspecified, it is treated as a REGEXP."]
        OperatorUnspecified,
        #[serde(rename = "REGEXP")]
        #[doc = "The match expression is treated as a regular expression. All other match types are not treated as regular expressions."]
        Regexp,
        #[serde(rename = "BEGINS_WITH")]
        #[doc = "Matches the values which begin with the match expression provided."]
        BeginsWith,
        #[serde(rename = "ENDS_WITH")]
        #[doc = "Matches the values which end with the match expression provided."]
        EndsWith,
        #[serde(rename = "PARTIAL")]
        #[doc = "Substring match."]
        Partial,
        #[serde(rename = "EXACT")]
        #[doc = "The value should match the match expression entirely."]
        Exact,
        #[serde(rename = "IN_LIST")]
        #[doc = "This option is used to specify a dimension filter whose expression can take any value from a selected list of values. This helps avoiding evaluating multiple exact match dimension filters which are OR'ed for every single response row. For example: expressions: [\"A\", \"B\", \"C\"] Any response row whose dimension has it is value as A, B or C, matches this DimensionFilter."]
        InList,
        #[serde(rename = "NUMERIC_LESS_THAN")]
        #[doc = "Integer comparison filters. case sensitivity is ignored for these and the expression is assumed to be a string representing an integer. Failure conditions: - if expression is not a valid int64, the client should expect an error. - input dimensions that are not valid int64 values will never match the filter. Checks if the dimension is numerically less than the match expression."]
        NumericLessThan,
        #[serde(rename = "NUMERIC_GREATER_THAN")]
        #[doc = "Checks if the dimension is numerically greater than the match expression."]
        NumericGreaterThan,
        #[serde(rename = "NUMERIC_BETWEEN")]
        #[doc = "Checks if the dimension is numerically between the minimum and maximum of the match expression, boundaries excluded."]
        NumericBetween,
    }
    impl ::std::default::Default for SegmentDimensionFilterOperatorEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SegmentFilter defines the segment to be either a simple or a sequence segment. A simple segment condition contains dimension and metric conditions to select the sessions or users. A sequence segment condition can be used to select users or sessions based on sequential conditions."]
    pub struct SegmentFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "not")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, match the complement of simple or sequence segment. For example, to match all visits not from \"New York\", we can define the segment as follows: \"sessionSegment\": { \"segmentFilters\": [{ \"simpleSegment\" :{ \"orFiltersForSegment\": [{ \"segmentFilterClauses\":[{ \"dimensionFilter\": { \"dimensionName\": \"ga:city\", \"expressions\": [\"New York\"] } }] }] }, \"not\": \"True\" }] },"]
        pub not: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sequenceSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sequence conditions consist of one or more steps, where each step is defined by one or more dimension/metric conditions. Multiple steps can be combined with special sequence operators."]
        pub sequence_segment: ::std::option::Option<::std::boxed::Box<SequenceSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Simple segment conditions consist of one or more dimension/metric conditions that can be combined"]
        pub simple_segment: ::std::option::Option<::std::boxed::Box<SimpleSegment>>,
    }
    impl SegmentFilter {
        pub fn builder() -> SegmentFilterBuilder {
            SegmentFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Filter Clause to be used in a segment definition, can be wither a metric or a dimension filter."]
    pub struct SegmentFilterClause {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension Filter for the segment definition."]
        pub dimension_filter: ::std::option::Option<::std::boxed::Box<SegmentDimensionFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metric Filter for the segment definition."]
        pub metric_filter: ::std::option::Option<::std::boxed::Box<SegmentMetricFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "not")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches the complement (`!`) of the filter."]
        pub not: ::std::option::Option<::std::primitive::bool>,
    }
    impl SegmentFilterClause {
        pub fn builder() -> SegmentFilterClauseBuilder {
            SegmentFilterClauseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metric filter to be used in a segment filter clause."]
    pub struct SegmentMetricFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to compare against. If the operator is `BETWEEN`, this value is treated as minimum comparison value."]
        pub comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxComparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max comparison value is only used for `BETWEEN` operator."]
        pub max_comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric that will be filtered on. A `metricFilter` must contain a metric name."]
        pub metric_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies is the operation to perform to compare the metric. The default is `EQUAL`."]
        pub operator: ::std::option::Option<SegmentMetricFilterOperatorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scope for a metric defines the level at which that metric is defined. The specified metric scope must be equal to or greater than its primary scope as defined in the data model. The primary scope is defined by if the segment is selecting users or sessions."]
        pub scope: ::std::option::Option<SegmentMetricFilterScopeEnum>,
    }
    impl SegmentMetricFilter {
        pub fn builder() -> SegmentMetricFilterBuilder {
            SegmentMetricFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies is the operation to perform to compare the metric. The default is `EQUAL`."]
    pub enum SegmentMetricFilterOperatorEnum {
        #[serde(rename = "UNSPECIFIED_OPERATOR")]
        #[doc = "Unspecified operator is treated as `LESS_THAN` operator."]
        UnspecifiedOperator,
        #[serde(rename = "LESS_THAN")]
        #[doc = "Checks if the metric value is less than comparison value."]
        LessThan,
        #[serde(rename = "GREATER_THAN")]
        #[doc = "Checks if the metric value is greater than comparison value."]
        GreaterThan,
        #[serde(rename = "EQUAL")]
        #[doc = "Equals operator."]
        Equal,
        #[serde(rename = "BETWEEN")]
        #[doc = "For between operator, both the minimum and maximum are exclusive. We will use `LT` and `GT` for comparison."]
        Between,
    }
    impl ::std::default::Default for SegmentMetricFilterOperatorEnum {
        fn default() -> Self {
            Self::UnspecifiedOperator
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Scope for a metric defines the level at which that metric is defined. The specified metric scope must be equal to or greater than its primary scope as defined in the data model. The primary scope is defined by if the segment is selecting users or sessions."]
    pub enum SegmentMetricFilterScopeEnum {
        #[serde(rename = "UNSPECIFIED_SCOPE")]
        #[doc = "If the scope is unspecified, it defaults to the condition scope, `USER` or `SESSION` depending on if the segment is trying to choose users or sessions."]
        UnspecifiedScope,
        #[serde(rename = "PRODUCT")]
        #[doc = "Product scope."]
        Product,
        #[serde(rename = "HIT")]
        #[doc = "Hit scope."]
        Hit,
        #[serde(rename = "SESSION")]
        #[doc = "Session scope."]
        Session,
        #[serde(rename = "USER")]
        #[doc = "User scope."]
        User,
    }
    impl ::std::default::Default for SegmentMetricFilterScopeEnum {
        fn default() -> Self {
            Self::UnspecifiedScope
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A segment sequence definition."]
    pub struct SegmentSequenceStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies if the step immediately precedes or can be any time before the next step."]
        pub match_type: ::std::option::Option<SegmentSequenceStepMatchTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orFiltersForSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sequence is specified with a list of Or grouped filters which are combined with `AND` operator."]
        pub or_filters_for_segment:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrFiltersForSegment>>>,
    }
    impl SegmentSequenceStep {
        pub fn builder() -> SegmentSequenceStepBuilder {
            SegmentSequenceStepBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies if the step immediately precedes or can be any time before the next step."]
    pub enum SegmentSequenceStepMatchTypeEnum {
        #[serde(rename = "UNSPECIFIED_MATCH_TYPE")]
        #[doc = "Unspecified match type is treated as precedes."]
        UnspecifiedMatchType,
        #[serde(rename = "PRECEDES")]
        #[doc = "Operator indicates that the previous step precedes the next step."]
        Precedes,
        #[serde(rename = "IMMEDIATELY_PRECEDES")]
        #[doc = "Operator indicates that the previous step immediately precedes the next step."]
        ImmediatelyPrecedes,
    }
    impl ::std::default::Default for SegmentSequenceStepMatchTypeEnum {
        fn default() -> Self {
            Self::UnspecifiedMatchType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sequence conditions consist of one or more steps, where each step is defined by one or more dimension/metric conditions. Multiple steps can be combined with special sequence operators."]
    pub struct SequenceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstStepShouldMatchFirstHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, first step condition must match the first hit of the visitor (in the date range)."]
        pub first_step_should_match_first_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentSequenceSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of steps in the sequence."]
        pub segment_sequence_steps:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SegmentSequenceStep>>>,
    }
    impl SequenceSegment {
        pub fn builder() -> SequenceSegmentBuilder {
            SequenceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Simple segment conditions consist of one or more dimension/metric conditions that can be combined."]
    pub struct SimpleSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orFiltersForSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of segment filters groups which are combined with logical `AND` operator."]
        pub or_filters_for_segment:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrFiltersForSegment>>>,
    }
    impl SimpleSegment {
        pub fn builder() -> SimpleSegmentBuilder {
            SimpleSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents details collected when the visitor performs a transaction on the page."]
    pub struct TransactionData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction ID, supplied by the e-commerce tracking method, for the purchase in the shopping cart."]
        pub transaction_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionRevenue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total sale revenue (excluding shipping and tax) of the transaction."]
        pub transaction_revenue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionShipping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total cost of shipping."]
        pub transaction_shipping: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total tax for the transaction."]
        pub transaction_tax: ::std::option::Option<::std::primitive::f64>,
    }
    impl TransactionData {
        pub fn builder() -> TransactionDataBuilder {
            TransactionDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information to identify a particular user uniquely."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the user in the request. The field `userId` is associated with this type."]
        pub _type: ::std::option::Option<UserTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique Id of the user for which the data is being requested."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the user in the request. The field `userId` is associated with this type."]
    pub enum UserTypeEnum {
        #[serde(rename = "USER_ID_TYPE_UNSPECIFIED")]
        #[doc = "When the User Id Type is not specified, the default type used will be CLIENT_ID."]
        UserIdTypeUnspecified,
        #[serde(rename = "USER_ID")]
        #[doc = "A single user, like a signed-in user account, that may interact with content across one or more devices and / or browser instances."]
        UserId,
        #[serde(rename = "CLIENT_ID")]
        #[doc = "Analytics assigned client_id."]
        ClientId,
    }
    impl ::std::default::Default for UserTypeEnum {
        fn default() -> Self {
            Self::UserIdTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This represents a user session performed on a specific device at a certain time over a period of time."]
    pub struct UserActivitySession {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a detailed view into each of the activity in this session."]
        pub activities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Activity>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source of a hit. By default, hits sent from analytics.js are reported as \"web\" and hits sent from the mobile SDKs are reported as \"app\". These values can be overridden in the Measurement Protocol."]
        pub data_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of device used: \"mobile\", \"tablet\" etc."]
        pub device_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform on which the activity happened: \"android\", \"ios\" etc."]
        pub platform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of this session in ISO-8601 format."]
        pub session_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID of the session."]
        pub session_id: ::std::option::Option<::std::string::String>,
    }
    impl UserActivitySession {
        pub fn builder() -> UserActivitySessionBuilder {
            UserActivitySessionBuilder::default()
        }
    }
}
