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
    pub mod billing_accounts {
        pub mod resources {
            pub mod budgets {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of budgets to return per page. The default and maximum value are 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The value returned by the last `ListBudgetsResponse` which indicates that this is a continuation of a prior `ListBudgets` call, and that the system should return the next page of data."]
                            pub page_token: ::std::option::Option<::std::string::String>,
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AllUpdatesRule defines notifications that are sent based on budget spend and thresholds."]
    pub struct GoogleCloudBillingBudgetsV1beta1AllUpdatesRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableDefaultIamRecipients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. When set to true, disables default notifications sent when a threshold is exceeded. Default notifications are sent to those with Billing Account Administrator and Billing Account User IAM roles for the target account."]
        pub disable_default_iam_recipients: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoringNotificationChannels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Targets to send notifications to when a threshold is exceeded. This is in addition to default recipients who have billing account IAM roles. The value is the full REST resource name of a monitoring notification channel with the form `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5 channels are allowed. See https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients for more details."]
        pub monitoring_notification_channels:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the Pub/Sub topic where budget related messages will be published, in the form `projects/{project_id}/topics/{topic_id}`. Updates are sent at regular intervals to the topic. The topic needs to be created before the budget is created; see https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications for more details. Caller is expected to have `pubsub.topics.setIamPolicy` permission on the topic when it's set for a budget, otherwise, the API call will fail with PERMISSION_DENIED. See https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#permissions_required_for_this_task for more details on Pub/Sub roles and permissions."]
        pub pubsub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Required when AllUpdatesRule.pubsub_topic is set. The schema version of the notification sent to AllUpdatesRule.pubsub_topic. Only \"1.0\" is accepted. It represents the JSON schema as defined in https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format."]
        pub schema_version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudBillingBudgetsV1beta1AllUpdatesRule {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1AllUpdatesRuleBuilder {
            GoogleCloudBillingBudgetsV1beta1AllUpdatesRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A budget is a plan that describes what you expect to spend on Cloud projects, plus the rules to execute as spend is tracked against that plan, (for example, send an alert when 90% of the target spend is met). The budget time period is configurable, with options such as month (default), quarter, year, or custom time period."]
    pub struct GoogleCloudBillingBudgetsV1beta1Budget {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allUpdatesRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Rules to apply to notifications sent based on budget spend and thresholds."]
        pub all_updates_rule: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1AllUpdatesRule>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Budgeted amount."]
        pub amount:
            ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1BudgetAmount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "budgetFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Filters that define which resources are used to compute the actual spend against the budget."]
        pub budget_filter:
            ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1Filter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User data for display name in UI. Validation: <= 60 chars."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag will cause an update to overwrite other changes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thresholdRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget."]
        pub threshold_rules: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1ThresholdRule>>,
        >,
    }
    impl GoogleCloudBillingBudgetsV1beta1Budget {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1BudgetBuilder {
            GoogleCloudBillingBudgetsV1beta1BudgetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The budgeted amount for each usage period."]
    pub struct GoogleCloudBillingBudgetsV1beta1BudgetAmount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastPeriodAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use the last period's actual spend as the budget for the present period. Cannot be set in combination with Filter.custom_period."]
        pub last_period_amount: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1LastPeriodAmount>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "specifiedAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A specified amount to use as the budget. `currency_code` is optional. If specified when creating a budget, it must match the currency of the billing account. If specified when updating a budget, it must match the currency_code of the existing budget. The `currency_code` is provided on output."]
        pub specified_amount: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
    }
    impl GoogleCloudBillingBudgetsV1beta1BudgetAmount {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1BudgetAmountBuilder {
            GoogleCloudBillingBudgetsV1beta1BudgetAmountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for CreateBudget"]
    pub struct GoogleCloudBillingBudgetsV1beta1CreateBudgetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "budget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Budget to create."]
        pub budget:
            ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1Budget>>,
    }
    impl GoogleCloudBillingBudgetsV1beta1CreateBudgetRequest {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1CreateBudgetRequestBuilder {
            GoogleCloudBillingBudgetsV1beta1CreateBudgetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All date times begin at 12 AM US and Canadian Pacific Time (UTC-8)."]
    pub struct GoogleCloudBillingBudgetsV1beta1CustomPeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The end date of the time period. If unset, specifies to track all usage incurred since the start_date."]
        pub end_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The start date must be after January 1, 2017."]
        pub start_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
    }
    impl GoogleCloudBillingBudgetsV1beta1CustomPeriod {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1CustomPeriodBuilder {
            GoogleCloudBillingBudgetsV1beta1CustomPeriodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter for a budget, limiting the scope of the cost to calculate."]
    pub struct GoogleCloudBillingBudgetsV1beta1Filter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendarPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies to track usage for recurring calendar period. E.g. Assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when current calendar month is July, August, September, and so on."]
        pub calendar_period:
            ::std::option::Option<GoogleCloudBillingBudgetsV1beta1FilterCalendarPeriodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creditTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If Filter.credit_types_treatment is INCLUDE_SPECIFIED_CREDITS, this is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. If Filter.credit_types_treatment is **not** INCLUDE_SPECIFIED_CREDITS, this field must be empty. See [a list of acceptable credit type values](https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type)."]
        pub credit_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creditTypesTreatment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`."]
        pub credit_types_treatment:
            ::std::option::Option<GoogleCloudBillingBudgetsV1beta1FilterCreditTypesTreatmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies to track usage from any start date (required) to any end date (optional)."]
        pub custom_period:
            ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1CustomPeriod>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A single label and value pair specifying that usage from only this set of labeled resources should be included in the budget. Currently, multiple entries or multiple values per entry are not allowed. If omitted, the report will include all labeled and unlabeled usage."]
        pub labels: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::vec::Vec<::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A set of projects of the form `projects/{project}`, specifying that usage from only this set of projects should be included in the budget. If omitted, the report will include all usage for the billing account, regardless of which project the usage occurred on. Only zero or one project can be specified currently."]
        pub projects: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A set of services of the form `services/{service_id}`, specifying that usage from only this set of services should be included in the budget. If omitted, the report will include usage for all the services. The service names are available through the Catalog API: https://cloud.google.com/billing/v1/how-tos/catalog-api."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subaccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A set of subaccounts of the form `billingAccounts/{account_id}`, specifying that usage from only this set of subaccounts should be included in the budget. If a subaccount is set to the name of the parent account, usage from the parent account will be included. If omitted, the report will include usage from the parent account and all subaccounts, if they exist."]
        pub subaccounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudBillingBudgetsV1beta1Filter {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1FilterBuilder {
            GoogleCloudBillingBudgetsV1beta1FilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Specifies to track usage for recurring calendar period. E.g. Assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when current calendar month is July, August, September, and so on."]
    pub enum GoogleCloudBillingBudgetsV1beta1FilterCalendarPeriodEnum {
        #[serde(rename = "CALENDAR_PERIOD_UNSPECIFIED")]
        #[doc = ""]
        CalendarPeriodUnspecified,
        #[serde(rename = "MONTH")]
        #[doc = "A month. Month starts on the first day of each month, such as January 1, February 1, March 1, and so on."]
        Month,
        #[serde(rename = "QUARTER")]
        #[doc = "A quarter. Quarters start on dates January 1, April 1, July 1, and October 1 of each year."]
        Quarter,
        #[serde(rename = "YEAR")]
        #[doc = "A year. Year starts on January 1."]
        Year,
    }
    impl ::std::default::Default for GoogleCloudBillingBudgetsV1beta1FilterCalendarPeriodEnum {
        fn default() -> Self {
            Self::CalendarPeriodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`."]
    pub enum GoogleCloudBillingBudgetsV1beta1FilterCreditTypesTreatmentEnum {
        #[serde(rename = "CREDIT_TYPES_TREATMENT_UNSPECIFIED")]
        #[doc = ""]
        CreditTypesTreatmentUnspecified,
        #[serde(rename = "INCLUDE_ALL_CREDITS")]
        #[doc = "All types of credit are subtracted from the gross cost to determine the spend for threshold calculations."]
        IncludeAllCredits,
        #[serde(rename = "EXCLUDE_ALL_CREDITS")]
        #[doc = "All types of credit are added to the net cost to determine the spend for threshold calculations."]
        ExcludeAllCredits,
        #[serde(rename = "INCLUDE_SPECIFIED_CREDITS")]
        #[doc = "Credit types specified in the credit_types field are subtracted from the gross cost to determine the spend for threshold calculations."]
        IncludeSpecifiedCredits,
    }
    impl ::std::default::Default for GoogleCloudBillingBudgetsV1beta1FilterCreditTypesTreatmentEnum {
        fn default() -> Self {
            Self::CreditTypesTreatmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a budget amount targeted to last period's spend. At this time, the amount is automatically 100% of last period's spend; that is, there are no other options yet. Future configuration will be described here (for example, configuring a percentage of last period's spend)."]
    pub struct GoogleCloudBillingBudgetsV1beta1LastPeriodAmount {}
    impl GoogleCloudBillingBudgetsV1beta1LastPeriodAmount {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1LastPeriodAmountBuilder {
            GoogleCloudBillingBudgetsV1beta1LastPeriodAmountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListBudgets"]
    pub struct GoogleCloudBillingBudgetsV1beta1ListBudgetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "budgets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the budgets owned by the requested billing account."]
        pub budgets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1Budget>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there may be more budgets that match the request; this value should be passed in a new `ListBudgetsRequest`."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudBillingBudgetsV1beta1ListBudgetsResponse {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1ListBudgetsResponseBuilder {
            GoogleCloudBillingBudgetsV1beta1ListBudgetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ThresholdRule contains a definition of a threshold which triggers an alert (a notification of a threshold being crossed) to be sent when spend goes above the specified amount. Alerts are automatically e-mailed to users with the Billing Account Administrator role or the Billing Account User role. The thresholds here have no effect on notifications sent to anything configured under `Budget.all_updates_rule`."]
    pub struct GoogleCloudBillingBudgetsV1beta1ThresholdRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spendBasis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set."]
        pub spend_basis:
            ::std::option::Option<GoogleCloudBillingBudgetsV1beta1ThresholdRuleSpendBasisEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thresholdPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Send an alert when this threshold is exceeded. This is a 1.0-based percentage, so 0.5 = 50%. Validation: non-negative number."]
        pub threshold_percent: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudBillingBudgetsV1beta1ThresholdRule {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1ThresholdRuleBuilder {
            GoogleCloudBillingBudgetsV1beta1ThresholdRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set."]
    pub enum GoogleCloudBillingBudgetsV1beta1ThresholdRuleSpendBasisEnum {
        #[serde(rename = "BASIS_UNSPECIFIED")]
        #[doc = "Unspecified threshold basis."]
        BasisUnspecified,
        #[serde(rename = "CURRENT_SPEND")]
        #[doc = "Use current spend as the basis for comparison against the threshold."]
        CurrentSpend,
        #[serde(rename = "FORECASTED_SPEND")]
        #[doc = "Use forecasted spend for the period as the basis for comparison against the threshold. Cannot be set in combination with Filter.custom_period."]
        ForecastedSpend,
    }
    impl ::std::default::Default for GoogleCloudBillingBudgetsV1beta1ThresholdRuleSpendBasisEnum {
        fn default() -> Self {
            Self::BasisUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for UpdateBudget"]
    pub struct GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "budget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The updated budget object. The budget to update is specified by the budget name in the budget."]
        pub budget:
            ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1beta1Budget>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates which fields in the provided budget to update. Read-only fields (such as `name`) cannot be changed. If this is not provided, then only fields with non-default values from the request are updated. See https://developers.google.com/protocol-buffers/docs/proto3#default for more details about default values."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequest {
        pub fn builder() -> GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequestBuilder {
            GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct GoogleTypeDate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleTypeDate {
        pub fn builder() -> GoogleTypeDateBuilder {
            GoogleTypeDateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct GoogleTypeMoney {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeMoney {
        pub fn builder() -> GoogleTypeMoneyBuilder {
            GoogleTypeMoneyBuilder::default()
        }
    }
}
