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
    pub mod account_reports {
        pub mod methods {
            pub mod search {
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
                    #[serde(rename = "endDate.day")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub end_date_day: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endDate.month")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub end_date_month: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endDate.year")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub end_date_year: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `next_page_token` value returned from a previous request to SearchAccountReports that indicates where listing should continue. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Account Report for Manager with id 123. | Required."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.day")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub start_date_day: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.month")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub start_date_month: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.year")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub start_date_year: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod detailed_lead_reports {
        pub mod methods {
            pub mod search {
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
                    #[serde(rename = "endDate.day")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub end_date_day: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endDate.month")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub end_date_month: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endDate.year")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub end_date_year: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `next_page_token` value returned from a previous request to SearchDetailedLeadReports that indicates where listing should continue. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Detailed Lead Report for Manager with id | | | 123. | Required."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.day")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub start_date_day: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.month")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub start_date_month: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate.year")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub start_date_year: ::std::option::Option<::std::primitive::i64>,
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
    #[doc = "An Account Report of a GLS account identified by their account id containing aggregate data gathered from a particular date range."]
    pub struct GoogleAdsHomeservicesLocalservicesV1AccountReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of the GLS account."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregatorInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregator specific information related to the account."]
        pub aggregator_info: ::std::option::Option<
            ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averageFiveStarRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Average review rating score from 1-5 stars."]
        pub average_five_star_rating: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averageWeeklyBudget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Average weekly budget in the currency code of the account."]
        pub average_weekly_budget: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "businessName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Business name of the account."]
        pub business_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currency code of the account."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPeriodChargedLeads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of charged leads the account received in current specified period."]
        pub current_period_charged_leads: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPeriodConnectedPhoneCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of connected phone calls (duration over 30s) in current specified period."]
        pub current_period_connected_phone_calls: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPeriodPhoneCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of phone calls in current specified period, including both connected and unconnected calls."]
        pub current_period_phone_calls: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPeriodTotalCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total cost of the account in current specified period in the account's specified currency."]
        pub current_period_total_cost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneLeadResponsiveness")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Phone lead responsiveness of the account for the past 90 days from current date. This is computed by taking the total number of connected calls from charged phone leads and dividing by the total number of calls received."]
        pub phone_lead_responsiveness: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousPeriodChargedLeads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of charged leads the account received in previous specified period."]
        pub previous_period_charged_leads: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousPeriodConnectedPhoneCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of connected phone calls (duration over 30s) in previous specified period."]
        pub previous_period_connected_phone_calls: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousPeriodPhoneCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of phone calls in previous specified period, including both connected and unconnected calls."]
        pub previous_period_phone_calls: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousPeriodTotalCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total cost of the account in previous specified period in the account's specified currency."]
        pub previous_period_total_cost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalReview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of reviews the account has up to current date."]
        pub total_review: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1AccountReport {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1AccountReportBuilder {
            GoogleAdsHomeservicesLocalservicesV1AccountReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Conatiner for aggregator specific information if lead is for an aggregator GLS account."]
    pub struct GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregatorProviderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provider id (listed in aggregator system) which maps to a account id in GLS system."]
        pub aggregator_provider_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1AggregatorInfoBuilder {
            GoogleAdsHomeservicesLocalservicesV1AggregatorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Detailed Lead Report of a lead identified by their lead id and contains consumer, account, monetization, and lead data."]
    pub struct GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies account that received the lead."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregatorInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregator specific information related to the lead."]
        pub aggregator_info: ::std::option::Option<
            ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "businessName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Business name associated to the account."]
        pub business_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chargeStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the lead has been charged."]
        pub charge_status: ::std::option::Option<
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currency code."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disputeStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dispute status related to the lead."]
        pub dispute_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the associated account's home city."]
        pub geo: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leadCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lead category (e.g. hvac, plumber)"]
        pub lead_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leadCreationTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of when the lead was created."]
        pub lead_creation_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leadId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of a Detailed Lead Report."]
        pub lead_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leadPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the lead (available only after it has been charged)."]
        pub lead_price: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leadType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lead type."]
        pub lead_type: ::std::option::Option<
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageLead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information associated to only message leads."]
        pub message_lead: ::std::option::Option<
            ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1MessageLead>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneLead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information associated to only phone leads."]
        pub phone_lead:
            ::std::option::Option<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1PhoneLead>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timezone of the particular provider associated to a lead."]
        pub timezone: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeZone>>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportBuilder {
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the lead has been charged."]
    pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum {
        #[serde(rename = "CHARGE_STATUS_UNSPECIFIED")]
        #[doc = "Not specified."]
        ChargeStatusUnspecified,
        #[serde(rename = "CHARGED")]
        #[doc = "Charged."]
        Charged,
        #[serde(rename = "NOT_CHARGED")]
        #[doc = "Not charged."]
        NotCharged,
    }
    impl ::std::default::Default
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum
    {
        fn default() -> Self {
            Self::ChargeStatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Lead type."]
    pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum {
        #[serde(rename = "LEAD_TYPE_UNSPECIFIED")]
        #[doc = "Not specified."]
        LeadTypeUnspecified,
        #[serde(rename = "MESSAGE")]
        #[doc = "Message lead."]
        Message,
        #[serde(rename = "PHONE_CALL")]
        #[doc = "Phone call lead."]
        PhoneCall,
    }
    impl ::std::default::Default
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum
    {
        fn default() -> Self {
            Self::LeadTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container for message lead specific information."]
    pub struct GoogleAdsHomeservicesLocalservicesV1MessageLead {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerPhoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Consumer phone number associated with the message lead."]
        pub consumer_phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the customer who created the lead."]
        pub customer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job type of the specified lead."]
        pub job_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The postal code of the customer who created the lead."]
        pub postal_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1MessageLead {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1MessageLeadBuilder {
            GoogleAdsHomeservicesLocalservicesV1MessageLeadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container for phone lead specific information."]
    pub struct GoogleAdsHomeservicesLocalservicesV1PhoneLead {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chargedCallTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the phone call which resulted in a charged phone lead."]
        pub charged_call_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chargedConnectedCallDurationSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of the charged phone call in seconds."]
        pub charged_connected_call_duration_seconds: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerPhoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Consumer phone number associated with the phone lead."]
        pub consumer_phone_number: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1PhoneLead {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1PhoneLeadBuilder {
            GoogleAdsHomeservicesLocalservicesV1PhoneLeadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A page of the response received from the SearchAccountReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page."]
    pub struct GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of account reports which maps 1:1 to a particular linked GLS account."]
        pub account_reports: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AccountReport>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
        pub fn builder() -> GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponseBuilder
        {
            GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A page of the response received from the SearchDetailedLeadReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page."]
    pub struct GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detailedLeadReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of detailed lead reports uniquely identified by external lead id."]
        pub detailed_lead_reports: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {
        pub fn builder(
        ) -> GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponseBuilder {
            GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
    pub struct GoogleTypeTimeZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeTimeZone {
        pub fn builder() -> GoogleTypeTimeZoneBuilder {
            GoogleTypeTimeZoneBuilder::default()
        }
    }
}
