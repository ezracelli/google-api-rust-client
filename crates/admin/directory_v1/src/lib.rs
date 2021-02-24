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
    pub mod chromeosdevices {
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
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Determines whether the response contains the full list of properties or only a subset."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Determines whether the response contains the full list of properties or only a subset."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, serialNumber, status, and user)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Device property to use for sorting results."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orgUnitPath")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The full path of the organizational unit or its unique ID."]
                    pub org_unit_path: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `pageToken` query parameter is used to request the next page of query results. The follow-on request's `pageToken` query parameter is the `nextPageToken` from your previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search string in the format given at http://support.google.com/chromeos/a/bin/answer.py?answer=1698333"]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter."]
                    pub sort_order: ::std::option::Option<QueryParametersSortOrderEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Device property to use for sorting results."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "orderByUndefined")]
                    #[doc = ""]
                    OrderByUndefined,
                    #[serde(rename = "annotatedLocation")]
                    #[doc = "Chrome device location as annotated by the administrator."]
                    AnnotatedLocation,
                    #[serde(rename = "annotatedUser")]
                    #[doc = "Chromebook user as annotated by administrator."]
                    AnnotatedUser,
                    #[serde(rename = "lastSync")]
                    #[doc = "The date and time the Chrome device was last synchronized with the policy settings in the Admin console."]
                    LastSync,
                    #[serde(rename = "notes")]
                    #[doc = "Chrome device notes as annotated by the administrator."]
                    Notes,
                    #[serde(rename = "serialNumber")]
                    #[doc = "The Chrome device serial number entered when the device was enabled."]
                    SerialNumber,
                    #[serde(rename = "status")]
                    #[doc = "Chrome device status. For more information, see the <a [chromeosdevices](/admin-sdk/directory/v1/reference/chromeosdevices.html)."]
                    Status,
                    #[serde(rename = "supportEndDate")]
                    #[doc = "Chrome device support end date. This is applicable only for devices purchased directly from Google."]
                    SupportEndDate,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, serialNumber, status, and user)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter."]
                pub enum QueryParametersSortOrderEnum {
                    #[serde(rename = "SORT_ORDER_UNDEFINED")]
                    #[doc = ""]
                    SortOrderUndefined,
                    #[serde(rename = "ASCENDING")]
                    #[doc = "Ascending order."]
                    Ascending,
                    #[serde(rename = "DESCENDING")]
                    #[doc = "Descending order."]
                    Descending,
                }
                impl ::std::default::Default for QueryParametersSortOrderEnum {
                    fn default() -> Self {
                        Self::SortOrderUndefined
                    }
                }
            }
            pub mod move_devices_to_ou {
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
                    #[serde(rename = "orgUnitPath")]
                    #[doc = "Full path of the target organizational unit or its ID"]
                    pub org_unit_path: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod patch {
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
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, serialNumber, status, and user)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, serialNumber, status, and user)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
            }
        }
    }
    pub mod domain_aliases {
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
                    #[serde(rename = "parentDomainName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Name of the parent domain for which domain aliases are to be fetched."]
                    pub parent_domain_name: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod groups {
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
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users](/admin-sdk/directory/v1/reference/users)"]
                    pub customer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "domain")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead."]
                    pub domain: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return. Max allowed value is 200."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Column to use for sorting results"]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string search. Should be of the form \"\". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-groups"]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return results in ascending or descending order. Only of use when orderBy is also used"]
                    pub sort_order: ::std::option::Option<QueryParametersSortOrderEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Email or immutable ID of the user if only those groups are to be listed, the given user is a member of. If it's an ID, it should match with the ID of the user object."]
                    pub user_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"200").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Column to use for sorting results"]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "orderByUndefined")]
                    #[doc = ""]
                    OrderByUndefined,
                    #[serde(rename = "email")]
                    #[doc = "Email of the group."]
                    Email,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return results in ascending or descending order. Only of use when orderBy is also used"]
                pub enum QueryParametersSortOrderEnum {
                    #[serde(rename = "SORT_ORDER_UNDEFINED")]
                    #[doc = ""]
                    SortOrderUndefined,
                    #[serde(rename = "ASCENDING")]
                    #[doc = "Ascending order."]
                    Ascending,
                    #[serde(rename = "DESCENDING")]
                    #[doc = "Descending order."]
                    Descending,
                }
                impl ::std::default::Default for QueryParametersSortOrderEnum {
                    fn default() -> Self {
                        Self::SortOrderUndefined
                    }
                }
            }
        }
    }
    pub mod members {
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
                    #[serde(rename = "includeDerivedMembership")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to list indirect memberships. Default: false."]
                    pub include_derived_membership: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return. Max allowed value is 200."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "roles")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `roles` query parameter allows you to retrieve group members by role. Allowed values are `OWNER`, `MANAGER`, and `MEMBER`."]
                    pub roles: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"200").unwrap()
                    }
                }
            }
        }
    }
    pub mod mobiledevices {
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
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, model, status, type, and status)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return. Max allowed value is 100."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Device property to use for sorting results."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search string in the format given at https://developers.google.com/admin-sdk/directory/v1/search-operators"]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter."]
                    pub sort_order: ::std::option::Option<QueryParametersSortOrderEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Device property to use for sorting results."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "orderByUndefined")]
                    #[doc = ""]
                    OrderByUndefined,
                    #[serde(rename = "deviceId")]
                    #[doc = "The serial number for a Google Sync mobile device. For Android devices, this is a software generated unique identifier."]
                    DeviceId,
                    #[serde(rename = "email")]
                    #[doc = "The device owner's email address."]
                    Email,
                    #[serde(rename = "lastSync")]
                    #[doc = "Last policy settings sync date time of the device."]
                    LastSync,
                    #[serde(rename = "model")]
                    #[doc = "The mobile device's model."]
                    Model,
                    #[serde(rename = "name")]
                    #[doc = "The device owner's user name."]
                    Name,
                    #[serde(rename = "os")]
                    #[doc = "The device's operating system."]
                    Os,
                    #[serde(rename = "status")]
                    #[doc = "The device status."]
                    Status,
                    #[serde(rename = "type")]
                    #[doc = "Type of the device."]
                    Type,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "BASIC")]
                    #[doc = "Includes only the basic metadata fields (e.g., deviceId, model, status, type, and status)"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all metadata fields"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter."]
                pub enum QueryParametersSortOrderEnum {
                    #[serde(rename = "SORT_ORDER_UNDEFINED")]
                    #[doc = ""]
                    SortOrderUndefined,
                    #[serde(rename = "ASCENDING")]
                    #[doc = "Ascending order."]
                    Ascending,
                    #[serde(rename = "DESCENDING")]
                    #[doc = "Descending order."]
                    Descending,
                }
                impl ::std::default::Default for QueryParametersSortOrderEnum {
                    fn default() -> Self {
                        Self::SortOrderUndefined
                    }
                }
            }
        }
    }
    pub mod orgunits {
        pub mod methods {
            pub mod delete {
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
                    #[serde(rename = "allowPlus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Parses org unit path without url decode to allow for plus in ou name"]
                    pub allow_plus: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "allowPlus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Parses org unit path without url decode to allow for plus in ou name"]
                    pub allow_plus: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: org_unit_path () }",
                        setter(into)
                    )]
                    #[serde(rename = "orgUnitPath")]
                    #[serde(default = "query_parameters_defaults :: org_unit_path")]
                    #[doc = "The full path to the organizational unit or its unique ID. Returns the children of the specified organizational unit."]
                    pub org_unit_path: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "type")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return all sub-organizations or just immediate children."]
                    pub _type: ::std::option::Option<QueryParametersTypeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn org_unit_path() -> ::std::string::String {
                        serde_json::from_str(&"\"\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return all sub-organizations or just immediate children."]
                pub enum QueryParametersTypeEnum {
                    #[serde(rename = "typeUndefined")]
                    #[doc = ""]
                    TypeUndefined,
                    #[serde(rename = "all")]
                    #[doc = "All sub-organizational units."]
                    All,
                    #[serde(rename = "children")]
                    #[doc = "Immediate children only (default)."]
                    Children,
                }
                impl ::std::default::Default for QueryParametersTypeEnum {
                    fn default() -> Self {
                        Self::TypeUndefined
                    }
                }
            }
            pub mod patch {
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
                    #[serde(rename = "allowPlus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Parses org unit path without url decode to allow for plus in ou name"]
                    pub allow_plus: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "allowPlus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Parses org unit path without url decode to allow for plus in ou name"]
                    pub allow_plus: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod resources {
        pub mod resources {
            pub mod buildings {
                pub mod methods {
                    pub mod insert {
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
                                default = "{ query_parameters_defaults :: coordinates_source () }",
                                setter(into)
                            )]
                            #[serde(rename = "coordinatesSource")]
                            #[serde(default = "query_parameters_defaults :: coordinates_source")]
                            #[doc = "Source from which Building.coordinates are derived."]
                            pub coordinates_source: QueryParametersCoordinatesSourceEnum,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn coordinates_source(
                            ) -> super::QueryParametersCoordinatesSourceEnum
                            {
                                serde_json::from_str(&"\"SOURCE_UNSPECIFIED\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Source from which Building.coordinates are derived."]
                        pub enum QueryParametersCoordinatesSourceEnum {
                            #[serde(rename = "COORDINATES_SOURCE_UNDEFINED")]
                            #[doc = ""]
                            CoordinatesSourceUndefined,
                            #[serde(rename = "CLIENT_SPECIFIED")]
                            #[doc = "Building.coordinates are set to the coordinates included in the request."]
                            ClientSpecified,
                            #[serde(rename = "RESOLVED_FROM_ADDRESS")]
                            #[doc = "Building.coordinates are automatically populated based on the postal address."]
                            ResolvedFromAddress,
                            #[serde(rename = "SOURCE_UNSPECIFIED")]
                            #[doc = "Defaults to `RESOLVED_FROM_ADDRESS` if postal address is provided. Otherwise, defaults to `CLIENT_SPECIFIED` if coordinates are provided."]
                            SourceUnspecified,
                        }
                        impl ::std::default::Default for QueryParametersCoordinatesSourceEnum {
                            fn default() -> Self {
                                Self::SourceUnspecified
                            }
                        }
                    }
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify the next page in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod patch {
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
                                default = "{ query_parameters_defaults :: coordinates_source () }",
                                setter(into)
                            )]
                            #[serde(rename = "coordinatesSource")]
                            #[serde(default = "query_parameters_defaults :: coordinates_source")]
                            #[doc = "Source from which Building.coordinates are derived."]
                            pub coordinates_source: QueryParametersCoordinatesSourceEnum,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn coordinates_source(
                            ) -> super::QueryParametersCoordinatesSourceEnum
                            {
                                serde_json::from_str(&"\"SOURCE_UNSPECIFIED\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Source from which Building.coordinates are derived."]
                        pub enum QueryParametersCoordinatesSourceEnum {
                            #[serde(rename = "COORDINATES_SOURCE_UNDEFINED")]
                            #[doc = ""]
                            CoordinatesSourceUndefined,
                            #[serde(rename = "CLIENT_SPECIFIED")]
                            #[doc = "Building.coordinates are set to the coordinates included in the request."]
                            ClientSpecified,
                            #[serde(rename = "RESOLVED_FROM_ADDRESS")]
                            #[doc = "Building.coordinates are automatically populated based on the postal address."]
                            ResolvedFromAddress,
                            #[serde(rename = "SOURCE_UNSPECIFIED")]
                            #[doc = "Defaults to `RESOLVED_FROM_ADDRESS` if postal address is provided. Otherwise, defaults to `CLIENT_SPECIFIED` if coordinates are provided."]
                            SourceUnspecified,
                        }
                        impl ::std::default::Default for QueryParametersCoordinatesSourceEnum {
                            fn default() -> Self {
                                Self::SourceUnspecified
                            }
                        }
                    }
                    pub mod update {
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
                                default = "{ query_parameters_defaults :: coordinates_source () }",
                                setter(into)
                            )]
                            #[serde(rename = "coordinatesSource")]
                            #[serde(default = "query_parameters_defaults :: coordinates_source")]
                            #[doc = "Source from which Building.coordinates are derived."]
                            pub coordinates_source: QueryParametersCoordinatesSourceEnum,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn coordinates_source(
                            ) -> super::QueryParametersCoordinatesSourceEnum
                            {
                                serde_json::from_str(&"\"SOURCE_UNSPECIFIED\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Source from which Building.coordinates are derived."]
                        pub enum QueryParametersCoordinatesSourceEnum {
                            #[serde(rename = "COORDINATES_SOURCE_UNDEFINED")]
                            #[doc = ""]
                            CoordinatesSourceUndefined,
                            #[serde(rename = "CLIENT_SPECIFIED")]
                            #[doc = "Building.coordinates are set to the coordinates included in the request."]
                            ClientSpecified,
                            #[serde(rename = "RESOLVED_FROM_ADDRESS")]
                            #[doc = "Building.coordinates are automatically populated based on the postal address."]
                            ResolvedFromAddress,
                            #[serde(rename = "SOURCE_UNSPECIFIED")]
                            #[doc = "Defaults to `RESOLVED_FROM_ADDRESS` if postal address is provided. Otherwise, defaults to `CLIENT_SPECIFIED` if coordinates are provided."]
                            SourceUnspecified,
                        }
                        impl ::std::default::Default for QueryParametersCoordinatesSourceEnum {
                            fn default() -> Self {
                                Self::SourceUnspecified
                            }
                        }
                    }
                }
            }
            pub mod calendars {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Field(s) to sort results by in either ascending or descending order. Supported fields include `resourceId`, `resourceName`, `capacity`, `buildingId`, and `floorName`. If no order is specified, defaults to ascending. Should be of the form \"field [asc|desc], field [asc|desc], ...\". For example `buildingId, capacity desc` would return results sorted first by `buildingId` in ascending order then by `capacity` in descending order."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify the next page in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "query")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String query used to filter results. Should be of the form \"field operator value\" where field can be any of supported fields and operators can be any of supported operations. Operators include '=' for exact match, '!=' for mismatch and ':' for prefix match or HAS match where applicable. For prefix match, the value should always be followed by a *. Logical operators NOT and AND are supported (in this order of precedence). Supported fields include `generatedResourceName`, `name`, `buildingId`, `floor_name`, `capacity`, `featureInstances.feature.name`, `resourceEmail`, `resourceCategory`. For example `buildingId=US-NYC-9TH AND featureInstances.feature.name:Phone`."]
                            pub query: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod features {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify the next page in the list."]
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
    pub mod role_assignments {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify the next page in the list."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "roleId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Immutable ID of a role. If included in the request, returns only role assignments containing this role ID."]
                    pub role_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The user's primary email address, alias email address, or unique user ID. If included in the request, returns role assignments only for this user."]
                    pub user_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod roles {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify the next page in the list."]
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
    pub mod users {
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
                    #[serde(rename = "customFieldMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`."]
                    pub custom_field_mask: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: projection () }",
                        setter(into)
                    )]
                    #[serde(rename = "projection")]
                    #[serde(default = "query_parameters_defaults :: projection")]
                    #[doc = "What subset of fields to fetch for this user."]
                    pub projection: QueryParametersProjectionEnum,
                    #[builder(
                        default = "{ query_parameters_defaults :: view_type () }",
                        setter(into)
                    )]
                    #[serde(rename = "viewType")]
                    #[serde(default = "query_parameters_defaults :: view_type")]
                    #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                    pub view_type: QueryParametersViewTypeEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn projection() -> super::QueryParametersProjectionEnum {
                        serde_json::from_str(&"\"basic\"").unwrap()
                    }
                    pub fn view_type() -> super::QueryParametersViewTypeEnum {
                        serde_json::from_str(&"\"admin_view\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "What subset of fields to fetch for this user."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "projectionUndefined")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "basic")]
                    #[doc = "Do not include any custom fields for the user."]
                    Basic,
                    #[serde(rename = "custom")]
                    #[doc = "Include custom fields from schemas requested in `customFieldMask`."]
                    Custom,
                    #[serde(rename = "full")]
                    #[doc = "Include all fields associated with this user."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                pub enum QueryParametersViewTypeEnum {
                    #[serde(rename = "view_type_undefined")]
                    #[doc = ""]
                    ViewTypeUndefined,
                    #[serde(rename = "admin_view")]
                    #[doc = "Results include both administrator-only and domain-public fields for the user."]
                    AdminView,
                    #[serde(rename = "domain_public")]
                    #[doc = "Results only include fields for the user that are publicly visible to other users in the domain."]
                    DomainPublic,
                }
                impl ::std::default::Default for QueryParametersViewTypeEnum {
                    fn default() -> Self {
                        Self::AdminView
                    }
                }
            }
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
                    #[serde(rename = "customFieldMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`."]
                    pub custom_field_mask: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. You can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users). Either the `customer` or the `domain` parameter must be provided."]
                    pub customer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "domain")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead. Either the `customer` or the `domain` parameter must be provided."]
                    pub domain: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Property to use for sorting results."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: projection () }",
                        setter(into)
                    )]
                    #[serde(rename = "projection")]
                    #[serde(default = "query_parameters_defaults :: projection")]
                    #[doc = "What subset of fields to fetch for this user."]
                    pub projection: QueryParametersProjectionEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for searching user fields. For more information on constructing user queries, see [Search for Users](/admin-sdk/directory/v1/guides/search-users)."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set to `true`, retrieves the list of deleted users. (Default: `false`)"]
                    pub show_deleted: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return results in ascending or descending order."]
                    pub sort_order: ::std::option::Option<QueryParametersSortOrderEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: view_type () }",
                        setter(into)
                    )]
                    #[serde(rename = "viewType")]
                    #[serde(default = "query_parameters_defaults :: view_type")]
                    #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                    pub view_type: QueryParametersViewTypeEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn projection() -> super::QueryParametersProjectionEnum {
                        serde_json::from_str(&"\"basic\"").unwrap()
                    }
                    pub fn view_type() -> super::QueryParametersViewTypeEnum {
                        serde_json::from_str(&"\"admin_view\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Property to use for sorting results."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "orderByUndefined")]
                    #[doc = ""]
                    OrderByUndefined,
                    #[serde(rename = "email")]
                    #[doc = "Primary email of the user."]
                    Email,
                    #[serde(rename = "familyName")]
                    #[doc = "User's family name."]
                    FamilyName,
                    #[serde(rename = "givenName")]
                    #[doc = "User's given name."]
                    GivenName,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "What subset of fields to fetch for this user."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "projectionUndefined")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "basic")]
                    #[doc = "Do not include any custom fields for the user."]
                    Basic,
                    #[serde(rename = "custom")]
                    #[doc = "Include custom fields from schemas requested in `customFieldMask`."]
                    Custom,
                    #[serde(rename = "full")]
                    #[doc = "Include all fields associated with this user."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return results in ascending or descending order."]
                pub enum QueryParametersSortOrderEnum {
                    #[serde(rename = "SORT_ORDER_UNDEFINED")]
                    #[doc = ""]
                    SortOrderUndefined,
                    #[serde(rename = "ASCENDING")]
                    #[doc = "Ascending order."]
                    Ascending,
                    #[serde(rename = "DESCENDING")]
                    #[doc = "Descending order."]
                    Descending,
                }
                impl ::std::default::Default for QueryParametersSortOrderEnum {
                    fn default() -> Self {
                        Self::SortOrderUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                pub enum QueryParametersViewTypeEnum {
                    #[serde(rename = "view_type_undefined")]
                    #[doc = ""]
                    ViewTypeUndefined,
                    #[serde(rename = "admin_view")]
                    #[doc = "Results include both administrator-only and domain-public fields for the user."]
                    AdminView,
                    #[serde(rename = "domain_public")]
                    #[doc = "Results only include fields for the user that are publicly visible to other users in the domain."]
                    DomainPublic,
                }
                impl ::std::default::Default for QueryParametersViewTypeEnum {
                    fn default() -> Self {
                        Self::AdminView
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
                    #[serde(rename = "customFieldMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when projection=custom."]
                    pub custom_field_mask: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Immutable ID of the Google Workspace account. In case of multi-domain, to fetch all users for a customer, fill this field instead of domain."]
                    pub customer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "domain")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Name of the domain. Fill this field to get users from only this domain. To return all users in a multi-domain fill customer field instead.\""]
                    pub domain: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "event")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Events to watch for."]
                    pub event: ::std::option::Option<QueryParametersEventEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Column to use for sorting results"]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: projection () }",
                        setter(into)
                    )]
                    #[serde(rename = "projection")]
                    #[serde(default = "query_parameters_defaults :: projection")]
                    #[doc = "What subset of fields to fetch for this user."]
                    pub projection: QueryParametersProjectionEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string search. Should be of the form \"\". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-users"]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set to true, retrieves the list of deleted users. (Default: false)"]
                    pub show_deleted: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to return results in ascending or descending order."]
                    pub sort_order: ::std::option::Option<QueryParametersSortOrderEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: view_type () }",
                        setter(into)
                    )]
                    #[serde(rename = "viewType")]
                    #[serde(default = "query_parameters_defaults :: view_type")]
                    #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                    pub view_type: QueryParametersViewTypeEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn projection() -> super::QueryParametersProjectionEnum {
                        serde_json::from_str(&"\"basic\"").unwrap()
                    }
                    pub fn view_type() -> super::QueryParametersViewTypeEnum {
                        serde_json::from_str(&"\"admin_view\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Events to watch for."]
                pub enum QueryParametersEventEnum {
                    #[serde(rename = "eventTypeUnspecified")]
                    #[doc = ""]
                    EventTypeUnspecified,
                    #[serde(rename = "add")]
                    #[doc = "User Created Event"]
                    Add,
                    #[serde(rename = "delete")]
                    #[doc = "User Deleted Event"]
                    Delete,
                    #[serde(rename = "makeAdmin")]
                    #[doc = "User Admin Status Change Event"]
                    MakeAdmin,
                    #[serde(rename = "undelete")]
                    #[doc = "User Undeleted Event"]
                    Undelete,
                    #[serde(rename = "update")]
                    #[doc = "User Updated Event"]
                    Update,
                }
                impl ::std::default::Default for QueryParametersEventEnum {
                    fn default() -> Self {
                        Self::EventTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Column to use for sorting results"]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "orderByUnspecified")]
                    #[doc = ""]
                    OrderByUnspecified,
                    #[serde(rename = "email")]
                    #[doc = "Primary email of the user."]
                    Email,
                    #[serde(rename = "familyName")]
                    #[doc = "User's family name."]
                    FamilyName,
                    #[serde(rename = "givenName")]
                    #[doc = "User's given name."]
                    GivenName,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "What subset of fields to fetch for this user."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "projectionUnspecified")]
                    #[doc = ""]
                    ProjectionUnspecified,
                    #[serde(rename = "basic")]
                    #[doc = "Do not include any custom fields for the user."]
                    Basic,
                    #[serde(rename = "custom")]
                    #[doc = "Include custom fields from schemas mentioned in customFieldMask."]
                    Custom,
                    #[serde(rename = "full")]
                    #[doc = "Include all fields associated with this user."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to return results in ascending or descending order."]
                pub enum QueryParametersSortOrderEnum {
                    #[serde(rename = "sortOrderUnspecified")]
                    #[doc = ""]
                    SortOrderUnspecified,
                    #[serde(rename = "ASCENDING")]
                    #[doc = "Ascending order."]
                    Ascending,
                    #[serde(rename = "DESCENDING")]
                    #[doc = "Descending order."]
                    Descending,
                }
                impl ::std::default::Default for QueryParametersSortOrderEnum {
                    fn default() -> Self {
                        Self::SortOrderUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin)."]
                pub enum QueryParametersViewTypeEnum {
                    #[serde(rename = "admin_view")]
                    #[doc = "Results include both administrator-only and domain-public fields."]
                    AdminView,
                    #[serde(rename = "domain_public")]
                    #[doc = "Results only include fields for the user that are publicly visible to other users in the domain."]
                    DomainPublic,
                }
                impl ::std::default::Default for QueryParametersViewTypeEnum {
                    fn default() -> Self {
                        Self::AdminView
                    }
                }
            }
        }
        pub mod resources {
            pub mod aliases {
                pub mod methods {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "event")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Events to watch for."]
                            pub event: ::std::option::Option<QueryParametersEventEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Events to watch for."]
                        pub enum QueryParametersEventEnum {
                            #[serde(rename = "eventUndefined")]
                            #[doc = ""]
                            EventUndefined,
                            #[serde(rename = "add")]
                            #[doc = "Alias Created Event"]
                            Add,
                            #[serde(rename = "delete")]
                            #[doc = "Alias Deleted Event"]
                            Delete,
                        }
                        impl ::std::default::Default for QueryParametersEventEnum {
                            fn default() -> Self {
                                Self::EventUndefined
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
    #[doc = "JSON template for Alias object in Directory API."]
    pub struct Alias {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ alias_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "alias_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub primary_email: ::std::option::Option<::std::string::String>,
    }
    impl Alias {
        pub fn builder() -> AliasBuilder {
            AliasBuilder::default()
        }
    }
    mod alias_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#alias\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON response template to list aliases in Directory API."]
    pub struct Aliases {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub aliases: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ aliases_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "aliases_defaults :: kind")]
        pub kind: ::std::string::String,
    }
    impl Aliases {
        pub fn builder() -> AliasesBuilder {
            AliasesBuilder::default()
        }
    }
    mod aliases_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#aliases\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp)."]
    pub struct Asp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the ASP."]
        pub code_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the ASP was created. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the ASP."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ asp_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "asp_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#asp`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastTimeUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the ASP was last used. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format."]
        pub last_time_used: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the application that the user, represented by their `userId`, entered when the ASP was created."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the user who issued the ASP."]
        pub user_key: ::std::option::Option<::std::string::String>,
    }
    impl Asp {
        pub fn builder() -> AspBuilder {
            AspBuilder::default()
        }
    }
    mod asp_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#asp\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Asps {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of ASP resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Asp>>>,
        #[builder(default = "{ asps_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "asps_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#aspList`."]
        pub kind: ::std::string::String,
    }
    impl Asps {
        pub fn builder() -> AspsBuilder {
            AspsBuilder::default()
        }
    }
    mod asps_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#aspList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.buildings"]
    pub struct Building {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The postal address of the building. See [`PostalAddress`](/my-business/reference/rest/v4/PostalAddress) for details. Note that only a single address line and region code are required."]
        pub address: ::std::option::Option<::std::boxed::Box<BuildingAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the building. The maximum length is 100 characters."]
        pub building_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildingName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The building name as seen by users in Calendar. Must be unique for the customer. For example, \"NYC-CHEL\". The maximum length is 100 characters."]
        pub building_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coordinates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic coordinates of the center of the building, expressed as latitude and longitude in decimal degrees."]
        pub coordinates: ::std::option::Option<::std::boxed::Box<BuildingCoordinates>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A brief description of the building. For example, \"Chelsea Market\"."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etags: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floorNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display names for all floors in this building. The floors are expected to be sorted in ascending order, from lowest floor to highest floor. For example, [\"B2\", \"B1\", \"L\", \"1\", \"2\", \"2M\", \"3\", \"PH\"] Must contain at least one entry."]
        pub floor_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ building_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "building_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
    }
    impl Building {
        pub fn builder() -> BuildingBuilder {
            BuildingBuilder::default()
        }
    }
    mod building_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#buildings#Building\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.buildings"]
    pub struct BuildingAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unstructured address lines describing the lower levels of an address."]
        pub address_lines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "administrativeArea")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region."]
        pub administrative_area: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known)."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Postal code of the address."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. CLDR region code of the country/region of the address."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sublocality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Sublocality of the address."]
        pub sublocality: ::std::option::Option<::std::string::String>,
    }
    impl BuildingAddress {
        pub fn builder() -> BuildingAddressBuilder {
            BuildingAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.buildings"]
    pub struct BuildingCoordinates {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Latitude in decimal degrees."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Longitude in decimal degrees."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl BuildingCoordinates {
        pub fn builder() -> BuildingCoordinatesBuilder {
            BuildingCoordinatesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.buildings"]
    pub struct Buildings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Buildings in this page of results."]
        pub buildings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Building>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ buildings_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "buildings_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Buildings {
        pub fn builder() -> BuildingsBuilder {
            BuildingsBuilder::default()
        }
    }
    mod buildings_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#buildings#buildingsList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.calendars"]
    pub struct CalendarResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for the building a resource is located in."]
        pub building_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capacity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Capacity of a resource, number of seats in a room."]
        pub capacity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etags: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instances of features for the calendar resource."]
        pub feature_instances: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floorName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the floor a resource is located on."]
        pub floor_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floorSection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the section within a floor a resource is located in."]
        pub floor_section: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generatedResourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The read-only auto-generated name of the calendar resource which includes metadata about the resource such as building name, floor, capacity, etc. For example, \"NYC-2-Training Room 1A (16)\"."]
        pub generated_resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ calendar_resource_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "calendar_resource_defaults :: kind")]
        #[doc = "The type of the resource. For calendar resources, the value is `admin#directory#resources#calendars#CalendarResource`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the calendar resource. Either CONFERENCE_ROOM or OTHER. Legacy data is set to CATEGORY_UNKNOWN."]
        pub resource_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the resource, visible only to admins."]
        pub resource_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The read-only email for the calendar resource. Generated as part of creating a new calendar resource."]
        pub resource_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for the calendar resource."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the calendar resource. For example, \"Training Room 1A\"."]
        pub resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the calendar resource, intended for non-room resources."]
        pub resource_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userVisibleDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the resource, visible to users and admins."]
        pub user_visible_description: ::std::option::Option<::std::string::String>,
    }
    impl CalendarResource {
        pub fn builder() -> CalendarResourceBuilder {
            CalendarResourceBuilder::default()
        }
    }
    mod calendar_resource_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#calendars#CalendarResource\"")
                .unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.calendars"]
    pub struct CalendarResources {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CalendarResources in this page of results."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarResource>>>,
        #[builder(default = "{ calendar_resources_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "calendar_resources_defaults :: kind")]
        #[doc = "Identifies this as a collection of CalendarResources. This is always `admin#directory#resources#calendars#calendarResourcesList`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CalendarResources {
        pub fn builder() -> CalendarResourcesBuilder {
            CalendarResourcesBuilder::default()
        }
    }
    mod calendar_resources_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#calendars#calendarResourcesList\"")
                .unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An notification channel used to watch for resource changes."]
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
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`."]
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
        #[doc = "The type of delivery mechanism used for this channel."]
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
    #[doc = "Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices)."]
    pub struct ChromeOsDevice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeTimeRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of active time ranges (Read-only)."]
        pub active_time_ranges:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceActiveTimeRanges>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedAssetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The asset identifier as noted by an administrator or specified during enrollment."]
        pub annotated_asset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address or location of the device as noted by the administrator. Maximum length is `200` characters. Empty values are allowed."]
        pub annotated_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user of the device as noted by the administrator. Maximum length is 100 characters. Empty values are allowed."]
        pub annotated_user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoUpdateExpiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Read-only) The timestamp after which the device will stop receiving Chrome updates or support"]
        pub auto_update_expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The boot mode for the device. The possible values are: * `Verified`: The device is running a valid version of the Chrome OS. * `Dev`: The devices's developer hardware switch is enabled. When booted, the device has a command line shell. For an example of a developer switch, see the [Chromebook developer information](https://www.chromium.org/chromium-os/developer-information-for-chrome-os-devices/samsung-series-5-chromebook#TOC-Developer-switch)."]
        pub boot_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuStatusReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reports of CPU utilization and temperature (Read-only)"]
        pub cpu_status_reports:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceCpuStatusReports>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of device files to download (Read-only)"]
        pub device_files: ::std::option::Option<::std::vec::Vec<ChromeOsDeviceDeviceFiles>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the Chrome device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskVolumeReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reports of disk space and other info about mounted/connected volumes."]
        pub disk_volume_reports:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceDiskVolumeReports>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockMacAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Read-only) Built-in MAC address for the docking station that the device connected to. Factory sets Media access control address (MAC address) assigned for use by a dock. It is reserved specifically for MAC pass through device policy. The format is twelve (12) hexadecimal digits without any delimiter (uppercase letters). This is only relevant for some devices."]
        pub dock_mac_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ethernetMacAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's MAC address on the ethernet network interface."]
        pub ethernet_mac_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ethernetMacAddress0")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Read-only) MAC address used by the Chromebook’s internal ethernet port, and for onboard network (ethernet) interface. The format is twelve (12) hexadecimal digits without any delimiter (uppercase letters). This is only relevant for some devices."]
        pub ethernet_mac_address0: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firmwareVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Chrome device's firmware version."]
        pub firmware_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ chrome_os_device_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "chrome_os_device_defaults :: kind")]
        #[doc = "The type of resource. For the Chromeosdevices resource, the value is `admin#directory#chromeosdevice`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastEnrollmentTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the device was last enrolled (Read-only)"]
        pub last_enrollment_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastKnownNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains last known network (Read-only)"]
        pub last_known_network:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceLastKnownNetwork>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSync")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the device was last synchronized with the policy settings in the G Suite administrator control panel (Read-only)"]
        pub last_sync: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "macAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's wireless MAC address. If the device does not have this information, it is not included in the response."]
        pub mac_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufactureDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Read-only) The date the device was manufactured in yyyy-mm-dd format."]
        pub manufacture_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Mobile Equipment Identifier (MEID) or the International Mobile Equipment Identity (IMEI) for the 3G mobile card in a mobile device. A MEID/IMEI is typically used when adding a device to a wireless carrier's post-pay service plan. If the device does not have this information, this property is not included in the response. For more information on how to export a MEID/IMEI list, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices.html#export_meid)."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's model information. If the device does not have this information, this property is not included in the response."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notes about this device added by the administrator. This property can be [searched](https://support.google.com/chrome/a/answer/1698333) with the [list](/admin-sdk/directory/v1/reference/chromeosdevices/list) method's `query` parameter. Maximum length is 500 characters. Empty values are allowed."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's order number. Only devices directly purchased from Google have an order number."]
        pub order_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full parent path with the organizational unit's name associated with the device. Path names are case insensitive. If the parent organizational unit is the top-level organization, it is represented as a forward slash, `/`. This property can be [updated](/admin-sdk/directory/v1/guides/manage-chrome-devices#update_chrome_device) using the API. For more information about how to create an organizational structure for your device, see the [administration help center](https://support.google.com/a/answer/182433)."]
        pub org_unit_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Chrome device's operating system version."]
        pub os_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Chrome device's platform version."]
        pub platform_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recentUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of recent device users, in descending order, by last login time."]
        pub recent_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RecentUsers>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Chrome device serial number entered when the device was enabled. This value is the same as the Admin console's *Serial Number* in the *Chrome OS Devices* tab."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the device."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportEndDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Final date the device will be supported (Read-only)"]
        pub support_end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemRamFreeReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reports of amounts of available RAM memory (Read-only)"]
        pub system_ram_free_reports:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceSystemRamFreeReports>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemRamTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total RAM on the device [in bytes] (Read-only)"]
        pub system_ram_total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tpmVersionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trusted Platform Module (TPM) (Read-only)"]
        pub tpm_version_info: ::std::option::Option<ChromeOsDeviceTpmVersionInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "willAutoRenew")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the device will auto renew its support after the support end date. This is a read-only property."]
        pub will_auto_renew: ::std::option::Option<::std::primitive::bool>,
    }
    impl ChromeOsDevice {
        pub fn builder() -> ChromeOsDeviceBuilder {
            ChromeOsDeviceBuilder::default()
        }
    }
    mod chrome_os_device_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#chromeosdevice\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceActiveTimeRanges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of usage in milliseconds."]
        pub active_time: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of usage"]
        pub date: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceActiveTimeRanges {
        pub fn builder() -> ChromeOsDeviceActiveTimeRangesBuilder {
            ChromeOsDeviceActiveTimeRangesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceCpuStatusReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuTemperatureInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of CPU temperature samples."]
        pub cpu_temperature_info: ::std::option::Option<
            ::std::vec::Vec<ChromeOsDeviceCpuStatusReportsCpuTemperatureInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuUtilizationPercentageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_utilization_percentage_info:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the report was received."]
        pub report_time: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceCpuStatusReports {
        pub fn builder() -> ChromeOsDeviceCpuStatusReportsBuilder {
            ChromeOsDeviceCpuStatusReportsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceCpuStatusReportsCpuTemperatureInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU label"]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "temperature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Temperature in Celsius degrees."]
        pub temperature: ::std::option::Option<::std::primitive::i64>,
    }
    impl ChromeOsDeviceCpuStatusReportsCpuTemperatureInfo {
        pub fn builder() -> ChromeOsDeviceCpuStatusReportsCpuTemperatureInfoBuilder {
            ChromeOsDeviceCpuStatusReportsCpuTemperatureInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceDeviceFiles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the file was created"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File download URL"]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File name"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File type"]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceDeviceFiles {
        pub fn builder() -> ChromeOsDeviceDeviceFilesBuilder {
            ChromeOsDeviceDeviceFilesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceDiskVolumeReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disk volumes"]
        pub volume_info:
            ::std::option::Option<::std::vec::Vec<ChromeOsDeviceDiskVolumeReportsVolumeInfo>>,
    }
    impl ChromeOsDeviceDiskVolumeReports {
        pub fn builder() -> ChromeOsDeviceDiskVolumeReportsBuilder {
            ChromeOsDeviceDiskVolumeReportsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceDiskVolumeReportsVolumeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageFree")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free disk space [in bytes]"]
        pub storage_free: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total disk space [in bytes]"]
        pub storage_total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume id"]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceDiskVolumeReportsVolumeInfo {
        pub fn builder() -> ChromeOsDeviceDiskVolumeReportsVolumeInfoBuilder {
            ChromeOsDeviceDiskVolumeReportsVolumeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information for an ip address."]
    pub struct ChromeOsDeviceLastKnownNetwork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wanIpAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The WAN IP address."]
        pub wan_ip_address: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceLastKnownNetwork {
        pub fn builder() -> ChromeOsDeviceLastKnownNetworkBuilder {
            ChromeOsDeviceLastKnownNetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceSystemRamFreeReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the report was received."]
        pub report_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemRamFreeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub system_ram_free_info: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ChromeOsDeviceSystemRamFreeReports {
        pub fn builder() -> ChromeOsDeviceSystemRamFreeReportsBuilder {
            ChromeOsDeviceSystemRamFreeReportsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Trusted Platform Module (TPM) (Read-only)"]
    pub struct ChromeOsDeviceTpmVersionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "family")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TPM family. We use the TPM 2.0 style encoding, e.g.: TPM 1.2: \"1.2\" -> 312e3200 TPM 2.0: \"2.0\" -> 322e3000"]
        pub family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firmwareVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TPM firmware version."]
        pub firmware_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TPM manufacturer code."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "specLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TPM specification level. See Library Specification for TPM 2.0 and Main Specification for TPM 1.2."]
        pub spec_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tpmModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TPM model number."]
        pub tpm_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vendorSpecific")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Vendor-specific information such as Vendor ID."]
        pub vendor_specific: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceTpmVersionInfo {
        pub fn builder() -> ChromeOsDeviceTpmVersionInfoBuilder {
            ChromeOsDeviceTpmVersionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDeviceAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action to be taken on the Chrome OS device."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprovisionReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only used when the action is `deprovision`. With the `deprovision` action, this field is required. *Note*: The deprovision reason is audited because it might have implications on licenses for perpetual subscription customers."]
        pub deprovision_reason: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDeviceAction {
        pub fn builder() -> ChromeOsDeviceActionBuilder {
            ChromeOsDeviceActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsDevices {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chromeosdevices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Chrome OS Device objects."]
        pub chromeosdevices:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChromeOsDevice>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ chrome_os_devices_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "chrome_os_devices_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access the next page of this result. To access the next page, use this token's value in the `pageToken` query string of this request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ChromeOsDevices {
        pub fn builder() -> ChromeOsDevicesBuilder {
            ChromeOsDevicesBuilder::default()
        }
    }
    mod chrome_os_devices_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#chromeosdevices\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChromeOsMoveDevicesToOu {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Chrome OS devices to be moved to OU"]
        pub device_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ChromeOsMoveDevicesToOu {
        pub fn builder() -> ChromeOsMoveDevicesToOuBuilder {
            ChromeOsMoveDevicesToOuBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Customer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's secondary contact email address. This email address cannot be on the same domain as the `customerDomain`"]
        pub alternate_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerCreationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's creation time (Readonly)"]
        pub customer_creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's primary domain name string. Do not include the `www` prefix when creating a new customer."]
        pub customer_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for the customer's Google Workspace account. (Readonly)"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ customer_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "customer_defaults :: kind")]
        #[doc = "Identifies the resource as a customer. Value: `admin#directory#customer`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's ISO 639-2 language code. See the [Language Codes](/admin-sdk/directory/v1/languages) page for the list of supported codes. Valid language codes outside the supported set will be accepted by the API but may lead to unexpected behavior. The default value is `en`."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's contact phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's postal address information."]
        pub postal_address: ::std::option::Option<::std::boxed::Box<CustomerPostalAddress>>,
    }
    impl Customer {
        pub fn builder() -> CustomerBuilder {
            CustomerBuilder::default()
        }
    }
    mod customer_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#customer\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomerPostalAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A customer's physical address. The address can be composed of one to three lines."]
        pub address_line1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Address line 2 of the address."]
        pub address_line2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Address line 3 of the address."]
        pub address_line3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer contact's name."]
        pub contact_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a required property. For `countryCode` information see the [ISO 3166 country code elements](https://www.iso.org/iso/country_codes.htm)."]
        pub country_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the locality. An example of a locality value is the city of `San Francisco`."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The company or company division name."]
        pub organization_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The postal code. A postalCode example is a postal zip code such as `10009`. This is in accordance with - http: //portablecontacts.net/draft-spec.html#address_element."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the region. An example of a region value is `NY` for the state of New York."]
        pub region: ::std::option::Option<::std::string::String>,
    }
    impl CustomerPostalAddress {
        pub fn builder() -> CustomerPostalAddressBuilder {
            CustomerPostalAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information regarding a command that was issued to a device."]
    pub struct DirectoryChromeosdevicesCommand {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the command will expire. If the device doesn't execute the command within this time the command will become expired."]
        pub command_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID of a device command."]
        pub command_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the command execution."]
        pub command_result:
            ::std::option::Option<::std::boxed::Box<DirectoryChromeosdevicesCommandResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issueTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the command was issued by the admin."]
        pub issue_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The payload that the command specified, if any."]
        pub payload: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the command state."]
        pub state: ::std::option::Option<DirectoryChromeosdevicesCommandStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the command."]
        pub _type: ::std::option::Option<DirectoryChromeosdevicesCommandTypeEnum>,
    }
    impl DirectoryChromeosdevicesCommand {
        pub fn builder() -> DirectoryChromeosdevicesCommandBuilder {
            DirectoryChromeosdevicesCommandBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the command state."]
    pub enum DirectoryChromeosdevicesCommandStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The command status was unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "An unexpired command not yet sent to the client."]
        Pending,
        #[serde(rename = "EXPIRED")]
        #[doc = "The command didn't get executed by the client within the expected time."]
        Expired,
        #[serde(rename = "CANCELLED")]
        #[doc = "The command is cancelled by admin while in PENDING."]
        Cancelled,
        #[serde(rename = "SENT_TO_CLIENT")]
        #[doc = "The command has been sent to the client."]
        SentToClient,
        #[serde(rename = "ACKED_BY_CLIENT")]
        #[doc = "The client has responded that it received the command."]
        AckedByClient,
        #[serde(rename = "EXECUTED_BY_CLIENT")]
        #[doc = "The client has (un)successfully executed the command."]
        ExecutedByClient,
    }
    impl ::std::default::Default for DirectoryChromeosdevicesCommandStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the command."]
    pub enum DirectoryChromeosdevicesCommandTypeEnum {
        #[serde(rename = "COMMAND_TYPE_UNSPECIFIED")]
        #[doc = "The command type was unspecified."]
        CommandTypeUnspecified,
        #[serde(rename = "REBOOT")]
        #[doc = "Reboot the device. Can only be issued to Kiosk and managed guest session devices."]
        Reboot,
        #[serde(rename = "TAKE_A_SCREENSHOT")]
        #[doc = "Take a screenshot of the device. Only available if the device is in Kiosk Mode."]
        TakeAScreenshot,
        #[serde(rename = "SET_VOLUME")]
        #[doc = "Set the volume of the device. Can only be issued to Kiosk and managed guest session devices."]
        SetVolume,
        #[serde(rename = "WIPE_USERS")]
        #[doc = "Wipe all the users off of the device. Executing this command in the device will remove all user profile data, but it will keep device policy and enrollment."]
        WipeUsers,
        #[serde(rename = "REMOTE_POWERWASH")]
        #[doc = "Wipes the device by performing a power wash. Executing this command in the device will remove all data including user policies, device policies and enrollment policies. Warning: This will revert the device back to a factory state with no enrollment unless the device is subject to forced or auto enrollment. Use with caution, as this is an irreversible action!"]
        RemotePowerwash,
    }
    impl ::std::default::Default for DirectoryChromeosdevicesCommandTypeEnum {
        fn default() -> Self {
            Self::CommandTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of executing a command."]
    pub struct DirectoryChromeosdevicesCommandResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error message with a short explanation as to why the command failed. Only present if the command failed."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the command was executed or failed to execute."]
        pub execute_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the command."]
        pub result: ::std::option::Option<DirectoryChromeosdevicesCommandResultResultEnum>,
    }
    impl DirectoryChromeosdevicesCommandResult {
        pub fn builder() -> DirectoryChromeosdevicesCommandResultBuilder {
            DirectoryChromeosdevicesCommandResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The result of the command."]
    pub enum DirectoryChromeosdevicesCommandResultResultEnum {
        #[serde(rename = "COMMAND_RESULT_TYPE_UNSPECIFIED")]
        #[doc = "The command result was unspecified."]
        CommandResultTypeUnspecified,
        #[serde(rename = "IGNORED")]
        #[doc = "The command was ignored as obsolete."]
        Ignored,
        #[serde(rename = "FAILURE")]
        #[doc = "The command could not be executed successfully."]
        Failure,
        #[serde(rename = "SUCCESS")]
        #[doc = "The command was successfully executed."]
        Success,
    }
    impl ::std::default::Default for DirectoryChromeosdevicesCommandResultResultEnum {
        fn default() -> Self {
            Self::CommandResultTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for issuing a command."]
    pub struct DirectoryChromeosdevicesIssueCommandRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of command."]
        pub command_type:
            ::std::option::Option<DirectoryChromeosdevicesIssueCommandRequestCommandTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The payload for the command, provide it only if command supports it. The following commands support adding payload: - SET_VOLUME: Payload is a stringified JSON object in the form: { \"volume\": 50 }. The volume has to be an integer in the range [0,100]."]
        pub payload: ::std::option::Option<::std::string::String>,
    }
    impl DirectoryChromeosdevicesIssueCommandRequest {
        pub fn builder() -> DirectoryChromeosdevicesIssueCommandRequestBuilder {
            DirectoryChromeosdevicesIssueCommandRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of command."]
    pub enum DirectoryChromeosdevicesIssueCommandRequestCommandTypeEnum {
        #[serde(rename = "COMMAND_TYPE_UNSPECIFIED")]
        #[doc = "The command type was unspecified."]
        CommandTypeUnspecified,
        #[serde(rename = "REBOOT")]
        #[doc = "Reboot the device. Can only be issued to Kiosk and managed guest session devices."]
        Reboot,
        #[serde(rename = "TAKE_A_SCREENSHOT")]
        #[doc = "Take a screenshot of the device. Only available if the device is in Kiosk Mode."]
        TakeAScreenshot,
        #[serde(rename = "SET_VOLUME")]
        #[doc = "Set the volume of the device. Can only be issued to Kiosk and managed guest session devices."]
        SetVolume,
        #[serde(rename = "WIPE_USERS")]
        #[doc = "Wipe all the users off of the device. Executing this command in the device will remove all user profile data, but it will keep device policy and enrollment."]
        WipeUsers,
        #[serde(rename = "REMOTE_POWERWASH")]
        #[doc = "Wipes the device by performing a power wash. Executing this command in the device will remove all data including user policies, device policies and enrollment policies. Warning: This will revert the device back to a factory state with no enrollment unless the device is subject to forced or auto enrollment. Use with caution, as this is an irreversible action!"]
        RemotePowerwash,
    }
    impl ::std::default::Default for DirectoryChromeosdevicesIssueCommandRequestCommandTypeEnum {
        fn default() -> Self {
            Self::CommandTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response for issuing a command."]
    pub struct DirectoryChromeosdevicesIssueCommandResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the issued command, used to retrieve the command status."]
        pub command_id: ::std::option::Option<::std::string::String>,
    }
    impl DirectoryChromeosdevicesIssueCommandResponse {
        pub fn builder() -> DirectoryChromeosdevicesIssueCommandResponseBuilder {
            DirectoryChromeosdevicesIssueCommandResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DomainAlias {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the domain alias. (Read-only)."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainAliasName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain alias name."]
        pub domain_alias_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ domain_alias_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "domain_alias_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentDomainName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parent domain name that the domain alias is associated with. This can either be a primary or secondary domain name within a customer."]
        pub parent_domain_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the verification state of a domain alias. (Read-only)"]
        pub verified: ::std::option::Option<::std::primitive::bool>,
    }
    impl DomainAlias {
        pub fn builder() -> DomainAliasBuilder {
            DomainAliasBuilder::default()
        }
    }
    mod domain_alias_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#domainAlias\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DomainAliases {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainAliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of domain alias objects."]
        pub domain_aliases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainAlias>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ domain_aliases_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "domain_aliases_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
    }
    impl DomainAliases {
        pub fn builder() -> DomainAliasesBuilder {
            DomainAliasesBuilder::default()
        }
    }
    mod domain_aliases_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#domainAliases\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Domains {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of the domain. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. (Read-only)."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainAliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of domain alias objects. (Read-only)"]
        pub domain_aliases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainAlias>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain name of the customer."]
        pub domain_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrimary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the domain is a primary domain (Read-only)."]
        pub is_primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ domains_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "domains_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the verification state of a domain. (Read-only)."]
        pub verified: ::std::option::Option<::std::primitive::bool>,
    }
    impl Domains {
        pub fn builder() -> DomainsBuilder {
            DomainsBuilder::default()
        }
    }
    mod domains_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#domain\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Domains2 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of domain objects."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Domains>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ domains2_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "domains2_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
    }
    impl Domains2 {
        pub fn builder() -> Domains2Builder {
            Domains2Builder::default()
        }
    }
    mod domains2_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#domains\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Feature object in Directory API."]
    pub struct Feature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etags: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ feature_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "feature_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the feature."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Feature {
        pub fn builder() -> FeatureBuilder {
            FeatureBuilder::default()
        }
    }
    mod feature_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#features#Feature\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a feature instance."]
    pub struct FeatureInstance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The feature that this is an instance of. A calendar resource may have multiple instances of a feature."]
        pub feature: ::std::option::Option<::std::boxed::Box<Feature>>,
    }
    impl FeatureInstance {
        pub fn builder() -> FeatureInstanceBuilder {
            FeatureInstanceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FeatureRename {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New name of the feature."]
        pub new_name: ::std::option::Option<::std::string::String>,
    }
    impl FeatureRename {
        pub fn builder() -> FeatureRenameBuilder {
            FeatureRenameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Public API: Resources.features"]
    pub struct Features {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Features in this page of results."]
        pub features: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Feature>>>,
        #[builder(default = "{ features_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "features_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Features {
        pub fn builder() -> FeaturesBuilder {
            FeaturesBuilder::default()
        }
    }
    mod features_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#resources#features#featuresList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups)."]
    pub struct Group {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adminCreated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value is `true` if this group was created by an administrator rather than a user."]
        pub admin_created: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of a group's alias email addresses."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An extended description to help users determine the purpose of a group. For example, you can include information about who should join the group, the types of messages to send to the group, links to FAQs about the group, or related groups. Maximum length is `4,096` characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directMembersCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of users that are direct members of the group. If a group is a member (child) of this group (the parent), members of the child group are not counted in the `directMembersCount` property of the parent group."]
        pub direct_members_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group's email address. If your account has multiple domains, select the appropriate domain for the email address. The `email` must be unique. This property is required when creating a group. Group email addresses are subject to the same character usage rules as usernames, see the [help center](https://support.google.com/a/answer/9193374) for details."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of a group. A group `id` can be used as a group request URI's `groupKey`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ group_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "group_defaults :: kind")]
        #[doc = "The type of the API resource. For Groups resources, the value is `admin#directory#group`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group's display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonEditableAliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the group's non-editable alias email addresses that are outside of the account's primary domain or subdomains. These are functioning email addresses used by the group. This is a read-only property returned in the API's response for a group. If edited in a group's POST or PUT request, the edit is ignored by the API service."]
        pub non_editable_aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Group {
        pub fn builder() -> GroupBuilder {
            GroupBuilder::default()
        }
    }
    mod group_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#group\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Groups {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of group objects."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
        #[builder(default = "{ groups_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "groups_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access next page of this result."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Groups {
        pub fn builder() -> GroupsBuilder {
            GroupsBuilder::default()
        }
    }
    mod groups_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#groups\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members)."]
    pub struct Member {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delivery_settings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines mail delivery preferences of member. This is only supported by create/update/get."]
        pub delivery_settings: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The member's email address. A member can be a user or another group. This property is required when adding a member to a group. The `email` must be unique and cannot be an alias of another group. If the email address is changed, the API automatically reflects the email address changes."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the group member. A member `id` can be used as a member request URI's `memberKey`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ member_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "member_defaults :: kind")]
        #[doc = "The type of the API resource. For Members resources, the value is `admin#directory#member`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The member's role in a group. The API returns an error for cycles in group memberships. For example, if `group1` is a member of `group2`, `group2` cannot be a member of `group1`. For more information about a member's role, see the [administration help center](https://support.google.com/a/answer/167094)."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of member (Immutable)"]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of group member."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Member {
        pub fn builder() -> MemberBuilder {
            MemberBuilder::default()
        }
    }
    mod member_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#member\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Members {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ members_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "members_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of member objects."]
        pub members: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Member>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access next page of this result."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Members {
        pub fn builder() -> MembersBuilder {
            MembersBuilder::default()
        }
    }
    mod members_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#members\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Has Member response in Directory API."]
    pub struct MembersHasMember {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isMember")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Identifies whether the given user is a member of the group. Membership can be direct or nested."]
        pub is_member: ::std::option::Option<::std::primitive::bool>,
    }
    impl MembersHasMember {
        pub fn builder() -> MembersHasMemberBuilder {
            MembersHasMemberBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html)."]
    pub struct MobileDevice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adbStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adb (USB debugging) enabled or disabled on device (Read-only)"]
        pub adb_status: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of applications installed on an Android mobile device. It is not applicable to Google Sync and iOS devices. The list includes any Android applications that access Google Workspace data. When updating an applications list, it is important to note that updates replace the existing list. If the Android device has two existing applications and the API updates the list with five applications, the is now the updated list of five applications."]
        pub applications: ::std::option::Option<::std::vec::Vec<MobileDeviceApplications>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basebandVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's baseband version."]
        pub baseband_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootloaderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device Bootloader version (Read-only)"]
        pub bootloader_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device Brand (Read-only)"]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's operating system build number."]
        pub build_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default locale used on the device."]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developerOptionsStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Developer options enabled or disabled on device (Read-only)"]
        pub developer_options_status: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceCompromisedStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The compromised device status."]
        pub device_compromised_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The serial number for a Google Sync mobile device. For Android and iOS devices, this is a software generated unique identifier."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devicePasswordStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DevicePasswordStatus (Read-only)"]
        pub device_password_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of owner's email addresses. If your application needs the current list of user emails, use the [get](/admin-sdk/directory/v1/reference/mobiledevices/get.html) method. For additional information, see the [retrieve a user](/admin-sdk/directory/v1/guides/manage-users#get_user) method."]
        pub email: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device Encryption Status (Read-only)"]
        pub encryption_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstSync")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the device was first synchronized with the policy settings in the G Suite administrator control panel (Read-only)"]
        pub first_sync: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardware")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device Hardware (Read-only)"]
        pub hardware: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardwareId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IMEI/MEID unique identifier for Android hardware. It is not applicable to Google Sync devices. When adding an Android mobile device, this is an optional property. When updating one of these devices, this is a read-only property."]
        pub hardware_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imei")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's IMEI number."]
        pub imei: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kernelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's kernel version."]
        pub kernel_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ mobile_device_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "mobile_device_defaults :: kind")]
        #[doc = "The type of the API resource. For Mobiledevices resources, the value is `admin#directory#mobiledevice`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSync")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time the device was last synchronized with the policy settings in the G Suite administrator control panel (Read-only)"]
        pub last_sync: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedAccountIsOnOwnerProfile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean indicating if this account is on owner/primary profile or not."]
        pub managed_account_is_on_owner_profile: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device manufacturer (Read-only)"]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's MEID number."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mobile device's model name, for example Nexus S. This property can be [updated](/admin-sdk/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile=devices#update_mobile_device)."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the owner's user names. If your application needs the current list of device owner names, use the [get](/admin-sdk/directory/v1/reference/mobiledevices/get.html) method. For more information about retrieving mobile device user information, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-users#get_user)."]
        pub name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkOperator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device mobile or network operator (if available) (Read-only)"]
        pub network_operator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "os")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mobile device's operating system, for example IOS 4.3 or Android 2.3.5. This property can be [updated](/admin-sdk/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices#update_mobile_device)."]
        pub os: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherAccountsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of accounts added on device (Read-only)"]
        pub other_accounts_info: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privilege")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DMAgentPermission (Read-only)"]
        pub privilege: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device release version version (Read-only)"]
        pub release_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID the API service uses to identify the mobile device."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityPatchLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile Device Security patch level (Read-only)"]
        pub security_patch_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's serial number."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's status."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsWorkProfile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Work profile supported on device (Read-only)"]
        pub supports_work_profile: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of mobile device."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unknownSourcesStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unknown sources enabled or disabled on device (Read-only)"]
        pub unknown_sources_status: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gives information about the device such as `os` version. This property can be [updated](/admin-sdk/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices#update_mobile_device)."]
        pub user_agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiMacAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device's MAC address on Wi-Fi networks."]
        pub wifi_mac_address: ::std::option::Option<::std::string::String>,
    }
    impl MobileDevice {
        pub fn builder() -> MobileDeviceBuilder {
            MobileDeviceBuilder::default()
        }
    }
    mod mobile_device_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#mobiledevice\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MobileDeviceApplications {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's display name. An example is `Browser`."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's package name. An example is `com.android.browser`."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of permissions of this application. These can be either a standard Android permission or one defined by the application, and are found in an application's [Android manifest](https://developer.android.com/guide/topics/manifest/uses-permission-element.html). Examples of a Calendar application's permissions are `READ_CALENDAR`, or `MANAGE_ACCOUNTS`."]
        pub permission: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's version code. An example is `13`."]
        pub version_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's version name. An example is `3.2-140714`."]
        pub version_name: ::std::option::Option<::std::string::String>,
    }
    impl MobileDeviceApplications {
        pub fn builder() -> MobileDeviceApplicationsBuilder {
            MobileDeviceApplicationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MobileDeviceAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action to be performed on the device."]
        pub action: ::std::option::Option<::std::string::String>,
    }
    impl MobileDeviceAction {
        pub fn builder() -> MobileDeviceActionBuilder {
            MobileDeviceActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MobileDevices {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ mobile_devices_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "mobile_devices_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobiledevices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Mobile Device objects."]
        pub mobiledevices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileDevice>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access next page of this result."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl MobileDevices {
        pub fn builder() -> MobileDevicesBuilder {
            MobileDevicesBuilder::default()
        }
    }
    mod mobile_devices_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#mobiledevices\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html)."]
    pub struct OrgUnit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockInheritance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if a sub-organizational unit can inherit the settings of the parent organization. The default value is `false`, meaning a sub-organizational unit inherits the settings of the nearest parent organizational unit. For more information on inheritance and users in an organization structure, see the [administration help center](https://support.google.com/a/answer/4352075)."]
        pub block_inheritance: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the organizational unit."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ org_unit_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "org_unit_defaults :: kind")]
        #[doc = "The type of the API resource. For Orgunits resources, the value is `admin#directory#orgUnit`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizational unit's path name. For example, an organizational unit's name within the /corp/support/sales_support parent path is sales_support. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the organizational unit."]
        pub org_unit_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full path to the organizational unit. The `orgUnitPath` is a derived property. When listed, it is derived from `parentOrgunitPath` and organizational unit's `name`. For example, for an organizational unit named 'apps' under parent organization '/engineering', the orgUnitPath is '/engineering/apps'. In order to edit an `orgUnitPath`, either update the name of the organization or the `parentOrgunitPath`. A user's organizational unit determines which Google Workspace services the user has access to. If the user is moved to a new organization, the user's access changes. For more information about organization structures, see the [administration help center](https://support.google.com/a/answer/4352075). For more information about moving a user to a different organization, see [Update a user](/admin-sdk/directory/v1/guides/manage-users.html#update_user)."]
        pub org_unit_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentOrgUnitId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the parent organizational unit. Required, unless `parentOrgUnitPath` is set."]
        pub parent_org_unit_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentOrgUnitPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizational unit's parent path. For example, /corp/sales is the parent path for /corp/sales/sales_support organizational unit. Required, unless `parentOrgUnitId` is set."]
        pub parent_org_unit_path: ::std::option::Option<::std::string::String>,
    }
    impl OrgUnit {
        pub fn builder() -> OrgUnitBuilder {
            OrgUnitBuilder::default()
        }
    }
    mod org_unit_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#orgUnit\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrgUnits {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ org_units_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "org_units_defaults :: kind")]
        #[doc = "The type of the API resource. For Org Unit resources, the type is `admin#directory#orgUnits`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizationUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of organizational unit objects."]
        pub organization_units: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrgUnit>>>,
    }
    impl OrgUnits {
        pub fn builder() -> OrgUnitsBuilder {
            OrgUnitsBuilder::default()
        }
    }
    mod org_units_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#orgUnits\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Privilege {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childPrivileges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of child privileges. Privileges for a service form a tree. Each privilege can have a list of child privileges; this list is empty for a leaf privilege."]
        pub child_privileges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Privilege>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isOuScopable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the privilege can be restricted to an organization unit."]
        pub is_ou_scopable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ privilege_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "privilege_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#privilege`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privilegeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the privilege."]
        pub privilege_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated ID of the service this privilege is for. This value is returned with [`Privileges.list()`](/admin-sdk/directory/v1/reference/privileges/list)."]
        pub service_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the service this privilege is for."]
        pub service_name: ::std::option::Option<::std::string::String>,
    }
    impl Privilege {
        pub fn builder() -> PrivilegeBuilder {
            PrivilegeBuilder::default()
        }
    }
    mod privilege_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#privilege\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Privileges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Privilege resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Privilege>>>,
        #[builder(default = "{ privileges_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "privileges_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#privileges`."]
        pub kind: ::std::string::String,
    }
    impl Privileges {
        pub fn builder() -> PrivilegesBuilder {
            PrivilegesBuilder::default()
        }
    }
    mod privileges_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#privileges\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of recent device users, in descending order, by last login time."]
    pub struct RecentUsers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's email address. This is only present if the user type is `USER_TYPE_MANAGED`."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the user."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl RecentUsers {
        pub fn builder() -> RecentUsersBuilder {
            RecentUsersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Role {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSuperAdminRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns `true` if the role is a super admin role."]
        pub is_super_admin_role: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSystemRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns `true` if this is a pre-defined system role."]
        pub is_system_role: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ role_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "role_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#role`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of the role."]
        pub role_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the role."]
        pub role_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the role."]
        pub role_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rolePrivileges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of privileges that are granted to this role."]
        pub role_privileges: ::std::option::Option<::std::vec::Vec<RoleRolePrivileges>>,
    }
    impl Role {
        pub fn builder() -> RoleBuilder {
            RoleBuilder::default()
        }
    }
    mod role_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#role\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RoleRolePrivileges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privilegeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the privilege."]
        pub privilege_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated ID of the service this privilege is for. This value is returned with [`Privileges.list()`](/admin-sdk/directory/v1/reference/privileges/list)."]
        pub service_id: ::std::option::Option<::std::string::String>,
    }
    impl RoleRolePrivileges {
        pub fn builder() -> RoleRolePrivilegesBuilder {
            RoleRolePrivilegesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RoleAssignment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignedTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the user this role is assigned to."]
        pub assigned_to: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ role_assignment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "role_assignment_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#roleAssignment`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the role is restricted to an organization unit, this contains the ID for the organization unit the exercise of this role is restricted to."]
        pub org_unit_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleAssignmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of this roleAssignment."]
        pub role_assignment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the role that is assigned."]
        pub role_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scope in which this role is assigned."]
        pub scope_type: ::std::option::Option<::std::string::String>,
    }
    impl RoleAssignment {
        pub fn builder() -> RoleAssignmentBuilder {
            RoleAssignmentBuilder::default()
        }
    }
    mod role_assignment_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#roleAssignment\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RoleAssignments {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of RoleAssignment resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RoleAssignment>>>,
        #[builder(default = "{ role_assignments_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "role_assignments_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#roleAssignments`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl RoleAssignments {
        pub fn builder() -> RoleAssignmentsBuilder {
            RoleAssignmentsBuilder::default()
        }
    }
    mod role_assignments_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#roleAssignments\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Roles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Role resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Role>>>,
        #[builder(default = "{ roles_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "roles_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#roles`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Roles {
        pub fn builder() -> RolesBuilder {
            RolesBuilder::default()
        }
    }
    mod roles_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#roles\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The type of API resource. For Schema resources, this is always `admin#directory#schema`."]
    pub struct Schema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for the schema."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of fields in the schema."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SchemaFieldSpec>>>,
        #[builder(default = "{ schema_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "schema_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the schema (Read-only)"]
        pub schema_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema's name."]
        pub schema_name: ::std::option::Option<::std::string::String>,
    }
    impl Schema {
        pub fn builder() -> SchemaBuilder {
            SchemaBuilder::default()
        }
    }
    mod schema_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#schema\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas)."]
    pub struct SchemaFieldSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display Name of the field."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the field."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the field (Read-only)"]
        pub field_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the field."]
        pub field_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the field."]
        pub field_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ schema_field_spec_defaults :: indexed () }", setter(into))]
        #[serde(rename = "indexed")]
        #[serde(default = "schema_field_spec_defaults :: indexed")]
        #[doc = "Boolean specifying whether the field is indexed or not. Default: `true`."]
        pub indexed: ::std::primitive::bool,
        #[builder(default = "{ schema_field_spec_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "schema_field_spec_defaults :: kind")]
        #[doc = "The kind of resource this is. For schema fields this is always `admin#directory#schema#fieldspec`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiValued")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean specifying whether this is a multi-valued field or not. Default: `false`."]
        pub multi_valued: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericIndexingSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indexing spec for a numeric field. By default, only exact match queries will be supported for numeric fields. Setting the `numericIndexingSpec` allows range queries to be supported."]
        pub numeric_indexing_spec: ::std::option::Option<SchemaFieldSpecNumericIndexingSpec>,
        #[builder(
            default = "{ schema_field_spec_defaults :: read_access_type () }",
            setter(into)
        )]
        #[serde(rename = "readAccessType")]
        #[serde(default = "schema_field_spec_defaults :: read_access_type")]
        #[doc = "Specifies who can view values of this field. See [Retrieve users as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin) for more information. Note: It may take up to 24 hours for changes to this field to be reflected."]
        pub read_access_type: ::std::string::String,
    }
    impl SchemaFieldSpec {
        pub fn builder() -> SchemaFieldSpecBuilder {
            SchemaFieldSpecBuilder::default()
        }
    }
    mod schema_field_spec_defaults {
        pub fn indexed() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#schema#fieldspec\"").unwrap()
        }
        pub fn read_access_type() -> ::std::string::String {
            serde_json::from_str(&"\"ALL_DOMAIN_USERS\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indexing spec for a numeric field. By default, only exact match queries will be supported for numeric fields. Setting the `numericIndexingSpec` allows range queries to be supported."]
    pub struct SchemaFieldSpecNumericIndexingSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum value of this field. This is meant to be indicative rather than enforced. Values outside this range will still be indexed, but search may not be as performant."]
        pub max_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum value of this field. This is meant to be indicative rather than enforced. Values outside this range will still be indexed, but search may not be as performant."]
        pub min_value: ::std::option::Option<::std::primitive::f64>,
    }
    impl SchemaFieldSpecNumericIndexingSpec {
        pub fn builder() -> SchemaFieldSpecNumericIndexingSpecBuilder {
            SchemaFieldSpecNumericIndexingSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON response template for List Schema operation in Directory API."]
    pub struct Schemas {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ schemas_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "schemas_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of UserSchema objects."]
        pub schemas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Schema>>>,
    }
    impl Schemas {
        pub fn builder() -> SchemasBuilder {
            SchemasBuilder::default()
        }
    }
    mod schemas_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#schemas\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for token resource in Directory API."]
    pub struct Token {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anonymous")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the application is registered with Google. The value is `true` if the application has an anonymous Client ID."]
        pub anonymous: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Client ID of the application the token is issued to."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The displayable name of the application the token is issued to."]
        pub display_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ token_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "token_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#token`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nativeApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the token is issued to an installed application. The value is `true` if the application is installed to a desktop or mobile device."]
        pub native_app: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of authorization scopes the application is granted."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the user that issued the token."]
        pub user_key: ::std::option::Option<::std::string::String>,
    }
    impl Token {
        pub fn builder() -> TokenBuilder {
            TokenBuilder::default()
        }
    }
    mod token_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#token\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON response template for List tokens operation in Directory API."]
    pub struct Tokens {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Token resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Token>>>,
        #[builder(default = "{ tokens_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "tokens_defaults :: kind")]
        #[doc = "The type of the API resource. This is always `admin#directory#tokenList`."]
        pub kind: ::std::string::String,
    }
    impl Tokens {
        pub fn builder() -> TokensBuilder {
            TokensBuilder::default()
        }
    }
    mod tokens_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#tokenList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html)."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the user's addresses. The maximum allowed data size for this field is 10Kb."]
        pub addresses: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agreedToTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This property is `true` if the user has completed an initial login and accepted the Terms of Service agreement."]
        pub agreed_to_terms: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. List of the user's alias email addresses."]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "archived")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if user is archived."]
        pub archived: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changePasswordAtNextLogin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the user is forced to change their password at next login. This setting doesn't apply when [the user signs in via a third-party identity provider](https://support.google.com/a/answer/60224)."]
        pub change_password_at_next_login: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's G Suite account creation time. (Read-only)"]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customSchemas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom fields of the user."]
        pub custom_schemas: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<UserCustomProperties>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The customer ID to [retrieve all account users](/admin-sdk/directory/v1/guides/manage-users.html#get_all_users). You can use the alias `my_customer` to represent your account's `customerId`. As a reseller administrator, you can use the resold customer account's `customerId`. To get a `customerId`, use the account's primary domain in the `domain` parameter of a [users.list](/admin-sdk/directory/v1/reference/users/list) request."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deletion_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the user's email addresses. The maximum allowed data size for this field is 10Kb."]
        pub emails: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of external IDs for the user, such as an employee or network ID. The maximum allowed data size for this field is 2Kb."]
        pub external_ids: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's gender. The maximum allowed data size for this field is 1Kb."]
        pub gender: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hashFunction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores the hash format of the password property. We recommend sending the `password` property value as a base 16 bit hexadecimal-encoded hash value. Set the `hashFunction` values as either the [SHA-1](https://wikipedia.org/wiki/SHA-1), [MD5](https://wikipedia.org/wiki/MD5), or [crypt](https://en.wikipedia.org/wiki/Crypt_\\(C\\)) hash format."]
        pub hash_function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for the user. A user `id` can be used as a user request URI's `userKey`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's Instant Messenger (IM) accounts. A user account can have multiple ims properties. But, only one of these ims properties can be the primary IM contact. The maximum allowed data size for this field is 2Kb."]
        pub ims: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeInGlobalAddressList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the user's profile is visible in the Google Workspace global address list when the contact sharing feature is enabled for the domain. For more information about excluding user profiles, see the [administration help center](https://support.google.com/a/answer/1285988)."]
        pub include_in_global_address_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipWhitelisted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, the user's IP address is [whitelisted](https://support.google.com/a/answer/60752)."]
        pub ip_whitelisted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAdmin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates a user with super admininistrator privileges. The `isAdmin` property can only be edited in the [Make a user an administrator](/admin-sdk/directory/v1/guides/manage-users.html#make_admin) operation ( [makeAdmin](/admin-sdk/directory/v1/reference/users/makeAdmin.html) method). If edited in the user [insert](/admin-sdk/directory/v1/reference/users/insert.html) or [update](/admin-sdk/directory/v1/reference/users/update.html) methods, the edit is ignored by the API service."]
        pub is_admin: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDelegatedAdmin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates if the user is a delegated administrator. Delegated administrators are supported by the API but cannot create or undelete users, or make users administrators. These requests are ignored by the API service. Roles and privileges for administrators are assigned using the [Admin console](https://support.google.com/a/answer/33325)."]
        pub is_delegated_admin: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEnforcedIn2Sv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Is 2-step verification enforced (Read-only)"]
        pub is_enforced_in2_sv: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEnrolledIn2Sv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Is enrolled in 2-step verification (Read-only)"]
        pub is_enrolled_in2_sv: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isMailboxSetup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates if the user's Google mailbox is created. This property is only applicable if the user has been assigned a Gmail license."]
        pub is_mailbox_setup: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keywords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's keywords. The maximum allowed data size for this field is 1Kb."]
        pub keywords: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ user_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "user_defaults :: kind")]
        #[doc = "Output only. The type of the API resource. For Users resources, the value is `admin#directory#user`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's languages. The maximum allowed data size for this field is 1Kb."]
        pub languages: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastLoginTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's last login time. (Read-only)"]
        pub last_login_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's locations. The maximum allowed data size for this field is 10Kb."]
        pub locations: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the given and family names of the user, and the read-only `fullName` value. The maximum number of characters in the `givenName` and in the `familyName` values is 60. In addition, name values support unicode/UTF-8 characters, and can contain spaces, letters (a-z), numbers (0-9), dashes (-), forward slashes (/), and periods (.). For more information about character usage rules, see the [administration help center](https://support.google.com/a/answer/9193374). Maximum allowed data size for this field is 1Kb."]
        pub name: ::std::option::Option<::std::boxed::Box<UserName>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonEditableAliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. List of the user's non-editable alias email addresses. These are typically outside the account's primary domain or sub-domain."]
        pub non_editable_aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notes for the user."]
        pub notes: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full path of the parent organization associated with the user. If the parent organization is the top-level, it is represented as a forward slash (`/`)."]
        pub org_unit_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of organizations the user belongs to. The maximum allowed data size for this field is 10Kb."]
        pub organizations: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's password"]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the user's phone numbers. The maximum allowed data size for this field is 1Kb."]
        pub phones: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posixAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of [POSIX](https://www.opengroup.org/austin/papers/posix_faq.html) account information for the user."]
        pub posix_accounts: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's primary email address. This property is required in a request to create a user account. The `primaryEmail` must be unique and cannot be an alias of another user."]
        pub primary_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recoveryEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recovery email of the user."]
        pub recovery_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recoveryPhone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recovery phone of the user. The phone number must be in the E.164 format, starting with the plus sign (+). Example: *+16506661212*."]
        pub recovery_phone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the user's relationships to other users. The maximum allowed data size for this field is 2Kb."]
        pub relations: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sshPublicKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of SSH public keys."]
        pub ssh_public_keys: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspended")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if user is suspended."]
        pub suspended: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspensionReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Has the reason a user account is suspended either by the administrator or by Google at the time of suspension. The property is returned only if the `suspended` property is `true`."]
        pub suspension_reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailPhotoEtag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ETag of the user's photo (Read-only)"]
        pub thumbnail_photo_etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailPhotoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Photo Url of the user (Read-only)"]
        pub thumbnail_photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's websites. The maximum allowed data size for this field is 2Kb."]
        pub websites: ::std::option::Option<::serde_json::Value>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    mod user_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#user\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for About (notes) of a user in Directory API."]
    pub struct UserAbout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "About entry can have a type which indicates the content type. It can either be plain or html. By default, notes contents are assumed to contain plain text."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actual value of notes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserAbout {
        pub fn builder() -> UserAboutBuilder {
            UserAboutBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for address."]
    pub struct UserAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country code."]
        pub country_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extendedAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extended Address."]
        pub extended_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formatted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Formatted address."]
        pub formatted: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locality."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "poBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other parts of address."]
        pub po_box: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal code."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary address. Only one entry could be marked as primary."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Region."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceIsStructured")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User supplied address was structured. Structured addresses are NOT supported at this time. You might be able to write structured addresses but any values will eventually be clobbered."]
        pub source_is_structured: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streetAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Street."]
        pub street_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard values of that entry. For example address could be of home work etc. In addition to the standard type an entry can have a custom type and can take any value. Such type should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserAddress {
        pub fn builder() -> UserAddressBuilder {
            UserAddressBuilder::default()
        }
    }
    #[doc = "JSON template for a set of custom properties (i.e. all fields in a particular schema)"]
    pub type UserCustomProperties = ::std::collections::BTreeMap<String, ::serde_json::Value>;
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an email."]
    pub struct UserEmail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email id of the user."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary email. Only one entry could be marked as primary."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example email could be of home, work etc. In addition to the standard type, an entry can have a custom type and can take any value Such types should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserEmail {
        pub fn builder() -> UserEmailBuilder {
            UserEmailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an externalId entry."]
    pub struct UserExternalId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the Id."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the id."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserExternalId {
        pub fn builder() -> UserExternalIdBuilder {
            UserExternalIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserGender {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressMeAs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "AddressMeAs. A human-readable string containing the proper way to refer to the profile owner by humans for example he/him/his or they/them/their."]
        pub address_me_as: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customGender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom gender."]
        pub custom_gender: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gender."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserGender {
        pub fn builder() -> UserGenderBuilder {
            UserGenderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for instant messenger of an user."]
    pub struct UserIm {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customProtocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom protocol."]
        pub custom_protocol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "im")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instant messenger id."]
        pub im: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary im. Only one entry could be marked as primary."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Protocol used in the instant messenger. It should be one of the values from ImProtocolTypes map. Similar to type it can take a CUSTOM value and specify the custom name in customProtocol field."]
        pub protocol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example instant messengers could be of home work etc. In addition to the standard type an entry can have a custom type and can take any value. Such types should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserIm {
        pub fn builder() -> UserImBuilder {
            UserImBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a keyword entry."]
    pub struct UserKeyword {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard type of that entry. For example keyword could be of type occupation or outlook. In addition to the standard type an entry can have a custom type and can give it any name. Such types should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Keyword."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserKeyword {
        pub fn builder() -> UserKeywordBuilder {
            UserKeywordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a language entry."]
    pub struct UserLanguage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other language. User can provide own language name if there is no corresponding Google III language code. If this is set LanguageCode can't be set"]
        pub custom_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language Code. Should be used for storing Google III LanguageCode string representation for language. Illegal values cause SchemaException."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl UserLanguage {
        pub fn builder() -> UserLanguageBuilder {
            UserLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a location entry."]
    pub struct UserLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "area")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual location. This is most useful for display purposes to concisely describe the location. For example 'Mountain View, CA', 'Near Seattle', 'US-NYC-9TH 9A209A.''"]
        pub area: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Building Identifier."]
        pub building_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deskCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Most specific textual code of individual desk location."]
        pub desk_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floorName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floor name/number."]
        pub floor_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floorSection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floor section. More specific location within the floor. For example if a floor is divided into sections 'A', 'B' and 'C' this field would identify one of those values."]
        pub floor_section: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example location could be of types default and desk. In addition to standard type an entry can have a custom type and can give it any name. Such types should have 'custom' as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserLocation {
        pub fn builder() -> UserLocationBuilder {
            UserLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserMakeAdmin {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the administrator status of the user."]
        pub status: ::std::option::Option<::std::primitive::bool>,
    }
    impl UserMakeAdmin {
        pub fn builder() -> UserMakeAdminBuilder {
            UserMakeAdminBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's last name. Required when creating a user account."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's full name formed by concatenating the first and last name values."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "givenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's first name. Required when creating a user account."]
        pub given_name: ::std::option::Option<::std::string::String>,
    }
    impl UserName {
        pub fn builder() -> UserNameBuilder {
            UserNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an organization entry."]
    pub struct UserOrganization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "costCenter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cost center of the users department."]
        pub cost_center: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "department")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Department within the organization."]
        pub department: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the organization."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain to which the organization belongs to."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTimeEquivalent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full-time equivalent millipercent within the organization (100000 = 100%)."]
        pub full_time_equivalent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the organization. This need not be fully qualified address."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the organization"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If it user's primary organization."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Symbol of the organization."]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title (designation) of the user in the organization."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example organization could be of school work etc. In addition to the standard type an entry can have a custom type and can give it any name. Such types should have the CUSTOM value as type and also have a CustomType value."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UserOrganization {
        pub fn builder() -> UserOrganizationBuilder {
            UserOrganizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a phone entry."]
    pub struct UserPhone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary phone or not."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example phone could be of home_fax work mobile etc. In addition to the standard type an entry can have a custom type and can give it any name. Such types should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Phone number."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserPhone {
        pub fn builder() -> UserPhoneBuilder {
            UserPhoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserPhoto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the photo in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID the API uses to uniquely identify the user."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ user_photo_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "user_photo_defaults :: kind")]
        #[doc = "The type of the API resource. For Photo resources, this is `admin#directory#user#photo`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the photo. Allowed values are `JPEG`, `PNG`, `GIF`, `BMP`, `TIFF`, and web-safe base64 encoding."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user photo's upload data in [web-safe Base64](https://en.wikipedia.org/wiki/Base64#URL_applications) format in bytes. This means: * The slash (/) character is replaced with the underscore (_) character. * The plus sign (+) character is replaced with the hyphen (-) character. * The equals sign (=) character is replaced with the asterisk (*). * For padding, the period (.) character is used instead of the RFC-4648 baseURL definition which uses the equals sign (=) for padding. This is done to simplify URL-parsing. * Whatever the size of the photo being uploaded, the API downsizes it to 96x96 pixels."]
        pub photo_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's primary email address."]
        pub primary_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the photo in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl UserPhoto {
        pub fn builder() -> UserPhotoBuilder {
            UserPhotoBuilder::default()
        }
    }
    mod user_photo_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#user#photo\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a POSIX account entry."]
    pub struct UserPosixAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A POSIX account field identifier."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gecos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GECOS (user information) for this account."]
        pub gecos: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default group ID."]
        pub gid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "homeDirectory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to the home directory for this account."]
        pub home_directory: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatingSystemType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operating system type for this account."]
        pub operating_system_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary account within the SystemId."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to the login shell for this account."]
        pub shell: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System identifier for which account Username or Uid apply to."]
        pub system_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The POSIX compliant user ID."]
        pub uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username of the account."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl UserPosixAccount {
        pub fn builder() -> UserPosixAccountBuilder {
            UserPosixAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a relation entry."]
    pub struct UserRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relation of the user. Some of the possible values are mother father sister brother manager assistant partner."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the relation."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserRelation {
        pub fn builder() -> UserRelationBuilder {
            UserRelationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a POSIX account entry."]
    pub struct UserSshPublicKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTimeUsec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An expiration time in microseconds since epoch."]
        pub expiration_time_usec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SHA-256 fingerprint of the SSH public key. (Read-only)"]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An SSH public key."]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl UserSshPublicKey {
        pub fn builder() -> UserSshPublicKeyBuilder {
            UserSshPublicKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserUndelete {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OrgUnit of User"]
        pub org_unit_path: ::std::option::Option<::std::string::String>,
    }
    impl UserUndelete {
        pub fn builder() -> UserUndeleteBuilder {
            UserUndeleteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a website entry."]
    pub struct UserWebsite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Type."]
        pub custom_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is user's primary website or not."]
        pub primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each entry can have a type which indicates standard types of that entry. For example website could be of home work blog etc. In addition to the standard type an entry can have a custom type and can give it any name. Such types should have the CUSTOM value as type and also have a customType value."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Website."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl UserWebsite {
        pub fn builder() -> UserWebsiteBuilder {
            UserWebsiteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Users {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ users_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "users_defaults :: kind")]
        #[doc = "Kind of resource this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access next page of this result."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger_event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event that triggered this response (only used in case of Push Response)"]
        pub trigger_event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of user objects."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
    }
    impl Users {
        pub fn builder() -> UsersBuilder {
            UsersBuilder::default()
        }
    }
    mod users_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#users\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Directory API allows you to view, generate, and invalidate backup verification codes for a user."]
    pub struct VerificationCode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ verification_code_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "verification_code_defaults :: kind")]
        #[doc = "The type of the resource. This is always `admin#directory#verificationCode`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated unique ID of the user."]
        pub user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A current verification code for the user. Invalidated or used verification codes are not returned as part of the result."]
        pub verification_code: ::std::option::Option<::std::string::String>,
    }
    impl VerificationCode {
        pub fn builder() -> VerificationCodeBuilder {
            VerificationCodeBuilder::default()
        }
    }
    mod verification_code_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#verificationCode\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON response template for List verification codes operation in Directory API."]
    pub struct VerificationCodes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of verification code resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VerificationCode>>>,
        #[builder(default = "{ verification_codes_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "verification_codes_defaults :: kind")]
        #[doc = "The type of the resource. This is always `admin#directory#verificationCodesList`."]
        pub kind: ::std::string::String,
    }
    impl VerificationCodes {
        pub fn builder() -> VerificationCodesBuilder {
            VerificationCodesBuilder::default()
        }
    }
    mod verification_codes_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"admin#directory#verificationCodesList\"").unwrap()
        }
    }
}
