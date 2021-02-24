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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bin is a discrete portion of data spanning from start to end, or if no end is given, then from start to +inf. A bin's start and end values are given in the value type of the metric it represents. For example, \"first contentful paint\" is measured in milliseconds and exposed as ints, therefore its metric bins will use int32s for its start and end types. However, \"cumulative layout shift\" is measured in unitless decimals and is exposed as a decimal encoded as a string, therefore its metric bins will use strings for its value type."]
    pub struct Bin {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "density")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The proportion of users that experienced this bin's value for the given metric."]
        pub density: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End is the end of the data bin. If end is not populated, then the bin has no end and is valid from start to +inf."]
        pub end: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start is the beginning of the data bin."]
        pub start: ::std::option::Option<::serde_json::Value>,
    }
    impl Bin {
        pub fn builder() -> BinBuilder {
            BinBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Key defines all the dimensions that identify this record as unique."]
    pub struct Key {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveConnectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The effective connection type is the general connection class that all users experienced for this record. This field uses the values [\"offline\", \"slow-2G\", \"2G\", \"3G\", \"4G\"] as specified in: https://wicg.github.io/netinfo/#effective-connection-types If the effective connection type is unspecified, then aggregated data over all effective connection types will be returned."]
        pub effective_connection_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formFactor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The form factor is the device class that all users used to access the site for this record. If the form factor is unspecified, then aggregated data over all form factors will be returned."]
        pub form_factor: ::std::option::Option<KeyFormFactorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "origin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Origin specifies the origin that this record is for. Note: When specifying an origin, data for loads under this origin over all pages are aggregated into origin level user experience data."]
        pub origin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Url specifies a specific url that this record is for. Note: When specifying a \"url\" only data for that specific url will be aggregated."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Key {
        pub fn builder() -> KeyBuilder {
            KeyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The form factor is the device class that all users used to access the site for this record. If the form factor is unspecified, then aggregated data over all form factors will be returned."]
    pub enum KeyFormFactorEnum {
        #[serde(rename = "ALL_FORM_FACTORS")]
        #[doc = "The default value, representing all device classes."]
        AllFormFactors,
        #[serde(rename = "PHONE")]
        #[doc = "The device class representing a \"mobile\"/\"phone\" sized client."]
        Phone,
        #[serde(rename = "DESKTOP")]
        #[doc = "The device class representing a \"desktop\"/\"laptop\" type full size client."]
        Desktop,
        #[serde(rename = "TABLET")]
        #[doc = "The device class representing a \"tablet\" type client."]
        Tablet,
    }
    impl ::std::default::Default for KeyFormFactorEnum {
        fn default() -> Self {
            Self::AllFormFactors
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `metric` is a set of user experience data for a single web performance metric, like \"first contentful paint\". It contains a summary histogram of real world Chrome usage as a series of `bins`."]
    pub struct Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogram")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The histogram of user experiences for a metric. The histogram will have at least one bin and the densities of all bins will add up to ~1."]
        pub histogram: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bin>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common useful percentiles of the Metric. The value type for the percentiles will be the same as the value types given for the Histogram bins."]
        pub percentiles: ::std::option::Option<::std::boxed::Box<Percentiles>>,
    }
    impl Metric {
        pub fn builder() -> MetricBuilder {
            MetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Percentiles contains synthetic values of a metric at a given statistical percentile. These are used for estimating a metric's value as experienced by a percentage of users out of the total number of users."]
    pub struct Percentiles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p75")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "75% of users experienced the given metric at or below this value."]
        pub p75: ::std::option::Option<::serde_json::Value>,
    }
    impl Percentiles {
        pub fn builder() -> PercentilesBuilder {
            PercentilesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request payload sent by a physical web client. This request includes all necessary context to load a particular user experience record."]
    pub struct QueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveConnectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The effective connection type is a query dimension that specifies the effective network class that the record's data should belong to. This field uses the values [\"offline\", \"slow-2G\", \"2G\", \"3G\", \"4G\"] as specified in: https://wicg.github.io/netinfo/#effective-connection-types Note: If no effective connection type is specified, then a special record with aggregated data over all effective connection types will be returned."]
        pub effective_connection_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formFactor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned."]
        pub form_factor: ::std::option::Option<QueryRequestFormFactorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics that should be included in the response. If none are specified then any metrics found will be returned. Allowed values: [\"first_contentful_paint\", \"first_input_delay\", \"largest_contentful_paint\", \"cumulative_layout_shift\"]"]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "origin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url pattern \"origin\" refers to a url pattern that is the origin of a website. Examples: \"https://example.com\", \"https://cloud.google.com\""]
        pub origin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url pattern \"url\" refers to a url pattern that is any arbitrary url. Examples: \"https://example.com/\", \"https://cloud.google.com/why-google-cloud/\""]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl QueryRequest {
        pub fn builder() -> QueryRequestBuilder {
            QueryRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned."]
    pub enum QueryRequestFormFactorEnum {
        #[serde(rename = "ALL_FORM_FACTORS")]
        #[doc = "The default value, representing all device classes."]
        AllFormFactors,
        #[serde(rename = "PHONE")]
        #[doc = "The device class representing a \"mobile\"/\"phone\" sized client."]
        Phone,
        #[serde(rename = "DESKTOP")]
        #[doc = "The device class representing a \"desktop\"/\"laptop\" type full size client."]
        Desktop,
        #[serde(rename = "TABLET")]
        #[doc = "The device class representing a \"tablet\" type client."]
        Tablet,
    }
    impl ::std::default::Default for QueryRequestFormFactorEnum {
        fn default() -> Self {
            Self::AllFormFactors
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response payload sent back to a physical web client. This response contains the record found based on the identiers present in a `QueryRequest`. The returned response will have a record, and sometimes details on normalization actions taken on the request that were necessary to make the request successful."]
    pub struct QueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "record")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The record that was found."]
        pub record: ::std::option::Option<::std::boxed::Box<Record>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlNormalizationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "These are details about automated normalization actions that were taken in order to make the requested `url_pattern` valid."]
        pub url_normalization_details: ::std::option::Option<::std::boxed::Box<UrlNormalization>>,
    }
    impl QueryResponse {
        pub fn builder() -> QueryResponseBuilder {
            QueryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Record is a single Chrome UX report data record. It contains use experience statistics for a single url pattern and set of dimensions."]
    pub struct Record {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key defines all of the unique querying parameters needed to look up a user experience record."]
        pub key: ::std::option::Option<::std::boxed::Box<Key>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metrics is the map of user experience data available for the record defined in the key field. Metrics are keyed on the metric name. Allowed key values: [\"first_contentful_paint\", \"first_input_delay\", \"largest_contentful_paint\", \"cumulative_layout_shift\"]"]
        pub metrics:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Metric>>>,
    }
    impl Record {
        pub fn builder() -> RecordBuilder {
            RecordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Object representing the normalization actions taken to normalize a url to achieve a higher chance of successful lookup. These are simple automated changes that are taken when looking up the provided `url_patten` would be known to fail. Complex actions like following redirects are not handled."]
    pub struct UrlNormalization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL after any normalization actions. This is a valid user experience URL that could reasonably be looked up."]
        pub normalized_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original requested URL prior to any normalization actions."]
        pub original_url: ::std::option::Option<::std::string::String>,
    }
    impl UrlNormalization {
        pub fn builder() -> UrlNormalizationBuilder {
            UrlNormalizationBuilder::default()
        }
    }
}
