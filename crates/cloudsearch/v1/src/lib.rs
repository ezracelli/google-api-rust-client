#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for boolean properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct BooleanOperatorOptions {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the boolean property. For example, if operatorName is *closed* and the property's name is *isClosed*, then queries like *closed:<value>* show results only where the value of the property named *isClosed* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for boolean properties."]
pub struct BooleanPropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the boolean should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<BooleanOperatorOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckAccessResponse {
    #[serde(rename = "hasAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns true if principal has access. Returns false otherwise."]
    pub has_access: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompositeFilter {
    #[serde(rename = "logicOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The logic operator of the sub filter."]
    pub logic_operator: ::std::option::Option<CompositeFilterLogicOperatorEnum>,
    #[serde(rename = "subFilters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub filters."]
    pub sub_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Filter>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The logic operator of the sub filter."]
pub enum CompositeFilterLogicOperatorEnum {
    #[serde(rename = "AND")]
    #[doc = "Logical operators, which can only be applied to sub filters."]
    And,
    #[serde(rename = "OR")]
    #[doc = ""]
    Or,
    #[serde(rename = "NOT")]
    #[doc = "NOT can only be applied on a single sub filter."]
    Not,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Aggregation of items by status code as of the specified date."]
pub struct CustomerIndexStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which statistics were calculated."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "itemCountByStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items aggregrated by status code."]
    pub item_count_by_status:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ItemCountByStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomerQueryStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which query stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "queryCountByStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub query_count_by_status:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryCountByStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomerSessionStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "searchSessionsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of search sessions on the day"]
    pub search_sessions_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomerUserStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "oneDayActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past one day"]
    pub one_day_active_users_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sevenDaysActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past seven days"]
    pub seven_days_active_users_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirtyDaysActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past thirty days"]
    pub thirty_days_active_users_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Datasource is a logical namespace for items to be indexed. All items must belong to a datasource. This is the prerequisite before items can be indexed into Cloud Search."]
pub struct DataSource {
    #[serde(rename = "disableModifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, sets the datasource to read-only mode. In read-only mode, the Indexing API rejects any requests to index or delete items in this source. Enabling read-only mode does not stop the processing of previously accepted data."]
    pub disable_modifications: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "disableServing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Disable serving any search or assist results."]
    pub disable_serving: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name of the datasource The maximum length is 300 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "indexingServiceAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of service accounts that have indexing access."]
    pub indexing_service_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "itemsVisibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field restricts visibility to items at the datasource level. Items within the datasource are restricted to the union of users and groups included in this field. Note that, this does not ensure access to a specific item, as users need to have ACL permissions on the contained items. This ensures a high level access on the entire datasource, and that the individual items are not shared outside this visibility."]
    pub items_visibility:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GSuitePrincipal>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the datasource resource. Format: datasources/{source_id}. The name is ignored when creating a datasource."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of the Long Running Operations (LROs) currently running for this schema."]
    pub operation_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short name or alias for the source. This value will be used to match the 'source' operator. For example, if the short name is *<value>* then queries like *source:<value>* will only return results for this source. The value must be unique across all datasources. The value must only contain alphanumeric characters (a-zA-Z0-9). The value cannot start with 'google' and cannot be one of the following: mail, gmail, docs, drive, groups, sites, calendar, hangouts, gplus, keep, people, teams. Its maximum length is 32 characters."]
    pub short_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Aggregation of items by status code as of the specified date."]
pub struct DataSourceIndexStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which index stats were calculated. If the date of request is not the current date then stats calculated on the next day are returned. Stats are calculated close to mid night in this case. If date of request is current date, then real time stats are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "itemCountByStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items aggregrated by status code."]
    pub item_count_by_status:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ItemCountByStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Restriction on Datasource."]
pub struct DataSourceRestriction {
    #[serde(rename = "filterOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter options restricting the results. If multiple filters are present, they are grouped by object type before joining. Filters with the same object type are joined conjunctively, then the resulting expressions are joined disjunctively. The maximum number of elements is 20. NOTE: Suggest API supports only few filters at the moment: \"objecttype\", \"type\" and \"mimetype\". For now, schema specific filters cannot be used to filter suggestions."]
    pub filter_options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterOptions>>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of restriction."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole calendar date, for example a date of birth. The time of day and time zone are either specified elsewhere or are not significant. The date is relative to the [Proleptic Gregorian Calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar). The date must be a valid calendar date between the year 1 and 9999."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of date. Must be from 1 to 12."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of date. Must be from 1 to 9999."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Optional. Provides a search operator for date properties. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct DateOperatorOptions {
    #[serde(rename = "greaterThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the date property using the greater-than operator. For example, if greaterThanOperatorName is *closedafter* and the property's name is *closeDate*, then queries like *closedafter:<value>* show results only where the value of the property named *closeDate* is later than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub greater_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lessThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the date property using the less-than operator. For example, if lessThanOperatorName is *closedbefore* and the property's name is *closeDate*, then queries like *closedbefore:<value>* show results only where the value of the property named *closeDate* is earlier than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub less_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the actual string required in the query in order to isolate the date property. For example, suppose an issue tracking schema object has a property named *closeDate* that specifies an operator with an operatorName of *closedon*. For searches on that data, queries like *closedon:<value>* show results only where the value of the *closeDate* property matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the indexed datasource. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for date properties."]
pub struct DatePropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the date should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<DateOperatorOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of date values."]
pub struct DateValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Date>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shared request debug options for all cloudsearch RPC methods."]
pub struct DebugOptions {
    #[serde(rename = "enableDebugging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
    pub enable_debugging: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteQueueItemsRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a queue to delete items from."]
    pub queue: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a top-level property within the object that should be displayed in search results. The values of the chosen properties is displayed in the search results along with the display label for that property if one is specified. If a display label is not specified, only the values is shown."]
pub struct DisplayedProperty {
    #[serde(rename = "propertyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the top-level property as defined in a property definition for the object. If the name is not a defined property in the schema, an error is given when attempting to update the schema."]
    pub property_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for double properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct DoubleOperatorOptions {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to use the double property in sorting or as a facet. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for double properties."]
pub struct DoublePropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the double should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<DoubleOperatorOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of double values."]
pub struct DoubleValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Drive follow-up search restricts (e.g. \"followup:suggestions\")."]
pub struct DriveFollowUpRestrict {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<DriveFollowUpRestrictTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DriveFollowUpRestrictTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "FOLLOWUP_SUGGESTIONS")]
    #[doc = ""]
    FollowupSuggestions,
    #[serde(rename = "FOLLOWUP_ACTION_ITEMS")]
    #[doc = ""]
    FollowupActionItems,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Drive location search restricts (e.g. \"is:starred\")."]
pub struct DriveLocationRestrict {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<DriveLocationRestrictTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DriveLocationRestrictTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "TRASHED")]
    #[doc = ""]
    Trashed,
    #[serde(rename = "STARRED")]
    #[doc = ""]
    Starred,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Drive mime-type search restricts (e.g. \"type:pdf\")."]
pub struct DriveMimeTypeRestrict {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<DriveMimeTypeRestrictTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DriveMimeTypeRestrictTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "PDF")]
    #[doc = ""]
    Pdf,
    #[serde(rename = "DOCUMENT")]
    #[doc = ""]
    Document,
    #[serde(rename = "PRESENTATION")]
    #[doc = ""]
    Presentation,
    #[serde(rename = "SPREADSHEET")]
    #[doc = ""]
    Spreadsheet,
    #[serde(rename = "FORM")]
    #[doc = ""]
    Form,
    #[serde(rename = "DRAWING")]
    #[doc = ""]
    Drawing,
    #[serde(rename = "SCRIPT")]
    #[doc = ""]
    Script,
    #[serde(rename = "MAP")]
    #[doc = ""]
    Map,
    #[serde(rename = "IMAGE")]
    #[doc = ""]
    Image,
    #[serde(rename = "AUDIO")]
    #[doc = ""]
    Audio,
    #[serde(rename = "VIDEO")]
    #[doc = ""]
    Video,
    #[serde(rename = "FOLDER")]
    #[doc = ""]
    Folder,
    #[serde(rename = "ARCHIVE")]
    #[doc = ""]
    Archive,
    #[serde(rename = "SITE")]
    #[doc = ""]
    Site,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The time span search restrict (e.g. \"after:2017-09-11 before:2017-09-12\")."]
pub struct DriveTimeSpanRestrict {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<DriveTimeSpanRestrictTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DriveTimeSpanRestrictTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "TODAY")]
    #[doc = ""]
    Today,
    #[serde(rename = "YESTERDAY")]
    #[doc = ""]
    Yesterday,
    #[serde(rename = "LAST_7_DAYS")]
    #[doc = ""]
    Last7Days,
    #[serde(rename = "LAST_30_DAYS")]
    #[doc = "Not Enabled"]
    Last30Days,
    #[serde(rename = "LAST_90_DAYS")]
    #[doc = "Not Enabled"]
    Last90Days,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's email address."]
pub struct EmailAddress {
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address."]
    pub email_address: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for enum properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched. For example, if you provide no operator for a *priority* enum property with possible values *p0* and *p1*, a query that contains the term *p0* returns items that have *p0* as the value of the *priority* property, as well as any items that contain the string *p0* in other fields. If you provide an operator name for the enum, such as *priority*, then search users can use that operator to refine results to only items that have *p0* as this property's value, with the query *priority:p0*."]
pub struct EnumOperatorOptions {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the enum property. For example, if operatorName is *priority* and the property's name is *priorityVal*, then queries like *priority:<value>* show results only where the value of the property named *priorityVal* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for enum properties, which allow you to define a restricted set of strings to match user queries, set rankings for those string values, and define an operator name to be paired with those strings so that users can narrow results to only items with a specific value. For example, for items in a request tracking system with priority information, you could define *p0* as an allowable enum value and tie this enum to the operator name *priority* so that search users could add *priority:p0* to their query to restrict the set of results to only those items indexed with the value *p0*."]
pub struct EnumPropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the enum should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<EnumOperatorOptions>>,
    #[serde(rename = "orderedRanking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to specify the ordered ranking for the enumeration that determines how the integer values provided in the possible EnumValuePairs are used to rank results. If specified, integer values must be provided for all possible EnumValuePair values given for this property. Can only be used if isRepeatable is false."]
    pub ordered_ranking: ::std::option::Option<EnumPropertyOptionsOrderedRankingEnum>,
    #[serde(rename = "possibleValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of possible values for the enumeration property. All EnumValuePairs must provide a string value. If you specify an integer value for one EnumValuePair, then all possible EnumValuePairs must provide an integer value. Both the string value and integer value must be unique over all possible values. Once set, possible values cannot be removed or modified. If you supply an ordered ranking and think you might insert additional enum values in the future, leave gaps in the initial integer values to allow adding a value in between previously registered values. The maximum number of elements is 100."]
    pub possible_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnumValuePair>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Used to specify the ordered ranking for the enumeration that determines how the integer values provided in the possible EnumValuePairs are used to rank results. If specified, integer values must be provided for all possible EnumValuePair values given for this property. Can only be used if isRepeatable is false."]
pub enum EnumPropertyOptionsOrderedRankingEnum {
    #[serde(rename = "NO_ORDER")]
    #[doc = "There is no ranking order for the property. Results aren't adjusted by this property's value."]
    NoOrder,
    #[serde(rename = "ASCENDING")]
    #[doc = "This property is ranked in ascending order. Lower values indicate lower ranking."]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = "This property is ranked in descending order. Lower values indicate higher ranking."]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The enumeration value pair defines two things: a required string value and an optional integer value. The string value defines the necessary query term required to retrieve that item, such as *p0* for a priority item. The integer value determines the ranking of that string value relative to other enumerated values for the same property. For example, you might associate *p0* with *0* and define another enum pair such as *p1* and *1*. You must use the integer value in combination with ordered ranking to set the ranking of a given value relative to other enumerated values for the same property name. Here, a ranking order of DESCENDING for *priority* properties results in a ranking boost for items indexed with a value of *p0* compared to items indexed with a value of *p1*. Without a specified ranking order, the integer value has no effect on item ranking."]
pub struct EnumValuePair {
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The integer value of the EnumValuePair which must be non-negative. Optional."]
    pub integer_value: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value of the EnumValuePair. The maximum length is 32 characters."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of enum values."]
pub struct EnumValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum allowable length for string values is 32 characters."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error information about the response."]
pub struct ErrorInfo {
    #[serde(rename = "errorMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorMessage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error message per source response."]
pub struct ErrorMessage {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A bucket in a facet is the basic unit of operation. A bucket can comprise either a single value OR a contiguous range of values, depending on the type of the field bucketed. FacetBucket is currently used only for returning the response object."]
pub struct FacetBucket {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of results that match the bucket value. Counts are only returned for searches when count accuracy is ensured. Can be empty."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "percentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percent of results that match the bucket value. The returned value is between (0-100], and is rounded down to an integer if fractional. If the value is not explicitly returned, it represents a percentage value that rounds to 0. Percentages are returned for all searches, but are an estimate. Because percentages are always returned, you should render percentages instead of counts."]
    pub percentage: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::boxed::Box<Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies operators to return facet results for. There will be one FacetResult for every source_name/object_type/operator_name combination."]
pub struct FacetOptions {
    #[serde(rename = "numFacetBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of facet buckets that should be returned for this facet. Defaults to 10. Maximum value is 100."]
    pub num_facet_buckets: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If object_type is set, only those objects of that type will be used to compute facets. If empty, then all objects will be used to compute facets."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the operator chosen for faceting. @see cloudsearch.SchemaPropertyOptions"]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source name to facet on. Format: datasources/{source_id} If empty, all data sources will be used."]
    pub source_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source specific facet response"]
pub struct FacetResult {
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "FacetBuckets for values in response containing at least a single result."]
    pub buckets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FacetBucket>>>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object type for which facet results are returned. Can be empty."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the operator chosen for faceting. @see cloudsearch.SchemaPropertyOptions"]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source name for which facet results are returned. Will not be empty."]
    pub source_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldViolation {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the error."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Path of field with violation."]
    pub field: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic way of expressing filters in a query, which supports two approaches: **1. Setting a ValueFilter.** The name must match an operator_name defined in the schema for your data source. **2. Setting a CompositeFilter.** The filters are evaluated using the logical operator. The top-level operators can only be either an AND or a NOT. AND can appear only at the top-most level. OR can appear only under a top-level AND."]
pub struct Filter {
    #[serde(rename = "compositeFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub composite_filter: ::std::option::Option<::std::boxed::Box<CompositeFilter>>,
    #[serde(rename = "valueFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value_filter: ::std::option::Option<::std::boxed::Box<ValueFilter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Filter options to be applied on query."]
pub struct FilterOptions {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Generic filter to restrict the search, such as `lang:en`, `site:xyz`."]
    pub filter: ::std::option::Option<::std::boxed::Box<Filter>>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If object_type is set, only objects of that type are returned. This should correspond to the name of the object that was registered within the definition of schema. The maximum length is 256 characters."]
    pub object_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Indicates which freshness property to use when adjusting search ranking for an item. Fresher, more recent dates indicate higher quality. Use the freshness option property that best works with your data. For fileshare documents, last modified time is most relevant. For calendar event data, the time when the event occurs is a more relevant freshness indicator. In this way, calendar events that occur closer to the time of the search query are considered higher quality and ranked accordingly."]
pub struct FreshnessOptions {
    #[serde(rename = "freshnessDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration after which an object should be considered stale. The default value is 180 days (in seconds)."]
    pub freshness_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "freshnessProperty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This property indicates the freshness level of the object in the index. If set, this property must be a top-level property within the property definitions and it must be a timestamp type or date type. Otherwise, the Indexing API uses updateTime as the freshness indicator. The maximum length is 256 characters. When a property is used to calculate freshness, the value defaults to 2 years from the current time."]
    pub freshness_property: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GSuitePrincipal {
    #[serde(rename = "gsuiteDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal represents all users of the G Suite domain of the customer."]
    pub gsuite_domain: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "gsuiteGroupEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal references a G Suite group account"]
    pub gsuite_group_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gsuiteUserEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal references a G Suite user account"]
    pub gsuite_user_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetCustomerIndexStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Summary of indexed item counts, one for each day in the requested range."]
    pub stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomerIndexStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetCustomerQueryStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomerQueryStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetCustomerSessionStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomerSessionStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetCustomerUserStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomerUserStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetDataSourceIndexStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Summary of indexed item counts, one for each day in the requested range."]
    pub stats: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceIndexStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetSearchApplicationQueryStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchApplicationQueryStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetSearchApplicationSessionStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchApplicationSessionStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetSearchApplicationUserStatsResponse {
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchApplicationUserStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for html properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct HtmlOperatorOptions {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the html property. For example, if operatorName is *subject* and the property's name is *subjectLine*, then queries like *subject:<value>* show results only where the value of the property named *subjectLine* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator return all items where *<value>* matches the value of any html properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for html properties."]
pub struct HtmlPropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the property should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<HtmlOperatorOptions>>,
    #[serde(rename = "retrievalImportance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the search quality importance of the tokens within the field when used for retrieval. Can only be set to DEFAULT or NONE."]
    pub retrieval_importance: ::std::option::Option<::std::boxed::Box<RetrievalImportance>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of html values."]
pub struct HtmlValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum allowable length for html values is 2048 characters."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexItemOptions {
    #[serde(rename = "allowUnknownGsuitePrincipals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies if the index request should allow gsuite principals that do not exist or are deleted in the index request."]
    pub allow_unknown_gsuite_principals: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexItemRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "indexItemOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub index_item_options: ::std::option::Option<::std::boxed::Box<IndexItemOptions>>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the item. Format: datasources/{source_id}/items/{item_id}"]
    pub item: ::std::option::Option<::std::boxed::Box<Item>>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The RequestMode for this request."]
    pub mode: ::std::option::Option<IndexItemRequestModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The RequestMode for this request."]
pub enum IndexItemRequestModeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Priority is not specified in the update request. Leaving priority unspecified results in an update failure."]
    Unspecified,
    #[serde(rename = "SYNCHRONOUS")]
    #[doc = "For real-time updates."]
    Synchronous,
    #[serde(rename = "ASYNCHRONOUS")]
    #[doc = "For changes that are executed after the response is sent back to the caller."]
    Asynchronous,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for integer properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct IntegerOperatorOptions {
    #[serde(rename = "greaterThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the integer property using the greater-than operator. For example, if greaterThanOperatorName is *priorityabove* and the property's name is *priorityVal*, then queries like *priorityabove:<value>* show results only where the value of the property named *priorityVal* is greater than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub greater_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lessThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the integer property using the less-than operator. For example, if lessThanOperatorName is *prioritybelow* and the property's name is *priorityVal*, then queries like *prioritybelow:<value>* show results only where the value of the property named *priorityVal* is less than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub less_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the integer property. For example, if operatorName is *priority* and the property's name is *priorityVal*, then queries like *priority:<value>* show results only where the value of the property named *priorityVal* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for integer properties."]
pub struct IntegerPropertyOptions {
    #[serde(rename = "maximumValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum value of the property. The minimum and maximum values for the property are used to rank results according to the ordered ranking. Indexing requests with values greater than the maximum are accepted and ranked with the same weight as items indexed with the maximum value."]
    pub maximum_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum value of the property. The minimum and maximum values for the property are used to rank results according to the ordered ranking. Indexing requests with values less than the minimum are accepted and ranked with the same weight as items indexed with the minimum value."]
    pub minimum_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the integer should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<IntegerOperatorOptions>>,
    #[serde(rename = "orderedRanking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to specify the ordered ranking for the integer. Can only be used if isRepeatable is false."]
    pub ordered_ranking: ::std::option::Option<IntegerPropertyOptionsOrderedRankingEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Used to specify the ordered ranking for the integer. Can only be used if isRepeatable is false."]
pub enum IntegerPropertyOptionsOrderedRankingEnum {
    #[serde(rename = "NO_ORDER")]
    #[doc = "There is no ranking order for the property. Results are not adjusted by this property's value."]
    NoOrder,
    #[serde(rename = "ASCENDING")]
    #[doc = "This property is ranked in ascending order. Lower values indicate lower ranking."]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = "This property is ranked in descending order. Lower values indicate higher ranking."]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of integer values."]
pub struct IntegerValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an interaction between a user and an item."]
pub struct Interaction {
    #[serde(rename = "interactionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the user acted on the item. If multiple actions of the same type exist for a single user, only the most recent action is recorded."]
    pub interaction_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user that acted on the item."]
    pub principal: ::std::option::Option<::std::boxed::Box<Principal>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<InteractionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum InteractionTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Invalid value."]
    Unspecified,
    #[serde(rename = "VIEW")]
    #[doc = "This interaction indicates the user viewed the item."]
    View,
    #[serde(rename = "EDIT")]
    #[doc = "This interaction indicates the user edited the item."]
    Edit,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single object that is an item in the search index, such as a file, folder, or a database record."]
pub struct Item {
    #[serde(rename = "acl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access control list for this item."]
    pub acl: ::std::option::Option<::std::boxed::Box<ItemAcl>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Item content to be indexed and made text searchable."]
    pub content: ::std::option::Option<::std::boxed::Box<ItemContent>>,
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type for this item."]
    pub item_type: ::std::option::Option<ItemItemTypeEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata information."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ItemMetadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the Item. Format: datasources/{source_id}/items/{item_id} This is a required field. The maximum length is 1536 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional state connector can store for this item. The maximum length is 10000 bytes."]
    pub payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Queue this item belongs to. The maximum length is 100 characters."]
    pub queue: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the item. Output only field."]
    pub status: ::std::option::Option<::std::boxed::Box<ItemStatus>>,
    #[serde(rename = "structuredData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The structured data for the item that should conform to a registered object definition in the schema for the data source."]
    pub structured_data: ::std::option::Option<::std::boxed::Box<ItemStructuredData>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The indexing system stores the version from the datasource as a byte string and compares the Item version in the index to the version of the queued Item using lexical ordering. Cloud Search Indexing won't index or delete any queued item with a version value that is less than or equal to the version of the currently indexed item. The maximum length for this field is 1024 bytes."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type for this item."]
pub enum ItemItemTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "CONTENT_ITEM")]
    #[doc = "An item that is indexed for the only purpose of serving information. These items cannot be referred in containerName or inheritAclFrom fields."]
    ContentItem,
    #[serde(rename = "CONTAINER_ITEM")]
    #[doc = "An item that gets indexed and whose purpose is to supply other items with ACLs and/or contain other items."]
    ContainerItem,
    #[serde(rename = "VIRTUAL_CONTAINER_ITEM")]
    #[doc = "An item that does not get indexed, but otherwise has the same purpose as CONTAINER_ITEM."]
    VirtualContainerItem,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Access control list information for the item. For more information see [Map ACLs](/cloud-search/docs/guides/acls)."]
pub struct ItemAcl {
    #[serde(rename = "aclInheritanceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets the type of access rules to apply when an item inherits its ACL from a parent. This should always be set in tandem with the inheritAclFrom field. Also, when the inheritAclFrom field is set, this field should be set to a valid AclInheritanceType."]
    pub acl_inheritance_type: ::std::option::Option<ItemAclAclInheritanceTypeEnum>,
    #[serde(rename = "deniedReaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of principals who are explicitly denied access to the item in search results. While principals are denied access by default, use denied readers to handle exceptions and override the list allowed readers. The maximum number of elements is 100."]
    pub denied_readers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Principal>>>,
    #[serde(rename = "inheritAclFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the item to inherit the Access Permission List (ACL) from. Note: ACL inheritance *only* provides access permissions to child items and does not define structural relationships, nor does it provide convenient ways to delete large groups of items. Deleting an ACL parent from the index only alters the access permissions of child items that reference the parent in the inheritAclFrom field. The item is still in the index, but may not visible in search results. By contrast, deletion of a container item also deletes all items that reference the container via the containerName field. The maximum length for this field is 1536 characters."]
    pub inherit_acl_from: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of owners for the item. This field has no bearing on document access permissions. It does, however, offer a slight ranking boosts items where the querying user is an owner. The maximum number of elements is 5."]
    pub owners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Principal>>>,
    #[serde(rename = "readers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of principals who are allowed to see the item in search results. Optional if inheriting permissions from another item or if the item is not intended to be visible, such as virtual containers. The maximum number of elements is 1000."]
    pub readers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Principal>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Sets the type of access rules to apply when an item inherits its ACL from a parent. This should always be set in tandem with the inheritAclFrom field. Also, when the inheritAclFrom field is set, this field should be set to a valid AclInheritanceType."]
pub enum ItemAclAclInheritanceTypeEnum {
    #[serde(rename = "NOT_APPLICABLE")]
    #[doc = "The default value when this item does not inherit an ACL. Use NOT_APPLICABLE when inheritAclFrom is empty. An item without ACL inheritance can still have ACLs supplied by its own readers and deniedReaders fields."]
    NotApplicable,
    #[serde(rename = "CHILD_OVERRIDE")]
    #[doc = "During an authorization conflict, the ACL of the child item determines its read access."]
    ChildOverride,
    #[serde(rename = "PARENT_OVERRIDE")]
    #[doc = "During an authorization conflict, the ACL of the parent item specified in the inheritAclFrom field determines read access."]
    ParentOverride,
    #[serde(rename = "BOTH_PERMIT")]
    #[doc = "Access is granted only if this item and the parent item specified in the inheritAclFrom field both permit read access."]
    BothPermit,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Content of an item to be indexed and surfaced by Cloud Search. Only UTF-8 encoded strings are allowed as inlineContent. If the content is uploaded and not binary, it must be UTF-8 encoded."]
pub struct ItemContent {
    #[serde(rename = "contentDataRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload reference ID of a previously uploaded content via write method."]
    pub content_data_ref: ::std::option::Option<::std::boxed::Box<UploadItemRef>>,
    #[serde(rename = "contentFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content_format: ::std::option::Option<ItemContentContentFormatEnum>,
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hashing info calculated and provided by the API client for content. Can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inlineContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content that is supplied inlined within the update method. The maximum length is 102400 bytes (100 KiB)."]
    pub inline_content: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ItemContentContentFormatEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Invalid value."]
    Unspecified,
    #[serde(rename = "HTML")]
    #[doc = "contentFormat is HTML."]
    Html,
    #[serde(rename = "TEXT")]
    #[doc = "contentFormat is free text."]
    Text,
    #[serde(rename = "RAW")]
    #[doc = "contentFormat is raw bytes."]
    Raw,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemCountByStatus {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items matching the status code."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the items."]
    pub status_code: ::std::option::Option<ItemCountByStatusStatusCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the items."]
pub enum ItemCountByStatusStatusCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
    CodeUnspecified,
    #[serde(rename = "ERROR")]
    #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
    Error,
    #[serde(rename = "MODIFIED")]
    #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
    Modified,
    #[serde(rename = "NEW_ITEM")]
    #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
    NewItem,
    #[serde(rename = "ACCEPTED")]
    #[doc = "API has accepted the up-to-date data of this item."]
    Accepted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Available metadata fields for the item."]
pub struct ItemMetadata {
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the container for this item. Deletion of the container item leads to automatic deletion of this item. Note: ACLs are not inherited from a container item. To provide ACL inheritance for an item, use the inheritAclFrom field. The maximum length is 1536 characters."]
    pub container_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP-47 language code for the item, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. The maximum length is 32 characters."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the item was created in the source repository."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hashing value provided by the API caller. This can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "interactions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of interactions for the item. Interactions are used to improve Search quality, but are not exposed to end users. The maximum number of elements is 1000."]
    pub interactions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Interaction>>>,
    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional keywords or phrases that should match the item. Used internally for user generated content. The maximum number of elements is 100. The maximum length is 8192 characters."]
    pub keywords: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original mime-type of ItemContent.content in the source repository. The maximum length is 256 characters."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the item. This should correspond to the name of an object definition in the schema registered for the data source. For example, if the schema for the data source contains an object definition with name 'document', then item indexing requests for objects of that type should set objectType to 'document'. The maximum length is 256 characters."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchQualityMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional search quality metadata of the item"]
    pub search_quality_metadata: ::std::option::Option<::std::boxed::Box<SearchQualityMetadata>>,
    #[serde(rename = "sourceRepositoryUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to the source repository serving the data. Search results apply this link to the title. Whitespace or special characters may cause Cloud Search result links to trigger a redirect notice; to avoid this, encode the URL. The maximum length is 2048 characters."]
    pub source_repository_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the item. If given, this will be the displayed title of the Search result. The maximum length is 2048 characters."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the item was last modified in the source repository."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This contains item's status and any errors."]
pub struct ItemStatus {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status code."]
    pub code: ::std::option::Option<ItemStatusCodeEnum>,
    #[serde(rename = "processingErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error details in case the item is in ERROR state."]
    pub processing_errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProcessingError>>>,
    #[serde(rename = "repositoryErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Repository error reported by connector."]
    pub repository_errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RepositoryError>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status code."]
pub enum ItemStatusCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
    CodeUnspecified,
    #[serde(rename = "ERROR")]
    #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
    Error,
    #[serde(rename = "MODIFIED")]
    #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
    Modified,
    #[serde(rename = "NEW_ITEM")]
    #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
    NewItem,
    #[serde(rename = "ACCEPTED")]
    #[doc = "API has accepted the up-to-date data of this item."]
    Accepted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Available structured data fields for the item."]
pub struct ItemStructuredData {
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hashing value provided by the API caller. This can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The structured data object that should conform to a registered object definition in the schema for the data source."]
    pub object: ::std::option::Option<::std::boxed::Box<StructuredDataObject>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListDataSourceResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListItemNamesForUnmappedIdentityResponse {
    #[serde(rename = "itemNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub item_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListItemsResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Item>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct ListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List sources response."]
pub struct ListQuerySourcesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QuerySource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSearchApplicationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchApplications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub search_applications:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchApplication>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnmappedIdentitiesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unmappedIdentities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub unmapped_identities:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnmappedIdentity>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Matched range of a snippet [start, end)."]
pub struct MatchRange {
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End of the match in the snippet."]
    pub end: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Starting position of the match in the snippet."]
    pub start: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Media resource."]
pub struct Media {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the media resource."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata of a matched search result."]
pub struct Metadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time for this document or object in the search result."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options that specify how to display a structured data search result."]
    pub display_options: ::std::option::Option<::std::boxed::Box<ResultDisplayMetadata>>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indexed fields in structured data, returned as a generic named property."]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedProperty>>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mime type of the search result."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object type of the search result."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Owner (usually creator) of the document or object of the search result."]
    pub owner: ::std::option::Option<::std::boxed::Box<Person>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The named source for the result, such as Gmail."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last modified date for the object in the search result. If not set in the item, the value returned here is empty. When `updateTime` is used for calculating freshness and is not set, this value defaults to 2 years from the current time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A metaline is a list of properties that are displayed along with the search result to provide context."]
pub struct Metaline {
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of displayed properties for the metaline. The maximum number of properties is 5."]
    pub properties: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DisplayedProperty>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's name."]
pub struct Name {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read-only display name formatted according to the locale specified by the viewer's account or the Accept-Language HTTP header."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A typed name-value pair for structured data. The type of the value should be the same as the registered type for the `name` property in the object definition of `objectType`."]
pub struct NamedProperty {
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub boolean_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dateValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub date_values: ::std::option::Option<::std::boxed::Box<DateValues>>,
    #[serde(rename = "doubleValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub double_values: ::std::option::Option<::std::boxed::Box<DoubleValues>>,
    #[serde(rename = "enumValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enum_values: ::std::option::Option<::std::boxed::Box<EnumValues>>,
    #[serde(rename = "htmlValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub html_values: ::std::option::Option<::std::boxed::Box<HtmlValues>>,
    #[serde(rename = "integerValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integer_values: ::std::option::Option<::std::boxed::Box<IntegerValues>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the property. This name should correspond to the name of the property that was registered for object definition in the schema. The maximum allowable length for this property is 256 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object_values: ::std::option::Option<::std::boxed::Box<ObjectValues>>,
    #[serde(rename = "textValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_values: ::std::option::Option<::std::boxed::Box<TextValues>>,
    #[serde(rename = "timestampValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp_values: ::std::option::Option<::std::boxed::Box<TimestampValues>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The definition for an object within a data source."]
pub struct ObjectDefinition {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name for the object, which then defines its type. Item indexing requests should set the objectType field equal to this value. For example, if *name* is *Document*, then indexing requests for items of type Document should set objectType equal to *Document*. Each object definition must be uniquely named within a schema. The name must start with a letter and can only contain letters (A-Z, a-z) or numbers (0-9). The maximum length is 256 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The optional object-specific options."]
    pub options: ::std::option::Option<::std::boxed::Box<ObjectOptions>>,
    #[serde(rename = "propertyDefinitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property definitions for the object. The maximum number of elements is 1000."]
    pub property_definitions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyDefinition>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The display options for an object."]
pub struct ObjectDisplayOptions {
    #[serde(rename = "metalines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the properties that are displayed in the metalines of the search results. The property values are displayed in the order given here. If a property holds multiple values, all of the values are displayed before the next properties. For this reason, it is a good practice to specify singular properties before repeated properties in this list. All of the properties must set is_returnable to true. The maximum number of metalines is 3."]
    pub metalines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metaline>>>,
    #[serde(rename = "objectDisplayLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user friendly label to display in the search result to indicate the type of the item. This is OPTIONAL; if not provided, an object label isn't displayed on the context line of the search results. The maximum length is 64 characters."]
    pub object_display_label: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for an object."]
pub struct ObjectOptions {
    #[serde(rename = "displayOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options that determine how the object is displayed in the Cloud Search results page."]
    pub display_options: ::std::option::Option<::std::boxed::Box<ObjectDisplayOptions>>,
    #[serde(rename = "freshnessOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The freshness options for an object."]
    pub freshness_options: ::std::option::Option<::std::boxed::Box<FreshnessOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for object properties."]
pub struct ObjectPropertyOptions {
    #[serde(rename = "subobjectProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties of the sub-object. These properties represent a nested object. For example, if this property represents a postal address, the subobjectProperties might be named *street*, *city*, and *state*. The maximum number of elements is 1000."]
    pub subobject_properties:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyDefinition>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of object values."]
pub struct ObjectValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuredDataObject>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This field contains information about the person being suggested."]
pub struct PeopleSuggestion {
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested person. All fields of the person object might not be populated."]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Object to represent a person."]
pub struct Person {
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's email addresses"]
    pub email_addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EmailAddress>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the person to provide information about. See People.get from Google People API."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "obfuscatedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Obfuscated ID of a person."]
    pub obfuscated_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "personNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The person's name"]
    pub person_names: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Name>>>,
    #[serde(rename = "photos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A person's read-only photo. A picture shown next to the person's name to help others recognize the person in search results."]
    pub photos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Photo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A person's photo."]
pub struct Photo {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the photo."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PollItemsRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of items to return. The maximum value is 100 and the default value is 20."]
    pub limit: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Queue name to fetch items from. If unspecified, PollItems will fetch from 'default' queue. The maximum length is 100 characters."]
    pub queue: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Limit the items polled to the ones with these statuses."]
    pub status_codes: ::std::option::Option<::std::vec::Vec<PollItemsRequestStatusCodesEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum PollItemsRequestStatusCodesEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
    CodeUnspecified,
    #[serde(rename = "ERROR")]
    #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
    Error,
    #[serde(rename = "MODIFIED")]
    #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
    Modified,
    #[serde(rename = "NEW_ITEM")]
    #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
    NewItem,
    #[serde(rename = "ACCEPTED")]
    #[doc = "API has accepted the up-to-date data of this item."]
    Accepted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PollItemsResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of items from the queue available for connector to process. These items have the following subset of fields populated: version metadata.hash structured_data.hash content.hash payload status queue"]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Item>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reference to a user, group, or domain."]
pub struct Principal {
    #[serde(rename = "groupResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal is a group identified using an external identity. The name field must specify the group resource name with this format: identitysources/{source_id}/groups/{ID}"]
    pub group_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gsuitePrincipal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal is a GSuite user, group or domain."]
    pub gsuite_principal: ::std::option::Option<::std::boxed::Box<GSuitePrincipal>>,
    #[serde(rename = "userResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This principal is a user identified using an external identity. The name field must specify the user resource name with this format: identitysources/{source_id}/users/{ID}"]
    pub user_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessingError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error code indicating the nature of the error."]
    pub code: ::std::option::Option<ProcessingErrorCodeEnum>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the error."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fieldViolations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In case the item fields are invalid, this field contains the details about the validation errors."]
    pub field_violations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FieldViolation>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Error code indicating the nature of the error."]
pub enum ProcessingErrorCodeEnum {
    #[serde(rename = "PROCESSING_ERROR_CODE_UNSPECIFIED")]
    #[doc = "Input only value. Use this value in Items."]
    ProcessingErrorCodeUnspecified,
    #[serde(rename = "MALFORMED_REQUEST")]
    #[doc = "Item's ACL, metadata, or content is malformed or in invalid state. FieldViolations contains more details on where the problem is."]
    MalformedRequest,
    #[serde(rename = "UNSUPPORTED_CONTENT_FORMAT")]
    #[doc = "Countent format is unsupported."]
    UnsupportedContentFormat,
    #[serde(rename = "INDIRECT_BROKEN_ACL")]
    #[doc = "Items with incomplete ACL information due to inheriting other items with broken ACL or having groups with unmapped descendants."]
    IndirectBrokenAcl,
    #[serde(rename = "ACL_CYCLE")]
    #[doc = "ACL inheritance graph formed a cycle."]
    AclCycle,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The definition of a property within an object."]
pub struct PropertyDefinition {
    #[serde(rename = "booleanPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub boolean_property_options: ::std::option::Option<::std::boxed::Box<BooleanPropertyOptions>>,
    #[serde(rename = "datePropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub date_property_options: ::std::option::Option<::std::boxed::Box<DatePropertyOptions>>,
    #[serde(rename = "displayOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options that determine how the property is displayed in the Cloud Search results page if it is specified to be displayed in the object's display options ."]
    pub display_options: ::std::option::Option<::std::boxed::Box<PropertyDisplayOptions>>,
    #[serde(rename = "doublePropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub double_property_options: ::std::option::Option<::std::boxed::Box<DoublePropertyOptions>>,
    #[serde(rename = "enumPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enum_property_options: ::std::option::Option<::std::boxed::Box<EnumPropertyOptions>>,
    #[serde(rename = "htmlPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub html_property_options: ::std::option::Option<::std::boxed::Box<HtmlPropertyOptions>>,
    #[serde(rename = "integerPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integer_property_options: ::std::option::Option<::std::boxed::Box<IntegerPropertyOptions>>,
    #[serde(rename = "isFacetable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the property can be used for generating facets. Cannot be true for properties whose type is object. IsReturnable must be true to set this option. Only supported for Boolean, Enum, and Text properties."]
    pub is_facetable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isRepeatable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that multiple values are allowed for the property. For example, a document only has one description but can have multiple comments. Cannot be true for properties whose type is a boolean. If set to false, properties that contain more than one value cause the indexing request for that item to be rejected."]
    pub is_repeatable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isReturnable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the property identifies data that should be returned in search results via the Query API. If set to *true*, indicates that Query API users can use matching property fields in results. However, storing fields requires more space allocation and uses more bandwidth for search queries, which impacts performance over large datasets. Set to *true* here only if the field is needed for search results. Cannot be true for properties whose type is an object."]
    pub is_returnable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSortable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the property can be used for sorting. Cannot be true for properties that are repeatable. Cannot be true for properties whose type is object or user identifier. IsReturnable must be true to set this option. Only supported for Boolean, Date, Double, Integer, and Timestamp properties."]
    pub is_sortable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSuggestable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the property can be used for generating query suggestions."]
    pub is_suggestable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isWildcardSearchable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that users can perform wildcard search for this property. Only supported for Text properties. IsReturnable must be true to set this option. In a given datasource maximum of 5 properties can be marked as is_wildcard_searchable."]
    pub is_wildcard_searchable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the property. Item indexing requests sent to the Indexing API should set the property name equal to this value. For example, if name is *subject_line*, then indexing requests for document items with subject fields should set the name for that field equal to *subject_line*. Use the name as the identifier for the object property. Once registered as a property for an object, you cannot re-use this name for another property within that object. The name must start with a letter and can only contain letters (A-Z, a-z) or numbers (0-9). The maximum length is 256 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object_property_options: ::std::option::Option<::std::boxed::Box<ObjectPropertyOptions>>,
    #[serde(rename = "textPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_property_options: ::std::option::Option<::std::boxed::Box<TextPropertyOptions>>,
    #[serde(rename = "timestampPropertyOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp_property_options:
        ::std::option::Option<::std::boxed::Box<TimestampPropertyOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The display options for a property."]
pub struct PropertyDisplayOptions {
    #[serde(rename = "displayLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user friendly label for the property that is used if the property is specified to be displayed in ObjectDisplayOptions. If provided, the display label is shown in front of the property values when the property is part of the object display options. For example, if the property value is '1', the value by itself may not be useful context for the user. If the display name given was 'priority', then the user sees 'priority : 1' in the search results which provides clear context to search users. This is OPTIONAL; if not given, only the property values are displayed. The maximum length is 64 characters."]
    pub display_label: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an item to be pushed to the indexing queue."]
pub struct PushItem {
    #[serde(rename = "contentHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
    pub content_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadataHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
    pub metadata_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides additional document state information for the connector, such as an alternate repository ID and other metadata. The maximum length is 8192 bytes."]
    pub payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Queue to which this item belongs to. The default queue is chosen if this field is not specified. The maximum length is 512 characters."]
    pub queue: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repositoryError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Populate this field to store Connector or repository error details. This information is displayed in the Admin Console. This field may only be populated when the Type is REPOSITORY_ERROR."]
    pub repository_error: ::std::option::Option<::std::boxed::Box<RepositoryError>>,
    #[serde(rename = "structuredDataHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structured data hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
    pub structured_data_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the push operation that defines the push behavior."]
    pub _type: ::std::option::Option<PushItemTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the push operation that defines the push behavior."]
pub enum PushItemTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Default UNSPECIFIED. Specifies that the push operation should not modify ItemStatus"]
    Unspecified,
    #[serde(rename = "MODIFIED")]
    #[doc = "Indicates that the repository document has been modified or updated since the previous update call. This changes status to MODIFIED state for an existing item. If this is called on a non existing item, the status is changed to NEW_ITEM."]
    Modified,
    #[serde(rename = "NOT_MODIFIED")]
    #[doc = "Item in the repository has not been modified since the last update call. This push operation will set status to ACCEPTED state."]
    NotModified,
    #[serde(rename = "REPOSITORY_ERROR")]
    #[doc = "Connector is facing a repository error regarding this item. Change status to REPOSITORY_ERROR state. Item is unreserved and rescheduled at a future time determined by exponential backoff."]
    RepositoryError,
    #[serde(rename = "REQUEUE")]
    #[doc = "Call push with REQUEUE only for items that have been reserved. This action unreserves the item and resets its available time to the wall clock time."]
    Requeue,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PushItemRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Item to push onto the queue."]
    pub item: ::std::option::Option<::std::boxed::Box<PushItem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryCountByStatus {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This represents the http status code."]
    pub status_code: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryInterpretation {
    #[serde(rename = "interpretationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub interpretation_type: ::std::option::Option<QueryInterpretationInterpretationTypeEnum>,
    #[serde(rename = "interpretedQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The interpretation of the query used in search. For example, queries with natural language intent like \"email from john\" will be interpreted as \"from:john source:mail\". This field will not be filled when the reason is NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY."]
    pub interpreted_query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for interpretation of the query. This field will not be UNSPECIFIED if the interpretation type is not NONE."]
    pub reason: ::std::option::Option<QueryInterpretationReasonEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum QueryInterpretationInterpretationTypeEnum {
    #[serde(rename = "NONE")]
    #[doc = "Neither the natural language interpretation, nor a broader version of the query is used to fetch the search results."]
    None,
    #[serde(rename = "BLEND")]
    #[doc = "The results from original query are blended with other results. The reason for blending these other results with the results from original query is populated in the 'Reason' field below."]
    Blend,
    #[serde(rename = "REPLACE")]
    #[doc = "The results from original query are replaced. The reason for replacing the results from original query is populated in the 'Reason' field below."]
    Replace,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The reason for interpretation of the query. This field will not be UNSPECIFIED if the interpretation type is not NONE."]
pub enum QueryInterpretationReasonEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = ""]
    Unspecified,
    #[serde(rename = "QUERY_HAS_NATURAL_LANGUAGE_INTENT")]
    #[doc = "Natural language interpretation of the query is used to fetch the search results."]
    QueryHasNaturalLanguageIntent,
    #[serde(rename = "NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY")]
    #[doc = "Query and document terms similarity is used to selectively broaden the query to retrieve additional search results since enough results were not found for the user query. Interpreted query will be empty for this case."]
    NotEnoughResultsFoundForUserQuery,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options to interpret user query."]
pub struct QueryInterpretationOptions {
    #[serde(rename = "disableNlInterpretation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag to disable natural language (NL) interpretation of queries. Default is false, Set to true to disable natural language interpretation. NL interpretation only applies to predefined datasources."]
    pub disable_nl_interpretation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableVerbatimMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable this flag to turn off all internal optimizations like natural language (NL) interpretation of queries, supplemental result retrieval, and usage of synonyms including custom ones. Nl interpretation will be disabled if either one of the two flags is true."]
    pub enable_verbatim_mode: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information relevant only to a query entry."]
pub struct QueryItem {
    #[serde(rename = "isSynthetic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the text was generated by means other than a previous user search."]
    pub is_synthetic: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The definition of a operator that can be used in a Search/Suggest request."]
pub struct QueryOperator {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the operator"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enumValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Potential list of values for the opeatror field. This field is only filled when we can safely enumerate all the possible values of this operator."]
    pub enum_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "greaterThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name that can be used to isolate the property using the greater-than operator."]
    pub greater_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isFacetable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can this operator be used to get facets."]
    pub is_facetable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isRepeatable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if multiple values can be set for this property."]
    pub is_repeatable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isReturnable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Will the property associated with this facet be returned as part of search results."]
    pub is_returnable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSortable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can this operator be used to sort results."]
    pub is_sortable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSuggestable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can get suggestions for this field."]
    pub is_suggestable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lessThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name that can be used to isolate the property using the less-than operator."]
    pub less_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the object corresponding to the operator. This field is only filled for schema-specific operators, and is unset for common operators."]
    pub object_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the operator."]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the operator."]
    pub _type: ::std::option::Option<QueryOperatorTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the operator."]
pub enum QueryOperatorTypeEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Invalid value."]
    Unknown,
    #[serde(rename = "INTEGER")]
    #[doc = ""]
    Integer,
    #[serde(rename = "DOUBLE")]
    #[doc = ""]
    Double,
    #[serde(rename = "TIMESTAMP")]
    #[doc = ""]
    Timestamp,
    #[serde(rename = "BOOLEAN")]
    #[doc = ""]
    Boolean,
    #[serde(rename = "ENUM")]
    #[doc = ""]
    Enum,
    #[serde(rename = "DATE")]
    #[doc = ""]
    Date,
    #[serde(rename = "TEXT")]
    #[doc = ""]
    Text,
    #[serde(rename = "HTML")]
    #[doc = ""]
    Html,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of sources that the user can search using the query API."]
pub struct QuerySource {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the data source."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operators")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all operators applicable for this source."]
    pub operators: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryOperator>>>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short name or alias for the source. This value can be used with the 'source' operator."]
    pub short_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the source"]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This field does not contain anything as of now and is just used as an indicator that the suggest result was a phrase completion."]
pub struct QuerySuggestion {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Errors when the connector is communicating to the source repository."]
pub struct RepositoryError {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Message that describes the error. The maximum allowable length of the message is 8192 characters."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error codes. Matches the definition of HTTP status codes."]
    pub http_status_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of error."]
    pub _type: ::std::option::Option<RepositoryErrorTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of error."]
pub enum RepositoryErrorTypeEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown error."]
    Unknown,
    #[serde(rename = "NETWORK_ERROR")]
    #[doc = "Unknown or unreachable host."]
    NetworkError,
    #[serde(rename = "DNS_ERROR")]
    #[doc = "DNS problem, such as the DNS server is not responding."]
    DnsError,
    #[serde(rename = "CONNECTION_ERROR")]
    #[doc = "Cannot connect to the repository server."]
    ConnectionError,
    #[serde(rename = "AUTHENTICATION_ERROR")]
    #[doc = "Failed authentication due to incorrect credentials."]
    AuthenticationError,
    #[serde(rename = "AUTHORIZATION_ERROR")]
    #[doc = "Service account is not authorized for the repository."]
    AuthorizationError,
    #[serde(rename = "SERVER_ERROR")]
    #[doc = "Repository server error."]
    ServerError,
    #[serde(rename = "QUOTA_EXCEEDED")]
    #[doc = "Quota exceeded."]
    QuotaExceeded,
    #[serde(rename = "SERVICE_UNAVAILABLE")]
    #[doc = "Server temporarily unavailable."]
    ServiceUnavailable,
    #[serde(rename = "CLIENT_ERROR")]
    #[doc = "Client-related error, such as an invalid request from the connector to the repository server."]
    ClientError,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shared request options for all RPC methods."]
pub struct RequestOptions {
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debug options of the request"]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. For translations. Set this field using the language set in browser or for the page. In the event that the user's language preference is known, set this field to the known user language. When specified, the documents in search results are biased towards the specified language. The suggest API does not use this parameter. Instead, suggest autocompletes only based on characters in the query."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchApplicationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID generated when you create a search application using the [admin console](https://support.google.com/a/answer/9043922)."]
    pub search_application_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or \"Australia/Sydney\". These IDs are defined by [Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/) project, and currently available in the file [timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml). This field is used to correctly interpret date and time queries. If this field is not specified, the default time zone (UTC) is used."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetSearchApplicationRequest {
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Debugging information about the response."]
pub struct ResponseDebugInfo {
    #[serde(rename = "formattedDebugInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General debug info formatted for display."]
    pub formatted_debug_info: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information relevant only to a restrict entry. NextId: 12"]
pub struct RestrictItem {
    #[serde(rename = "driveFollowUpRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LINT.ThenChange(//depot/google3/java/com/google/apps/search/quality/itemsuggest/utils/SubtypeRerankingUtils.java)"]
    pub drive_follow_up_restrict: ::std::option::Option<::std::boxed::Box<DriveFollowUpRestrict>>,
    #[serde(rename = "driveLocationRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub drive_location_restrict: ::std::option::Option<::std::boxed::Box<DriveLocationRestrict>>,
    #[serde(rename = "driveMimeTypeRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LINT.IfChange Drive Types."]
    pub drive_mime_type_restrict: ::std::option::Option<::std::boxed::Box<DriveMimeTypeRestrict>>,
    #[serde(rename = "driveTimeSpanRestrict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub drive_time_span_restrict: ::std::option::Option<::std::boxed::Box<DriveTimeSpanRestrict>>,
    #[serde(rename = "searchOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search restrict (e.g. \"after:2017-09-11 before:2017-09-12\")."]
    pub search_operator: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result count information"]
pub struct ResultCounts {
    #[serde(rename = "sourceResultCounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Result count information for each source with results."]
    pub source_result_counts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceResultCount>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Debugging information about the result."]
pub struct ResultDebugInfo {
    #[serde(rename = "formattedDebugInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "General debug info formatted for display."]
    pub formatted_debug_info: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Display Fields for Search Results"]
pub struct ResultDisplayField {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display label for the property."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operator name of the property."]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name value pair for the property."]
    pub property: ::std::option::Option<::std::boxed::Box<NamedProperty>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection of fields that make up a displayed line"]
pub struct ResultDisplayLine {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResultDisplayField>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultDisplayMetadata {
    #[serde(rename = "metalines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metalines content to be displayed with the result."]
    pub metalines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResultDisplayLine>>>,
    #[serde(rename = "objectTypeLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display label for the object."]
    pub object_type_label: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetrievalImportance {
    #[serde(rename = "importance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the ranking importance given to property when it is matched during retrieval. Once set, the token importance of a property cannot be changed."]
    pub importance: ::std::option::Option<RetrievalImportanceImportanceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates the ranking importance given to property when it is matched during retrieval. Once set, the token importance of a property cannot be changed."]
pub enum RetrievalImportanceImportanceEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = "Treat the match like a body text match."]
    Default,
    #[serde(rename = "HIGHEST")]
    #[doc = "Treat the match like a match against title of the item."]
    Highest,
    #[serde(rename = "HIGH")]
    #[doc = "Treat the match with higher importance than body text."]
    High,
    #[serde(rename = "LOW")]
    #[doc = "Treat the match with lower importance than body text."]
    Low,
    #[serde(rename = "NONE")]
    #[doc = "Do not match against this field during retrieval. The property can still be used for operator matching, faceting, and suggest if desired."]
    None,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The schema definition for a data source."]
pub struct Schema {
    #[serde(rename = "objectDefinitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of top-level objects for the data source. The maximum number of elements is 10."]
    pub object_definitions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectDefinition>>>,
    #[serde(rename = "operationIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of the Long Running Operations (LROs) currently running for this schema. After modifying the schema, wait for operations to complete before indexing additional content."]
    pub operation_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Scoring configurations for a source while processing a Search or Suggest request."]
pub struct ScoringConfig {
    #[serde(rename = "disableFreshness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to use freshness as a ranking signal. By default, freshness is used as a ranking signal. Note that this setting is not available in the Admin UI."]
    pub disable_freshness: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "disablePersonalization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to personalize the results. By default, personal signals will be used to boost results."]
    pub disable_personalization: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SearchApplication"]
pub struct SearchApplication {
    #[serde(rename = "dataSourceRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Retrictions applied to the configurations. The maximum number of elements is 10."]
    pub data_source_restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceRestriction>>>,
    #[serde(rename = "defaultFacetOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default fields for returning facet results. The sources specified here also have been included in data_source_restrictions above."]
    pub default_facet_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FacetOptions>>>,
    #[serde(rename = "defaultSortOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default options for sorting the search results"]
    pub default_sort_options: ::std::option::Option<::std::boxed::Box<SortOptions>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the Search Application. The maximum length is 300 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the Search Application. Format: searchapplications/{application_id}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. IDs of the Long Running Operations (LROs) currently running for this schema. Output only field."]
    pub operation_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "scoringConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for ranking results."]
    pub scoring_config: ::std::option::Option<::std::boxed::Box<ScoringConfig>>,
    #[serde(rename = "sourceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for a sources specified in data_source_restrictions."]
    pub source_config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceConfig>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchApplicationQueryStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which query stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "queryCountByStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub query_count_by_status:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryCountByStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchApplicationSessionStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "searchSessionsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of search sessions on the day"]
    pub search_sessions_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchApplicationUserStats {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "oneDayActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past one day"]
    pub one_day_active_users_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sevenDaysActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past seven days"]
    pub seven_days_active_users_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "thirtyDaysActiveUsersCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of unique active users in the past thirty days"]
    pub thirty_days_active_users_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchItemsByViewUrlRequest {
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next_page_token value returned from a previous request, if any."]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "viewUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the full view URL to find the corresponding item. The maximum length is 2048 characters."]
    pub view_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchItemsByViewUrlResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Item>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional search quality metadata of the item."]
pub struct SearchQualityMetadata {
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An indication of the quality of the item, used to influence search quality. Value should be between 0.0 (lowest quality) and 1.0 (highest quality). The default value is 0.0."]
    pub quality: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The search API request."]
pub struct SearchRequest {
    #[serde(rename = "dataSourceRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sources to use for querying. If not specified, all data sources from the current search application are used."]
    pub data_source_restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceRestriction>>>,
    #[serde(rename = "facetOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub facet_options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FacetOptions>>>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of search results to return in one page. Valid values are between 1 and 100, inclusive. Default value is 10. Minimum value is 50 when results beyond 2000 are requested."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw query string. See supported search operators in the [Cloud search Cheat Sheet](https://support.google.com/a/users/answer/9299929)"]
    pub query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryInterpretationOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options to interpret the user query."]
    pub query_interpretation_options:
        ::std::option::Option<::std::boxed::Box<QueryInterpretationOptions>>,
    #[serde(rename = "requestOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request options, such as the search application and user timezone."]
    pub request_options: ::std::option::Option<::std::boxed::Box<RequestOptions>>,
    #[serde(rename = "sortOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The options for sorting the search results"]
    pub sort_options: ::std::option::Option<::std::boxed::Box<SortOptions>>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Starting index of the results."]
    pub start: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The search API response."]
pub struct SearchResponse {
    #[serde(rename = "debugInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debugging information about the response."]
    pub debug_info: ::std::option::Option<::std::boxed::Box<ResponseDebugInfo>>,
    #[serde(rename = "errorInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error information about the response."]
    pub error_info: ::std::option::Option<::std::boxed::Box<ErrorInfo>>,
    #[serde(rename = "facetResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Repeated facet results."]
    pub facet_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FacetResult>>>,
    #[serde(rename = "hasMoreResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether there are more search results matching the query."]
    pub has_more_results: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "queryInterpretation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query interpretation result for user query. Empty if query interpretation is disabled."]
    pub query_interpretation: ::std::option::Option<::std::boxed::Box<QueryInterpretation>>,
    #[serde(rename = "resultCountEstimate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated result count for this query."]
    pub result_count_estimate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resultCountExact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exact result count for this query."]
    pub result_count_exact: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resultCounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expanded result count information."]
    pub result_counts: ::std::option::Option<::std::boxed::Box<ResultCounts>>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Results from a search query."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchResult>>>,
    #[serde(rename = "spellResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested spelling for the query."]
    pub spell_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpellResult>>>,
    #[serde(rename = "structuredResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structured results for the user query. These results are not counted against the page_size."]
    pub structured_results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StructuredResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results containing indexed information for a document."]
pub struct SearchResult {
    #[serde(rename = "clusteredResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If source is clustered, provide list of clustered results. There will only be one level of clustered results. If current source is not enabled for clustering, this field will be empty."]
    pub clustered_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchResult>>>,
    #[serde(rename = "debugInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debugging information about this search result."]
    pub debug_info: ::std::option::Option<::std::boxed::Box<ResultDebugInfo>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata of the search result."]
    pub metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
    #[serde(rename = "snippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The concatenation of all snippets (summaries) available for this result."]
    pub snippet: ::std::option::Option<::std::boxed::Box<Snippet>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the search result."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the search result. The URL contains a Google redirect to the actual item. This URL is signed and shouldn't be changed."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Snippet of the search result, which summarizes the content of the resulting page."]
pub struct Snippet {
    #[serde(rename = "matchRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The matched ranges in the snippet."]
    pub match_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatchRange>>>,
    #[serde(rename = "snippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snippet of the document. The snippet of the document. May contain escaped HTML character that should be unescaped prior to rendering."]
    pub snippet: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortOptions {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the operator corresponding to the field to sort on. The corresponding property must be marked as sortable."]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ascending is the default sort order"]
    pub sort_order: ::std::option::Option<SortOptionsSortOrderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Ascending is the default sort order"]
pub enum SortOptionsSortOrderEnum {
    #[serde(rename = "ASCENDING")]
    #[doc = ""]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = ""]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines sources for the suggest/search APIs."]
pub struct Source {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source name for content indexed by the Indexing API."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Predefined content source for Google Apps."]
    pub predefined_source: ::std::option::Option<SourcePredefinedSourceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Predefined content source for Google Apps."]
pub enum SourcePredefinedSourceEnum {
    #[serde(rename = "NONE")]
    #[doc = ""]
    None,
    #[serde(rename = "QUERY_HISTORY")]
    #[doc = "Suggests queries issued by the user in the past. Only valid when used with the suggest API. Ignored when used in the query API."]
    QueryHistory,
    #[serde(rename = "PERSON")]
    #[doc = "Suggests people in the organization. Only valid when used with the suggest API. Results in an error when used in the query API."]
    Person,
    #[serde(rename = "GOOGLE_DRIVE")]
    #[doc = ""]
    GoogleDrive,
    #[serde(rename = "GOOGLE_GMAIL")]
    #[doc = ""]
    GoogleGmail,
    #[serde(rename = "GOOGLE_SITES")]
    #[doc = ""]
    GoogleSites,
    #[serde(rename = "GOOGLE_GROUPS")]
    #[doc = ""]
    GoogleGroups,
    #[serde(rename = "GOOGLE_CALENDAR")]
    #[doc = ""]
    GoogleCalendar,
    #[serde(rename = "GOOGLE_KEEP")]
    #[doc = ""]
    GoogleKeep,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configurations for a source while processing a Search or Suggest request."]
pub struct SourceConfig {
    #[serde(rename = "crowdingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The crowding configuration for the source."]
    pub crowding_config: ::std::option::Option<::std::boxed::Box<SourceCrowdingConfig>>,
    #[serde(rename = "scoringConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scoring configuration for the source."]
    pub scoring_config: ::std::option::Option<::std::boxed::Box<SourceScoringConfig>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source for which this configuration is to be used."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Set search results crowding limits. Crowding is a situation in which multiple results from the same source or host \"crowd out\" other results, diminishing the quality of search for users. To foster better search quality and source diversity in search results, you can set a condition to reduce repetitive results by source."]
pub struct SourceCrowdingConfig {
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of results allowed from a source. No limits will be set on results if this value is less than or equal to 0."]
    pub num_results: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numSuggestions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of suggestions allowed from a source. No limits will be set on results if this value is less than or equal to 0."]
    pub num_suggestions: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Per source result count information."]
pub struct SourceResultCount {
    #[serde(rename = "hasMoreResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether there are more search results for this source."]
    pub has_more_results: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resultCountEstimate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated result count for this source."]
    pub result_count_estimate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resultCountExact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exact result count for this source."]
    pub result_count_exact: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source the result count information is associated with."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Set the scoring configuration. This allows modifying the ranking of results for a source."]
pub struct SourceScoringConfig {
    #[serde(rename = "sourceImportance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Importance of the source."]
    pub source_importance: ::std::option::Option<SourceScoringConfigSourceImportanceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Importance of the source."]
pub enum SourceScoringConfigSourceImportanceEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = ""]
    Default,
    #[serde(rename = "LOW")]
    #[doc = ""]
    Low,
    #[serde(rename = "HIGH")]
    #[doc = ""]
    High,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpellResult {
    #[serde(rename = "suggestedQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested spelling of the query."]
    pub suggested_query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Start upload file request."]
pub struct StartUploadItemRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A structured data object consisting of named properties."]
pub struct StructuredDataObject {
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties for the object. The maximum number of elements is 1000."]
    pub properties: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NamedProperty>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structured results that are returned as part of search request."]
pub struct StructuredResult {
    #[serde(rename = "person")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Representation of a person"]
    pub person: ::std::option::Option<::std::boxed::Box<Person>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request of suggest API."]
pub struct SuggestRequest {
    #[serde(rename = "dataSourceRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sources to use for suggestions. If not specified, the data sources are taken from the current search application. NOTE: Suggestions are only supported for the following sources: * Third-party data sources * PredefinedSource.PERSON * PredefinedSource.GOOGLE_DRIVE"]
    pub data_source_restrictions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceRestriction>>>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Partial query for which autocomplete suggestions will be shown. For example, if the query is \"sea\", then the server might return \"season\", \"search\", \"seagull\" and so on."]
    pub query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request options, such as the search application and user timezone."]
    pub request_options: ::std::option::Option<::std::boxed::Box<RequestOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of the suggest API."]
pub struct SuggestResponse {
    #[serde(rename = "suggestResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of suggestions."]
    pub suggest_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SuggestResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "One suggestion result."]
pub struct SuggestResult {
    #[serde(rename = "peopleSuggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is present when the suggestion indicates a person. It contains more information about the person - like their email ID, name etc."]
    pub people_suggestion: ::std::option::Option<::std::boxed::Box<PeopleSuggestion>>,
    #[serde(rename = "querySuggestion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field will be present if the suggested query is a word/phrase completion."]
    pub query_suggestion: ::std::option::Option<::std::boxed::Box<QuerySuggestion>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of the suggestion."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "suggestedQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The suggested query that will be used for search, when the user clicks on the suggestion"]
    pub suggested_query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for text properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct TextOperatorOptions {
    #[serde(rename = "exactMatchWithOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the text value is tokenized as one atomic value in operator searches and facet matches. For example, if the operator name is \"genre\" and the value is \"science-fiction\" the query restrictions \"genre:science\" and \"genre:fiction\" doesn't match the item; \"genre:science-fiction\" does. Value matching is case-sensitive and does not remove special characters. If false, the text is tokenized. For example, if the value is \"science-fiction\" the queries \"genre:science\" and \"genre:fiction\" matches the item."]
    pub exact_match_with_operator: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the text property. For example, if operatorName is *subject* and the property's name is *subjectLine*, then queries like *subject:<value>* show results only where the value of the property named *subjectLine* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any text properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for text properties."]
pub struct TextPropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the property should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<TextOperatorOptions>>,
    #[serde(rename = "retrievalImportance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the search quality importance of the tokens within the field when used for retrieval."]
    pub retrieval_importance: ::std::option::Option<::std::boxed::Box<RetrievalImportance>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of text values."]
pub struct TextValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum allowable length for text values is 2048 characters."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to provide a search operator for timestamp properties. This is optional. Search operators let users restrict the query to specific fields relevant to the type of item being searched."]
pub struct TimestampOperatorOptions {
    #[serde(rename = "greaterThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property using the greater-than operator. For example, if greaterThanOperatorName is *closedafter* and the property's name is *closeDate*, then queries like *closedafter:<value>* show results only where the value of the property named *closeDate* is later than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub greater_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lessThanOperatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property using the less-than operator. For example, if lessThanOperatorName is *closedbefore* and the property's name is *closeDate*, then queries like *closedbefore:<value>* show results only where the value of the property named *closeDate* is earlier than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub less_than_operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property. For example, if operatorName is *closedon* and the property's name is *closeDate*, then queries like *closedon:<value>* show results only where the value of the property named *closeDate* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
    pub operator_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for timestamp properties."]
pub struct TimestampPropertyOptions {
    #[serde(rename = "operatorOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, describes how the timestamp should be used as a search operator."]
    pub operator_options: ::std::option::Option<::std::boxed::Box<TimestampOperatorOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of timestamp values."]
pub struct TimestampValues {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnmappedIdentity {
    #[serde(rename = "externalIdentity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for an external user."]
    pub external_identity: ::std::option::Option<::std::boxed::Box<Principal>>,
    #[serde(rename = "resolutionStatusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resolution status for the external identity."]
    pub resolution_status_code: ::std::option::Option<UnmappedIdentityResolutionStatusCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The resolution status for the external identity."]
pub enum UnmappedIdentityResolutionStatusCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Input-only value. Used to list all unmapped identities regardless of status."]
    CodeUnspecified,
    #[serde(rename = "NOT_FOUND")]
    #[doc = "The unmapped identity was not found in IDaaS, and needs to be provided by the user."]
    NotFound,
    #[serde(rename = "IDENTITY_SOURCE_NOT_FOUND")]
    #[doc = "The identity source associated with the identity was either not found or deleted."]
    IdentitySourceNotFound,
    #[serde(rename = "IDENTITY_SOURCE_MISCONFIGURED")]
    #[doc = "IDaaS does not understand the identity source, probably because the schema was modified in a non compatible way."]
    IdentitySourceMisconfigured,
    #[serde(rename = "TOO_MANY_MAPPINGS_FOUND")]
    #[doc = "The number of users associated with the external identity is too large."]
    TooManyMappingsFound,
    #[serde(rename = "INTERNAL_ERROR")]
    #[doc = "Internal error."]
    InternalError,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnreserveItemsRequest {
    #[serde(rename = "connectorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a queue to unreserve items from."]
    pub queue: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::boxed::Box<DataSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateSchemaRequest {
    #[serde(rename = "debugOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common debug options."]
    pub debug_options: ::std::option::Option<::std::boxed::Box<DebugOptions>>,
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new schema for the source."]
    pub schema: ::std::option::Option<::std::boxed::Box<Schema>>,
    #[serde(rename = "validateOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the schema will be checked for validity, but will not be registered with the data source, even if valid."]
    pub validate_only: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an upload session reference. This reference is created via upload method. Updating of item content may refer to this uploaded content via contentDataRef."]
pub struct UploadItemRef {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the content reference. The maximum length is 2048 characters."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a single value with generic type."]
pub struct Value {
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub boolean_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dateValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub date_value: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integer_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueFilter {
    #[serde(rename = "operatorName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `operator_name` applied to the query, such as *price_greater_than*. The filter can work against both types of filters defined in the schema for your data source: 1. `operator_name`, where the query filters results by the property that matches the value. 2. `greater_than_operator_name` or `less_than_operator_name` in your schema. The query filters the results for the property values that are greater than or less than the supplied value in the query."]
    pub operator_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value to be compared with."]
    pub value: ::std::option::Option<::std::boxed::Box<Value>>,
}
