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
    pub mod activities {
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
                    #[serde(rename = "actorIpAddress")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions."]
                    pub actor_ip_address: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID of the customer to retrieve data for."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts: - *Date of the API's request for a report*: When the API created and retrieved the report. - *Report's start time*: The beginning of the timespan shown in the report. The `startTime` must be before the `endTime` (if specified) and the current time when the request is made, or the API returns an error. - *Report's end time*: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August. If the `endTime` is not specified, the report returns all activities from the `startTime` until the current time or the most recent 180 days if the `startTime` is more than 180 days in the past."]
                    pub end_time: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "eventName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of the event being queried by the API. Each `eventName` is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings `type` structure has all of the Calendar `eventName` activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings `type` and `eventName` parameters. For more information about `eventName` query strings and parameters, see the list of event names for various applications above in `applicationName`."]
                    pub event_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `filters` query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form `parameter1 name[parameter1 value],parameter2 name[parameter2 value],...` These event parameters are associated with a specific `eventName`. An empty report is returned if the filtered request's parameter does not belong to the `eventName`. For more information about `eventName` parameters, see the list of event names for various applications above in `applicationName`. In the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E): GET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS In the following Drive example, the list can be a view or edit event's `doc_id` parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's `doc_id`. In the second example, the report returns each viewed document's `doc_id` that equals the value 12345 and does not return any viewed document's which have a `doc_id` value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E): GET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765 The relational operators include: - `==` - 'equal to'. - `<>` - 'not equal to'. It is URL-encoded (%3C%3E). - `<` - 'less than'. It is URL-encoded (%3C). - `<=` - 'less than or equal to'. It is URL-encoded (%3C=). - `>` - 'greater than'. It is URL-encoded (%3E). - `>=` - 'greater than or equal to'. It is URL-encoded (%3E=). *Note:* The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned. "]
                    pub filters: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "groupIdFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma separated group ids (obfuscated) on which user activities are filtered, i.e, the response will contain activities for only those users that are a part of at least one of the group ids mentioned here. Format: \"id:abc123,id:xyz456\""]
                    pub group_id_filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets `maxResults=1` and the report has two activities, the report has two pages. The response's `nextPageToken` property has the token to the second page. The `maxResults` query string is optional in the request. The default value is 1000."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(
                        default = "{ query_parameters_defaults :: org_unit_id () }",
                        setter(into)
                    )]
                    #[serde(rename = "orgUnitID")]
                    #[serde(default = "query_parameters_defaults :: org_unit_id")]
                    #[doc = "ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
                    pub org_unit_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. In your follow-on request getting the next page of the report, enter the `nextPageToken` value in the `pageToken` query string."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from `startTime` until `endTime`. The `startTime` must be before the `endTime` (if specified) and the current time when the request is made, or the API returns an error."]
                    pub start_time: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"1000").unwrap()
                    }
                    pub fn org_unit_id() -> ::std::string::String {
                        serde_json::from_str(&"\"\"").unwrap()
                    }
                }
            }
            pub mod watch {
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
                    #[serde(rename = "actorIpAddress")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions."]
                    pub actor_ip_address: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID of the customer to retrieve data for."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts: - *Date of the API's request for a report*: When the API created and retrieved the report. - *Report's start time*: The beginning of the timespan shown in the report. The `startTime` must be before the `endTime` (if specified) and the current time when the request is made, or the API returns an error. - *Report's end time*: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August. If the `endTime` is not specified, the report returns all activities from the `startTime` until the current time or the most recent 180 days if the `startTime` is more than 180 days in the past."]
                    pub end_time: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "eventName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of the event being queried by the API. Each `eventName` is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings `type` structure has all of the Calendar `eventName` activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings `type` and `eventName` parameters. For more information about `eventName` query strings and parameters, see the list of event names for various applications above in `applicationName`."]
                    pub event_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `filters` query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form `parameter1 name[parameter1 value],parameter2 name[parameter2 value],...` These event parameters are associated with a specific `eventName`. An empty report is returned if the filtered request's parameter does not belong to the `eventName`. For more information about `eventName` parameters, see the list of event names for various applications above in `applicationName`. In the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E): GET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS In the following Drive example, the list can be a view or edit event's `doc_id` parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's `doc_id`. In the second example, the report returns each viewed document's `doc_id` that equals the value 12345 and does not return any viewed document's which have a `doc_id` value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E): GET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765 The relational operators include: - `==` - 'equal to'. - `<>` - 'not equal to'. It is URL-encoded (%3C%3E). - `<` - 'less than'. It is URL-encoded (%3C). - `<=` - 'less than or equal to'. It is URL-encoded (%3C=). - `>` - 'greater than'. It is URL-encoded (%3E). - `>=` - 'greater than or equal to'. It is URL-encoded (%3E=). *Note:* The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned. "]
                    pub filters: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "groupIdFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma separated group ids (obfuscated) on which user activities are filtered, i.e, the response will contain activities for only those users that are a part of at least one of the group ids mentioned here. Format: \"id:abc123,id:xyz456\""]
                    pub group_id_filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets `maxResults=1` and the report has two activities, the report has two pages. The response's `nextPageToken` property has the token to the second page. The `maxResults` query string is optional in the request. The default value is 1000."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(
                        default = "{ query_parameters_defaults :: org_unit_id () }",
                        setter(into)
                    )]
                    #[serde(rename = "orgUnitID")]
                    #[serde(default = "query_parameters_defaults :: org_unit_id")]
                    #[doc = "ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
                    pub org_unit_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. In your follow-on request getting the next page of the report, enter the `nextPageToken` value in the `pageToken` query string."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from `startTime` until `endTime`. The `startTime` must be before the `endTime` (if specified) and the current time when the request is made, or the API returns an error."]
                    pub start_time: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"1000").unwrap()
                    }
                    pub fn org_unit_id() -> ::std::string::String {
                        serde_json::from_str(&"\"\"").unwrap()
                    }
                }
            }
        }
    }
    pub mod customer_usage_reports {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID of the customer to retrieve data for."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parameters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `parameters` query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers usage report include `accounts`, `app_maker`, `apps_scripts`, `calendar`, `classroom`, `cros`, `docs`, `gmail`, `gplus`, `device_management`, `meet`, and `sites`. A `parameters` query string is in the CSV form of `app_name1:param_name1, app_name2:param_name2`. *Note:* The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned. "]
                    pub parameters: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod entity_usage_reports {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID of the customer to retrieve data for."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `filters` query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The `filters` query string includes the name of the application whose usage is returned in the report. The application values for the Entities usage report include `accounts`, `docs`, and `gmail`. Filters are in the form `[application name]:parameter name[parameter value],...`. In this example, the `<>` 'not equal to' operator is URL-encoded in the request's query string (%3C%3E): GET https://www.googleapis.com/admin/reports/v1/usage/gplus_communities/all/dates/2017-12-01 ?parameters=gplus:community_name,gplus:num_total_members &filters=gplus:num_total_members%3C%3E0 The relational operators include: - `==` - 'equal to'. - `<>` - 'not equal to'. It is URL-encoded (%3C%3E). - `<` - 'less than'. It is URL-encoded (%3C). - `<=` - 'less than or equal to'. It is URL-encoded (%3C=). - `>` - 'greater than'. It is URL-encoded (%3E). - `>=` - 'greater than or equal to'. It is URL-encoded (%3E=). Filters can only be applied to numeric parameters."]
                    pub filters: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets `maxResults=1` and the report has two activities, the report has two pages. The response's `nextPageToken` property has the token to the second page."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. In your follow-on request getting the next page of the report, enter the `nextPageToken` value in the `pageToken` query string."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parameters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `parameters` query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Entities usage report are only `gplus`. A `parameter` query string is in the CSV form of `[app_name1:param_name1], [app_name2:param_name2]...`. *Note:* The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned. "]
                    pub parameters: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"1000").unwrap()
                    }
                }
            }
        }
    }
    pub mod user_usage_report {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID of the customer to retrieve data for."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `filters` query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The `filters` query string includes the name of the application whose usage is returned in the report. The application values for the Users Usage Report include `accounts`, `docs`, and `gmail`. Filters are in the form `[application name]:parameter name[parameter value],...`. In this example, the `<>` 'not equal to' operator is URL-encoded in the request's query string (%3C%3E): GET https://www.googleapis.com/admin/reports/v1/usage/users/all/dates/2013-03-03 ?parameters=accounts:last_login_time &filters=accounts:last_login_time%3C%3E2010-10-28T10:26:35.000Z The relational operators include: - `==` - 'equal to'. - `<>` - 'not equal to'. It is URL-encoded (%3C%3E). - `<` - 'less than'. It is URL-encoded (%3C). - `<=` - 'less than or equal to'. It is URL-encoded (%3C=). - `>` - 'greater than'. It is URL-encoded (%3E). - `>=` - 'greater than or equal to'. It is URL-encoded (%3E=). "]
                    pub filters: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "groupIdFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma separated group ids (obfuscated) on which user activities are filtered, i.e, the response will contain activities for only those users that are a part of at least one of the group ids mentioned here. Format: \"id:abc123,id:xyz456\""]
                    pub group_id_filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets `maxResults=1` and the report has two activities, the report has two pages. The response's `nextPageToken` property has the token to the second page. The `maxResults` query string is optional."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(
                        default = "{ query_parameters_defaults :: org_unit_id () }",
                        setter(into)
                    )]
                    #[serde(rename = "orgUnitID")]
                    #[serde(default = "query_parameters_defaults :: org_unit_id")]
                    #[doc = "ID of the organizational unit to report on. User activity will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
                    pub org_unit_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. In your follow-on request getting the next page of the report, enter the `nextPageToken` value in the `pageToken` query string."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parameters")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `parameters` query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers Usage report include `accounts`, `app_maker`, `apps_scripts`, `calendar`, `classroom`, `cros`, `docs`, `gmail`, `gplus`, `device_management`, `meet`, and `sites`. A `parameters` query string is in the CSV form of `app_name1:param_name1, app_name2:param_name2`. *Note:* The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned. "]
                    pub parameters: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"1000").unwrap()
                    }
                    pub fn org_unit_id() -> ::std::string::String {
                        serde_json::from_str(&"\"\"").unwrap()
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
    #[doc = "JSON template for a collection of activities."]
    pub struct Activities {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each activity record in the response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Activity>>>,
        #[builder(default = "{ activities_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "activities_defaults :: kind")]
        #[doc = "The type of API resource. For an activity report, the value is `reports#activities`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token for retrieving the follow-on next page of the report. The `nextPageToken` value is used in the request's `pageToken` query string."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Activities {
        pub fn builder() -> ActivitiesBuilder {
            ActivitiesBuilder::default()
        }
    }
    mod activities_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#reports#activities\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for the activity resource."]
    pub struct Activity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User doing the action."]
        pub actor: ::std::option::Option<ActivityActor>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the entry."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Activity events in the report."]
        pub events: ::std::option::Option<::std::vec::Vec<ActivityEvents>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for each activity record."]
        pub id: ::std::option::Option<ActivityId>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IP address of the user doing the action. This is the Internet Protocol (IP) address of the user when logging into G Suite which may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. The API supports IPv4 and IPv6."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ activity_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "activity_defaults :: kind")]
        #[doc = "The type of API resource. For an activity report, the value is `audit#activity`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the domain that is affected by the report's event. For example domain of Admin console or the Drive application's document owner."]
        pub owner_domain: ::std::option::Option<::std::string::String>,
    }
    impl Activity {
        pub fn builder() -> ActivityBuilder {
            ActivityBuilder::default()
        }
    }
    mod activity_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#reports#activity\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User doing the action."]
    pub struct ActivityActor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of actor."]
        pub caller_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary email address of the actor. May be absent if there is no email address associated with the actor."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only present when `callerType` is `KEY`. Can be the `consumer_key` of the requestor for OAuth 2LO API requests or an identifier for robot accounts."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique G Suite profile ID of the actor. May be absent if the actor is not a G Suite user."]
        pub profile_id: ::std::option::Option<::std::string::String>,
    }
    impl ActivityActor {
        pub fn builder() -> ActivityActorBuilder {
            ActivityActorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ActivityEvents {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the event. This is the specific name of the activity reported by the API. And each `eventName` is related to a specific G Suite service or feature which the API organizes into types of events. For `eventName` request parameters in general: - If no `eventName` is given, the report returns all possible instances of an `eventName`. - When you request an `eventName`, the API's response returns all activities which contain that `eventName`. It is possible that the returned activities will have other `eventName` properties in addition to the one requested. For more information about `eventName` properties, see the list of event names for various applications above in `applicationName`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter value pairs for various applications. For more information about `eventName` parameters, see the list of event names for various applications above in `applicationName`."]
        pub parameters: ::std::option::Option<::std::vec::Vec<ActivityEventsParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of event. The G Suite service or feature that an administrator changes is identified in the `type` property which identifies an event using the `eventName` property. For a full list of the API's `type` categories, see the list of event names for various applications above in `applicationName`."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ActivityEvents {
        pub fn builder() -> ActivityEventsBuilder {
            ActivityEventsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ActivityEventsParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean value of the parameter."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer value of the parameter."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as `[{parameter: [{name: city, value: abc}]}]`"]
        pub message_value: ::std::option::Option<ActivityEventsParametersMessageValue>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiIntValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer values of the parameter."]
        pub multi_int_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiMessageValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of `messageValue` objects."]
        pub multi_message_value:
            ::std::option::Option<::std::vec::Vec<ActivityEventsParametersMultiMessageValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String values of the parameter."]
        pub multi_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String value of the parameter."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ActivityEventsParameters {
        pub fn builder() -> ActivityEventsParametersBuilder {
            ActivityEventsParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as `[{parameter: [{name: city, value: abc}]}]`"]
    pub struct ActivityEventsParametersMessageValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter values"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestedParameter>>>,
    }
    impl ActivityEventsParametersMessageValue {
        pub fn builder() -> ActivityEventsParametersMessageValueBuilder {
            ActivityEventsParametersMessageValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ActivityEventsParametersMultiMessageValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter values"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestedParameter>>>,
    }
    impl ActivityEventsParametersMultiMessageValue {
        pub fn builder() -> ActivityEventsParametersMultiMessageValueBuilder {
            ActivityEventsParametersMultiMessageValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Unique identifier for each activity record."]
    pub struct ActivityId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application name to which the event belongs. For possible values see the list of applications above in `applicationName`."]
        pub application_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier for a G suite account."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of occurrence of the activity. This is in UNIX epoch time in seconds."]
        pub time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueQualifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique qualifier if multiple events have the same time."]
        pub unique_qualifier: ::std::option::Option<::std::string::String>,
    }
    impl ActivityId {
        pub fn builder() -> ActivityIdBuilder {
            ActivityIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A notification channel used to watch for resource changes."]
    pub struct Channel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address where notifications are delivered for this channel."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A UUID or similar unique string that identifies this channel."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ channel_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "channel_defaults :: kind")]
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"`api#channel`\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
        pub payload: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A version-specific identifier for the watched resource."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
        pub token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of delivery mechanism used for this channel. The value should be set to `\"web_hook\"`."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Channel {
        pub fn builder() -> ChannelBuilder {
            ChannelBuilder::default()
        }
    }
    mod channel_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"api#channel\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a parameter used in various reports."]
    pub struct NestedParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean value of the parameter."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer value of the parameter."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiBoolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Multiple boolean values of the parameter."]
        pub multi_bool_value: ::std::option::Option<::std::vec::Vec<::std::primitive::bool>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiIntValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Multiple integer values of the parameter."]
        pub multi_int_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Multiple string values of the parameter."]
        pub multi_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String value of the parameter."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl NestedParameter {
        pub fn builder() -> NestedParameterBuilder {
            NestedParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a usage report."]
    pub struct UsageReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The date of the report request."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Information about the type of the item."]
        pub entity: ::std::option::Option<UsageReportEntity>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ usage_report_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "usage_report_defaults :: kind")]
        #[doc = "The type of API resource. For a usage report, the value is `admin#reports#usageReport`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Parameter value pairs for various applications. For the Entity Usage Report parameters and values, see [the Entity Usage parameters reference](/admin-sdk/reports/v1/reference/usage-ref-appendix-a/entities)."]
        pub parameters: ::std::option::Option<::std::vec::Vec<UsageReportParameters>>,
    }
    impl UsageReport {
        pub fn builder() -> UsageReportBuilder {
            UsageReportBuilder::default()
        }
    }
    mod usage_report_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#reports#usageReport\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Information about the type of the item."]
    pub struct UsageReportEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the customer's account."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Object key. Only relevant if entity.type = \"OBJECT\" Note: external-facing name of report is \"Entities\" rather than \"Objects\"."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The user's immutable G Suite profile identifier."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of item. The value is `user`."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The user's email address. Only relevant if entity.type = \"USER\""]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl UsageReportEntity {
        pub fn builder() -> UsageReportEntityBuilder {
            UsageReportEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsageReportParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Boolean value of the parameter."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datetimeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RFC 3339 formatted value of the parameter, for example 2010-10-28T10:26:35.000Z."]
        pub datetime_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Integer value of the parameter."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "msgValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Nested message value of the parameter."]
        pub msg_value: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the parameter. For the User Usage Report parameter names, see the User Usage parameters reference."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. String value of the parameter."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl UsageReportParameters {
        pub fn builder() -> UsageReportParametersBuilder {
            UsageReportParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsageReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ usage_reports_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "usage_reports_defaults :: kind")]
        #[doc = "The type of API resource. For a usage report, the value is `admin#reports#usageReports`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usageReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Various application parameter records."]
        pub usage_reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsageReport>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Warnings, if any."]
        pub warnings: ::std::option::Option<::std::vec::Vec<UsageReportsWarnings>>,
    }
    impl UsageReports {
        pub fn builder() -> UsageReportsBuilder {
            UsageReportsBuilder::default()
        }
    }
    mod usage_reports_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#reports#usageReports\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsageReportsWarnings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine readable code or warning type. The warning code value is `200`."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs to give detailed information on the warning."]
        pub data: ::std::option::Option<::std::vec::Vec<UsageReportsWarningsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human readable messages for a warning are: - Data is not available warning - Sorry, data for date yyyy-mm-dd for application \"`application name`\" is not available. - Partial data is available warning - Data for date yyyy-mm-dd for application \"`application name`\" is not available right now, please try again after a few hours. "]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl UsageReportsWarnings {
        pub fn builder() -> UsageReportsWarningsBuilder {
            UsageReportsWarningsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsageReportsWarningsData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key associated with a key-value pair to give detailed information on the warning."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value associated with a key-value pair to give detailed information on the warning."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UsageReportsWarningsData {
        pub fn builder() -> UsageReportsWarningsDataBuilder {
            UsageReportsWarningsDataBuilder::default()
        }
    }
}
