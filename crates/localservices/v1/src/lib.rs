#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Account Report of a GLS account identified by their account id containing aggregate data gathered from a particular date range."]
pub struct GoogleAdsHomeservicesLocalservicesV1AccountReport {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of the GLS account."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "aggregatorInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregator specific information related to the account."]
    pub aggregator_info: ::std::option::Option<
        ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
    >,
    #[serde(rename = "averageFiveStarRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average review rating score from 1-5 stars."]
    pub average_five_star_rating: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "averageWeeklyBudget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average weekly budget in the currency code of the account."]
    pub average_weekly_budget: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "businessName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Business name of the account."]
    pub business_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Currency code of the account."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentPeriodChargedLeads")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of charged leads the account received in current specified period."]
    pub current_period_charged_leads: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentPeriodConnectedPhoneCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of connected phone calls (duration over 30s) in current specified period."]
    pub current_period_connected_phone_calls: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentPeriodPhoneCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of phone calls in current specified period, including both connected and unconnected calls."]
    pub current_period_phone_calls: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentPeriodTotalCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total cost of the account in current specified period in the account's specified currency."]
    pub current_period_total_cost: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "phoneLeadResponsiveness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Phone lead responsiveness of the account for the past 90 days from current date. This is computed by taking the total number of connected calls from charged phone leads and dividing by the total number of calls received."]
    pub phone_lead_responsiveness: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "previousPeriodChargedLeads")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of charged leads the account received in previous specified period."]
    pub previous_period_charged_leads: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousPeriodConnectedPhoneCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of connected phone calls (duration over 30s) in previous specified period."]
    pub previous_period_connected_phone_calls: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousPeriodPhoneCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of phone calls in previous specified period, including both connected and unconnected calls."]
    pub previous_period_phone_calls: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousPeriodTotalCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total cost of the account in previous specified period in the account's specified currency."]
    pub previous_period_total_cost: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "totalReview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of reviews the account has up to current date."]
    pub total_review: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Conatiner for aggregator specific information if lead is for an aggregator GLS account."]
pub struct GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
    #[serde(rename = "aggregatorProviderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provider id (listed in aggregator system) which maps to a account id in GLS system."]
    pub aggregator_provider_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Detailed Lead Report of a lead identified by their lead id and contains consumer, account, monetization, and lead data."]
pub struct GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies account that received the lead."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "aggregatorInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregator specific information related to the lead."]
    pub aggregator_info: ::std::option::Option<
        ::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
    >,
    #[serde(rename = "businessName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Business name associated to the account."]
    pub business_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "chargeStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the lead has been charged."]
    pub charge_status: ::std::option::Option<
        GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum,
    >,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Currency code."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disputeStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dispute status related to the lead."]
    pub dispute_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the associated account's home city."]
    pub geo: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leadCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lead category (e.g. hvac, plumber)"]
    pub lead_category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leadCreationTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of when the lead was created."]
    pub lead_creation_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leadId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier of a Detailed Lead Report."]
    pub lead_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "leadPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the lead (available only after it has been charged)."]
    pub lead_price: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "leadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lead type."]
    pub lead_type:
        ::std::option::Option<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum>,
    #[serde(rename = "messageLead")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "More information associated to only message leads."]
    pub message_lead:
        ::std::option::Option<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1MessageLead>>,
    #[serde(rename = "phoneLead")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "More information associated to only phone leads."]
    pub phone_lead:
        ::std::option::Option<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1PhoneLead>>,
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timezone of the particular provider associated to a lead."]
    pub timezone: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeZone>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container for message lead specific information."]
pub struct GoogleAdsHomeservicesLocalservicesV1MessageLead {
    #[serde(rename = "consumerPhoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Consumer phone number associated with the message lead."]
    pub consumer_phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the customer who created the lead."]
    pub customer_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The job type of the specified lead."]
    pub job_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The postal code of the customer who created the lead."]
    pub postal_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container for phone lead specific information."]
pub struct GoogleAdsHomeservicesLocalservicesV1PhoneLead {
    #[serde(rename = "chargedCallTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the phone call which resulted in a charged phone lead."]
    pub charged_call_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "chargedConnectedCallDurationSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the charged phone call in seconds."]
    pub charged_connected_call_duration_seconds: ::std::option::Option<::std::string::String>,
    #[serde(rename = "consumerPhoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Consumer phone number associated with the phone lead."]
    pub consumer_phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A page of the response received from the SearchAccountReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page."]
pub struct GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
    #[serde(rename = "accountReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of account reports which maps 1:1 to a particular linked GLS account."]
    pub account_reports: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1AccountReport>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A page of the response received from the SearchDetailedLeadReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page."]
pub struct GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {
    #[serde(rename = "detailedLeadReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of detailed lead reports uniquely identified by external lead id."]
    pub detailed_lead_reports: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
pub struct GoogleTypeTimeZone {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
    pub version: ::std::option::Option<::std::string::String>,
}
