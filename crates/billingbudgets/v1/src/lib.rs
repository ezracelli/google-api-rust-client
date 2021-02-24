#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A budget is a plan that describes what you expect to spend on Cloud projects, plus the rules to execute as spend is tracked against that plan, (for example, send an alert when 90% of the target spend is met). The budget time period is configurable, with options such as month (default), quarter, year, or custom time period."]
pub struct GoogleCloudBillingBudgetsV1Budget {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Budgeted amount."]
    pub amount: ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1BudgetAmount>>,
    #[serde(rename = "budgetFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Filters that define which resources are used to compute the actual spend against the budget."]
    pub budget_filter: ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1Filter>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User data for display name in UI. The name must be less than or equal to 60 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag will cause an update to overwrite other changes."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationsRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Rules to apply to notifications sent based on budget spend and thresholds."]
    pub notifications_rule:
        ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1NotificationsRule>>,
    #[serde(rename = "thresholdRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget."]
    pub threshold_rules: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudBillingBudgetsV1ThresholdRule>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The budgeted amount for each usage period."]
pub struct GoogleCloudBillingBudgetsV1BudgetAmount {
    #[serde(rename = "lastPeriodAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use the last period's actual spend as the budget for the present period. Cannot be set in combination with Filter.custom_period."]
    pub last_period_amount:
        ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1LastPeriodAmount>>,
    #[serde(rename = "specifiedAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A specified amount to use as the budget. `currency_code` is optional. If specified when creating a budget, it must match the currency of the billing account. If specified when updating a budget, it must match the currency_code of the existing budget. The `currency_code` is provided on output."]
    pub specified_amount: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All date times begin at 12 AM US and Canadian Pacific Time (UTC-8)."]
pub struct GoogleCloudBillingBudgetsV1CustomPeriod {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The end date of the time period. If unset, specifies to track all usage incurred since the start_date."]
    pub end_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The start date must be after January 1, 2017."]
    pub start_date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter for a budget, limiting the scope of the cost to calculate."]
pub struct GoogleCloudBillingBudgetsV1Filter {
    #[serde(rename = "calendarPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies to track usage for recurring calendar period. E.g. Assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when current calendar month is July, August, September, and so on."]
    pub calendar_period: ::std::option::Option<GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum>,
    #[serde(rename = "creditTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If Filter.credit_types_treatment is INCLUDE_SPECIFIED_CREDITS, this is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. If Filter.credit_types_treatment is **not** INCLUDE_SPECIFIED_CREDITS, this field must be empty. See [a list of acceptable credit type values](https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type)."]
    pub credit_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "creditTypesTreatment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`."]
    pub credit_types_treatment:
        ::std::option::Option<GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum>,
    #[serde(rename = "customPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies to track usage from any start date (required) to any end date (optional)."]
    pub custom_period:
        ::std::option::Option<::std::boxed::Box<GoogleCloudBillingBudgetsV1CustomPeriod>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A single label and value pair specifying that usage from only this set of labeled resources should be included in the budget. Currently, multiple entries or multiple values per entry are not allowed. If omitted, the report will include all labeled and unlabeled usage."]
    pub labels: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::vec::Vec<::serde_json::Value>>,
    >,
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A set of projects of the form `projects/{project}`, specifying that usage from only this set of projects should be included in the budget. If omitted, the report will include all usage for the billing account, regardless of which project the usage occurred on. Only zero or one project can be specified currently."]
    pub projects: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A set of services of the form `services/{service_id}`, specifying that usage from only this set of services should be included in the budget. If omitted, the report will include usage for all the services. The service names are available through the Catalog API: https://cloud.google.com/billing/v1/how-tos/catalog-api."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subaccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A set of subaccounts of the form `billingAccounts/{account_id}`, specifying that usage from only this set of subaccounts should be included in the budget. If a subaccount is set to the name of the parent account, usage from the parent account will be included. If the field is omitted, the report will include usage from the parent account and all subaccounts, if they exist."]
    pub subaccounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Specifies to track usage for recurring calendar period. E.g. Assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when current calendar month is July, August, September, and so on."]
pub enum GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum {
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
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`."]
pub enum GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum {
    #[serde(rename = "CREDIT_TYPES_TREATMENT_UNSPECIFIED")]
    #[doc = "This is an invalid value."]
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a budget amount targeted to last period's spend. At this time, the amount is automatically 100% of last period's spend; that is, there are no other options yet. Future configuration will be described here (for example, configuring a percentage of last period's spend)."]
pub struct GoogleCloudBillingBudgetsV1LastPeriodAmount {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListBudgets"]
pub struct GoogleCloudBillingBudgetsV1ListBudgetsResponse {
    #[serde(rename = "budgets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the budgets owned by the requested billing account."]
    pub budgets: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudBillingBudgetsV1Budget>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more budgets that match the request; this value should be passed in a new `ListBudgetsRequest`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NotificationsRule defines notifications that are sent based on budget spend and thresholds."]
pub struct GoogleCloudBillingBudgetsV1NotificationsRule {
    #[serde(rename = "disableDefaultIamRecipients")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. When set to true, disables default notifications sent when a threshold is exceeded. Default notifications are sent to those with Billing Account Administrator and Billing Account User IAM roles for the target account."]
    pub disable_default_iam_recipients: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "monitoringNotificationChannels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Targets to send notifications to when a threshold is exceeded. This is in addition to default recipients who have billing account IAM roles. The value is the full REST resource name of a monitoring notification channel with the form `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5 channels are allowed. See https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients for more details."]
    pub monitoring_notification_channels:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "pubsubTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the Pub/Sub topic where budget related messages will be published, in the form `projects/{project_id}/topics/{topic_id}`. Updates are sent at regular intervals to the topic. The topic needs to be created before the budget is created; see https://cloud.google.com/billing/docs/how-to/budgets#manage-notifications for more details. Caller is expected to have `pubsub.topics.setIamPolicy` permission on the topic when it's set for a budget, otherwise, the API call will fail with PERMISSION_DENIED. See https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications for more details on Pub/Sub roles and permissions."]
    pub pubsub_topic: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Required when NotificationsRule.pubsub_topic is set. The schema version of the notification sent to NotificationsRule.pubsub_topic. Only \"1.0\" is accepted. It represents the JSON schema as defined in https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format."]
    pub schema_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ThresholdRule contains a definition of a threshold which triggers an alert (a notification of a threshold being crossed) to be sent when spend goes above the specified amount. Alerts are automatically e-mailed to users with the Billing Account Administrator role or the Billing Account User role. The thresholds here have no effect on notifications sent to anything configured under `Budget.all_updates_rule`."]
pub struct GoogleCloudBillingBudgetsV1ThresholdRule {
    #[serde(rename = "spendBasis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set."]
    pub spend_basis: ::std::option::Option<GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum>,
    #[serde(rename = "thresholdPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Send an alert when this threshold is exceeded. This is a 1.0-based percentage, so 0.5 = 50%. Validation: non-negative number."]
    pub threshold_percent: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set."]
pub enum GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum {
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct GoogleTypeDate {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an amount of money with its currency type."]
pub struct GoogleTypeMoney {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The three-letter currency code defined in ISO 4217."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
    pub units: ::std::option::Option<::std::string::String>,
}
