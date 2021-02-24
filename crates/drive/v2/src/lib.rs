#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
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
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod about {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: include_subscribed () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeSubscribed")]
                    #[serde(default = "query_parameters_defaults :: include_subscribed")]
                    #[doc = "Whether to count changes outside the My Drive hierarchy. When set to false, changes to files such as those in the Application Data folder or shared files which have not been added to My Drive will be omitted from the maxChangeIdCount."]
                    pub include_subscribed: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_change_id_count () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxChangeIdCount")]
                    #[serde(default = "query_parameters_defaults :: max_change_id_count")]
                    #[doc = "Maximum number of remaining change IDs to count"]
                    pub max_change_id_count: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startChangeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Change ID to start counting from when calculating number of remaining change IDs"]
                    pub start_change_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_subscribed() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn max_change_id_count() -> ::std::string::String {
                        String::from("1")
                    }
                }
            }
        }
    }
    pub mod apps {
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
                        default = "{ query_parameters_defaults :: app_filter_extensions () }",
                        setter(into)
                    )]
                    #[serde(rename = "appFilterExtensions")]
                    #[serde(default = "query_parameters_defaults :: app_filter_extensions")]
                    #[doc = "A comma-separated list of file extensions for open with filtering. All apps within the given app query scope which can open any of the given file extensions will be included in the response. If appFilterMimeTypes are provided as well, the result is a union of the two resulting app lists."]
                    pub app_filter_extensions: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: app_filter_mime_types () }",
                        setter(into)
                    )]
                    #[serde(rename = "appFilterMimeTypes")]
                    #[serde(default = "query_parameters_defaults :: app_filter_mime_types")]
                    #[doc = "A comma-separated list of MIME types for open with filtering. All apps within the given app query scope which can open any of the given MIME types will be included in the response. If appFilterExtensions are provided as well, the result is a union of the two resulting app lists."]
                    pub app_filter_mime_types: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/)."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn app_filter_extensions() -> ::std::string::String {
                        String::from("")
                    }
                    pub fn app_filter_mime_types() -> ::std::string::String {
                        String::from("")
                    }
                }
            }
        }
    }
    pub mod changes {
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
                    #[serde(rename = "driveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The shared drive from which the change is returned."]
                    pub drive_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teamDriveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated use driveId instead."]
                    pub team_drive_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
            pub mod get_start_page_token {
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
                    #[serde(rename = "driveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive is returned."]
                    pub drive_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teamDriveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated use driveId instead."]
                    pub team_drive_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[serde(rename = "driveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier."]
                    pub drive_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_corpus_removals () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeCorpusRemovals")]
                    #[serde(default = "query_parameters_defaults :: include_corpus_removals")]
                    #[doc = "Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file."]
                    pub include_corpus_removals: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_items_from_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeItemsFromAllDrives")]
                    #[serde(
                        default = "query_parameters_defaults :: include_items_from_all_drives"
                    )]
                    #[doc = "Whether both My Drive and shared drive items should be included in results."]
                    pub include_items_from_all_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_subscribed () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeSubscribed")]
                    #[serde(default = "query_parameters_defaults :: include_subscribed")]
                    #[doc = "Whether to include changes outside the My Drive hierarchy in the result. When set to false, changes to files such as those in the Application Data folder or shared files which have not been added to My Drive are omitted from the result."]
                    pub include_subscribed: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_team_drive_items () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeTeamDriveItems")]
                    #[serde(default = "query_parameters_defaults :: include_team_drive_items")]
                    #[doc = "Deprecated use includeItemsFromAllDrives instead."]
                    pub include_team_drive_items: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of changes to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "spaces")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of spaces to query. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startChangeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated - use pageToken instead."]
                    pub start_change_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teamDriveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated use driveId instead."]
                    pub team_drive_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_corpus_removals() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_items_from_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_subscribed() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_team_drive_items() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[serde(rename = "driveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier."]
                    pub drive_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_corpus_removals () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeCorpusRemovals")]
                    #[serde(default = "query_parameters_defaults :: include_corpus_removals")]
                    #[doc = "Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file."]
                    pub include_corpus_removals: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_items_from_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeItemsFromAllDrives")]
                    #[serde(
                        default = "query_parameters_defaults :: include_items_from_all_drives"
                    )]
                    #[doc = "Whether both My Drive and shared drive items should be included in results."]
                    pub include_items_from_all_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_subscribed () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeSubscribed")]
                    #[serde(default = "query_parameters_defaults :: include_subscribed")]
                    #[doc = "Whether to include changes outside the My Drive hierarchy in the result. When set to false, changes to files such as those in the Application Data folder or shared files which have not been added to My Drive are omitted from the result."]
                    pub include_subscribed: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_team_drive_items () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeTeamDriveItems")]
                    #[serde(default = "query_parameters_defaults :: include_team_drive_items")]
                    #[doc = "Deprecated use includeItemsFromAllDrives instead."]
                    pub include_team_drive_items: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of changes to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "spaces")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of spaces to query. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startChangeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated - use pageToken instead."]
                    pub start_change_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teamDriveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated use driveId instead."]
                    pub team_drive_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_corpus_removals() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_items_from_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_subscribed() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_team_drive_items() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
    pub mod children {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root."]
                    pub enforce_single_parent: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[doc = "Maximum number of children to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of sort keys. Valid keys are 'createdDate', 'folder', 'lastViewedByMeDate', 'modifiedByMeDate', 'modifiedDate', 'quotaBytesUsed', 'recency', 'sharedWithMeDate', 'starred', and 'title'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedDate desc,title. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token for children."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for searching children."]
                    pub q: ::std::option::Option<::std::string::String>,
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
            }
        }
    }
    pub mod comments {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "If set, this will succeed when retrieving a deleted comment, and will include any deleted replies."]
                    pub include_deleted: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "If set, all comments and replies, including deleted comments and replies (with content stripped) will be returned."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of discussions to include in the response, used for paging."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The continuation token, used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updatedMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Only discussions that were updated after this timestamp will be returned. Formatted as an RFC 3339 timestamp."]
                    pub updated_min: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                }
            }
        }
    }
    pub mod drives {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "requestId")]
                    #[doc = "An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned."]
                    pub request_id: ::std::string::String,
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
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of shared drives to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token for shared drives."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for searching shared drives."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"10").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
    pub mod files {
        pub mod methods {
            pub mod copy {
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
                        default = "{ query_parameters_defaults :: convert () }",
                        setter(into)
                    )]
                    #[serde(rename = "convert")]
                    #[serde(default = "query_parameters_defaults :: convert")]
                    #[doc = "Whether to convert this file to the corresponding Docs Editors format."]
                    pub convert: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Copying files into multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ query_parameters_defaults :: ocr () }", setter(into))]
                    #[serde(rename = "ocr")]
                    #[serde(default = "query_parameters_defaults :: ocr")]
                    #[doc = "Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads."]
                    pub ocr: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If ocr is true, hints at the language to use. Valid values are BCP 47 codes."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: pinned () }",
                        setter(into)
                    )]
                    #[serde(rename = "pinned")]
                    #[serde(default = "query_parameters_defaults :: pinned")]
                    #[doc = "Whether to pin the head revision of the new copy. A file can have a maximum of 200 pinned revisions."]
                    pub pinned: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language of the timed text."]
                    pub timed_text_language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextTrackName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The timed text track name."]
                    pub timed_text_track_name: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the new file. This parameter is only relevant when the source is not a native Google Doc and convert=false."]
                    pub visibility: QueryParametersVisibilityEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn convert() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn ocr() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn pinned() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn visibility() -> super::QueryParametersVisibilityEnum {
                        serde_json::from_str(&"DEFAULT").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The visibility of the new file. This parameter is only relevant when the source is not a native Google Doc and convert=false."]
                pub enum QueryParametersVisibilityEnum {
                    #[serde(rename = "DEFAULT")]
                    #[doc = "The visibility of the new file is determined by the user's default visibility/sharing policies."]
                    Default,
                    #[serde(rename = "PRIVATE")]
                    #[doc = "The new file will be visible to only the owner."]
                    Private,
                }
                impl ::std::default::Default for QueryParametersVisibilityEnum {
                    fn default() -> Self {
                        Self::Default
                    }
                }
            }
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
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
            pub mod empty_trash {
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
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root."]
                    pub enforce_single_parent: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
            pub mod export {
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
                    #[serde(rename = "mimeType")]
                    #[doc = "The MIME type of the format requested for this export."]
                    pub mime_type: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod generate_ids {
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
                    #[doc = "Maximum number of IDs to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ query_parameters_defaults :: space () }", setter(into))]
                    #[serde(rename = "space")]
                    #[serde(default = "query_parameters_defaults :: space")]
                    #[doc = "The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'."]
                    pub space: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"10").unwrap()
                    }
                    pub fn space() -> ::std::string::String {
                        String::from("drive")
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
                    #[builder(
                        default = "{ query_parameters_defaults :: acknowledge_abuse () }",
                        setter(into)
                    )]
                    #[serde(rename = "acknowledgeAbuse")]
                    #[serde(default = "query_parameters_defaults :: acknowledge_abuse")]
                    #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files."]
                    pub acknowledge_abuse: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter is deprecated and has no function."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revisionId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the Revision ID that should be downloaded. Ignored unless alt=media is specified."]
                    pub revision_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: update_viewed_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "updateViewedDate")]
                    #[serde(default = "query_parameters_defaults :: update_viewed_date")]
                    #[doc = "Deprecated: Use files.update with modifiedDateBehavior=noChange, updateViewedDate=true and an empty request body."]
                    pub update_viewed_date: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn acknowledge_abuse() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn update_viewed_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "This parameter is deprecated and has no function."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Deprecated"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Deprecated"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: convert () }",
                        setter(into)
                    )]
                    #[serde(rename = "convert")]
                    #[serde(default = "query_parameters_defaults :: convert")]
                    #[doc = "Whether to convert this file to the corresponding Docs Editors format."]
                    pub convert: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Creating files in multiple folders is no longer supported."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ query_parameters_defaults :: ocr () }", setter(into))]
                    #[serde(rename = "ocr")]
                    #[serde(default = "query_parameters_defaults :: ocr")]
                    #[doc = "Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads."]
                    pub ocr: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If ocr is true, hints at the language to use. Valid values are BCP 47 codes."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: pinned () }",
                        setter(into)
                    )]
                    #[serde(rename = "pinned")]
                    #[serde(default = "query_parameters_defaults :: pinned")]
                    #[doc = "Whether to pin the head revision of the uploaded file. A file can have a maximum of 200 pinned revisions."]
                    pub pinned: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language of the timed text."]
                    pub timed_text_language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextTrackName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The timed text track name."]
                    pub timed_text_track_name: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_content_as_indexable_text () }",
                        setter(into)
                    )]
                    #[serde(rename = "useContentAsIndexableText")]
                    #[serde(
                        default = "query_parameters_defaults :: use_content_as_indexable_text"
                    )]
                    #[doc = "Whether to use the content as indexable text."]
                    pub use_content_as_indexable_text: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the new file. This parameter is only relevant when convert=false."]
                    pub visibility: QueryParametersVisibilityEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn convert() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn ocr() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn pinned() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_content_as_indexable_text() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn visibility() -> super::QueryParametersVisibilityEnum {
                        serde_json::from_str(&"DEFAULT").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The visibility of the new file. This parameter is only relevant when convert=false."]
                pub enum QueryParametersVisibilityEnum {
                    #[serde(rename = "DEFAULT")]
                    #[doc = "The visibility of the new file is determined by the user's default visibility/sharing policies."]
                    Default,
                    #[serde(rename = "PRIVATE")]
                    #[doc = "The new file will be visible to only the owner."]
                    Private,
                }
                impl ::std::default::Default for QueryParametersVisibilityEnum {
                    fn default() -> Self {
                        Self::Default
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
                    #[serde(rename = "corpora")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency."]
                    pub corpora: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "corpus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The body of items (files/documents) to which the query applies. Deprecated: use 'corpora' instead."]
                    pub corpus: ::std::option::Option<QueryParametersCorpusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "driveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the shared drive to search."]
                    pub drive_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_items_from_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeItemsFromAllDrives")]
                    #[serde(
                        default = "query_parameters_defaults :: include_items_from_all_drives"
                    )]
                    #[doc = "Whether both My Drive and shared drive items should be included in results."]
                    pub include_items_from_all_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_team_drive_items () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeTeamDriveItems")]
                    #[serde(default = "query_parameters_defaults :: include_team_drive_items")]
                    #[doc = "Deprecated use includeItemsFromAllDrives instead."]
                    pub include_team_drive_items: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of sort keys. Valid keys are 'createdDate', 'folder', 'lastViewedByMeDate', 'modifiedByMeDate', 'modifiedDate', 'quotaBytesUsed', 'recency', 'sharedWithMeDate', 'starred', 'title', and 'title_natural'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedDate desc,title. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token for files."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter is deprecated and has no function."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for searching files."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "spaces")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of spaces to query. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "teamDriveId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated use driveId instead."]
                    pub team_drive_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_items_from_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_team_drive_items() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The body of items (files/documents) to which the query applies. Deprecated: use 'corpora' instead."]
                pub enum QueryParametersCorpusEnum {
                    #[serde(rename = "DEFAULT")]
                    #[doc = "The items that the user has accessed."]
                    Default,
                    #[serde(rename = "DOMAIN")]
                    #[doc = "Items shared to the user's domain."]
                    Domain,
                }
                impl ::std::default::Default for QueryParametersCorpusEnum {
                    fn default() -> Self {
                        Self::Default
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "This parameter is deprecated and has no function."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Deprecated"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Deprecated"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
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
                    #[serde(rename = "addParents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma-separated list of parent IDs to add."]
                    pub add_parents: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: convert () }",
                        setter(into)
                    )]
                    #[serde(rename = "convert")]
                    #[serde(default = "query_parameters_defaults :: convert")]
                    #[doc = "This parameter is deprecated and has no function."]
                    pub convert: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "modifiedDateBehavior")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Determines the behavior in which modifiedDate is updated. This overrides setModifiedDate."]
                    pub modified_date_behavior:
                        ::std::option::Option<QueryParametersModifiedDateBehaviorEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: new_revision () }",
                        setter(into)
                    )]
                    #[serde(rename = "newRevision")]
                    #[serde(default = "query_parameters_defaults :: new_revision")]
                    #[doc = "Whether a blob upload should create a new revision. If false, the blob data in the current head revision is replaced. If true or not set, a new blob is created as head revision, and previous unpinned revisions are preserved for a short period of time. Pinned revisions are stored indefinitely, using additional storage quota, up to a maximum of 200 revisions. For details on how revisions are retained, see the Drive Help Center. Note that this field is ignored if there is no payload in the request."]
                    pub new_revision: ::std::primitive::bool,
                    #[builder(default = "{ query_parameters_defaults :: ocr () }", setter(into))]
                    #[serde(rename = "ocr")]
                    #[serde(default = "query_parameters_defaults :: ocr")]
                    #[doc = "Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads."]
                    pub ocr: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If ocr is true, hints at the language to use. Valid values are BCP 47 codes."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: pinned () }",
                        setter(into)
                    )]
                    #[serde(rename = "pinned")]
                    #[serde(default = "query_parameters_defaults :: pinned")]
                    #[doc = "Whether to pin the new revision. A file can have a maximum of 200 pinned revisions. Note that this field is ignored if there is no payload in the request."]
                    pub pinned: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "removeParents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma-separated list of parent IDs to remove."]
                    pub remove_parents: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: set_modified_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "setModifiedDate")]
                    #[serde(default = "query_parameters_defaults :: set_modified_date")]
                    #[doc = "Whether to set the modified date using the value supplied in the request body. Setting this field to true is equivalent to modifiedDateBehavior=fromBodyOrNow, and false is equivalent to modifiedDateBehavior=now. To prevent any changes to the modified date set modifiedDateBehavior=noChange."]
                    pub set_modified_date: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language of the timed text."]
                    pub timed_text_language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextTrackName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The timed text track name."]
                    pub timed_text_track_name: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: update_viewed_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "updateViewedDate")]
                    #[serde(default = "query_parameters_defaults :: update_viewed_date")]
                    #[doc = "Whether to update the view date after successfully updating the file."]
                    pub update_viewed_date: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_content_as_indexable_text () }",
                        setter(into)
                    )]
                    #[serde(rename = "useContentAsIndexableText")]
                    #[serde(
                        default = "query_parameters_defaults :: use_content_as_indexable_text"
                    )]
                    #[doc = "Whether to use the content as indexable text."]
                    pub use_content_as_indexable_text: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn convert() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn new_revision() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn ocr() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn pinned() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn set_modified_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn update_viewed_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn use_content_as_indexable_text() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Determines the behavior in which modifiedDate is updated. This overrides setModifiedDate."]
                pub enum QueryParametersModifiedDateBehaviorEnum {
                    #[serde(rename = "fromBody")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request. No change if no value was provided."]
                    FromBody,
                    #[serde(rename = "fromBodyIfNeeded")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request depending on other contents of the update."]
                    FromBodyIfNeeded,
                    #[serde(rename = "fromBodyOrNow")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request, or to the current time if no value was provided."]
                    FromBodyOrNow,
                    #[serde(rename = "noChange")]
                    #[doc = "Maintain the previous value of modifiedDate."]
                    NoChange,
                    #[serde(rename = "now")]
                    #[doc = "Set modifiedDate to the current time."]
                    Now,
                    #[serde(rename = "nowIfNeeded")]
                    #[doc = "Set modifiedDate to the current time depending on contents of the update."]
                    NowIfNeeded,
                }
                impl ::std::default::Default for QueryParametersModifiedDateBehaviorEnum {
                    fn default() -> Self {
                        Self::FromBody
                    }
                }
            }
            pub mod touch {
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
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
            pub mod trash {
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
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
            pub mod untrash {
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
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[serde(rename = "addParents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma-separated list of parent IDs to add."]
                    pub add_parents: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: convert () }",
                        setter(into)
                    )]
                    #[serde(rename = "convert")]
                    #[serde(default = "query_parameters_defaults :: convert")]
                    #[doc = "This parameter is deprecated and has no function."]
                    pub convert: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "modifiedDateBehavior")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Determines the behavior in which modifiedDate is updated. This overrides setModifiedDate."]
                    pub modified_date_behavior:
                        ::std::option::Option<QueryParametersModifiedDateBehaviorEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: new_revision () }",
                        setter(into)
                    )]
                    #[serde(rename = "newRevision")]
                    #[serde(default = "query_parameters_defaults :: new_revision")]
                    #[doc = "Whether a blob upload should create a new revision. If false, the blob data in the current head revision is replaced. If true or not set, a new blob is created as head revision, and previous unpinned revisions are preserved for a short period of time. Pinned revisions are stored indefinitely, using additional storage quota, up to a maximum of 200 revisions. For details on how revisions are retained, see the Drive Help Center. Note that this field is ignored if there is no payload in the request."]
                    pub new_revision: ::std::primitive::bool,
                    #[builder(default = "{ query_parameters_defaults :: ocr () }", setter(into))]
                    #[serde(rename = "ocr")]
                    #[serde(default = "query_parameters_defaults :: ocr")]
                    #[doc = "Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads."]
                    pub ocr: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If ocr is true, hints at the language to use. Valid values are BCP 47 codes."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: pinned () }",
                        setter(into)
                    )]
                    #[serde(rename = "pinned")]
                    #[serde(default = "query_parameters_defaults :: pinned")]
                    #[doc = "Whether to pin the new revision. A file can have a maximum of 200 pinned revisions. Note that this field is ignored if there is no payload in the request."]
                    pub pinned: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "removeParents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma-separated list of parent IDs to remove."]
                    pub remove_parents: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: set_modified_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "setModifiedDate")]
                    #[serde(default = "query_parameters_defaults :: set_modified_date")]
                    #[doc = "Whether to set the modified date using the value supplied in the request body. Setting this field to true is equivalent to modifiedDateBehavior=fromBodyOrNow, and false is equivalent to modifiedDateBehavior=now. To prevent any changes to the modified date set modifiedDateBehavior=noChange."]
                    pub set_modified_date: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language of the timed text."]
                    pub timed_text_language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timedTextTrackName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The timed text track name."]
                    pub timed_text_track_name: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: update_viewed_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "updateViewedDate")]
                    #[serde(default = "query_parameters_defaults :: update_viewed_date")]
                    #[doc = "Whether to update the view date after successfully updating the file."]
                    pub update_viewed_date: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_content_as_indexable_text () }",
                        setter(into)
                    )]
                    #[serde(rename = "useContentAsIndexableText")]
                    #[serde(
                        default = "query_parameters_defaults :: use_content_as_indexable_text"
                    )]
                    #[doc = "Whether to use the content as indexable text."]
                    pub use_content_as_indexable_text: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn convert() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn new_revision() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn ocr() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn pinned() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn set_modified_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn update_viewed_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn use_content_as_indexable_text() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Determines the behavior in which modifiedDate is updated. This overrides setModifiedDate."]
                pub enum QueryParametersModifiedDateBehaviorEnum {
                    #[serde(rename = "fromBody")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request. No change if no value was provided."]
                    FromBody,
                    #[serde(rename = "fromBodyIfNeeded")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request depending on other contents of the update."]
                    FromBodyIfNeeded,
                    #[serde(rename = "fromBodyOrNow")]
                    #[doc = "Set modifiedDate to the value provided in the body of the request, or to the current time if no value was provided."]
                    FromBodyOrNow,
                    #[serde(rename = "noChange")]
                    #[doc = "Maintain the previous value of modifiedDate."]
                    NoChange,
                    #[serde(rename = "now")]
                    #[doc = "Set modifiedDate to the current time."]
                    Now,
                    #[serde(rename = "nowIfNeeded")]
                    #[doc = "Set modifiedDate to the current time depending on contents of the update."]
                    NowIfNeeded,
                }
                impl ::std::default::Default for QueryParametersModifiedDateBehaviorEnum {
                    fn default() -> Self {
                        Self::FromBody
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
                    #[builder(
                        default = "{ query_parameters_defaults :: acknowledge_abuse () }",
                        setter(into)
                    )]
                    #[serde(rename = "acknowledgeAbuse")]
                    #[serde(default = "query_parameters_defaults :: acknowledge_abuse")]
                    #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files."]
                    pub acknowledge_abuse: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter is deprecated and has no function."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revisionId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the Revision ID that should be downloaded. Ignored unless alt=media is specified."]
                    pub revision_id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: update_viewed_date () }",
                        setter(into)
                    )]
                    #[serde(rename = "updateViewedDate")]
                    #[serde(default = "query_parameters_defaults :: update_viewed_date")]
                    #[doc = "Deprecated: Use files.update with modifiedDateBehavior=noChange, updateViewedDate=true and an empty request body."]
                    pub update_viewed_date: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn acknowledge_abuse() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn update_viewed_date() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "This parameter is deprecated and has no function."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Deprecated"]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Deprecated"]
                    Full,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
            }
        }
    }
    pub mod parents {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root."]
                    pub enforce_single_parent: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
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
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
    pub mod permissions {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "emailMessage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A plain text custom message to include in notification emails."]
                    pub email_message: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. See moveToNewOwnersRoot for details."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: move_to_new_owners_root () }",
                        setter(into)
                    )]
                    #[serde(rename = "moveToNewOwnersRoot")]
                    #[serde(default = "query_parameters_defaults :: move_to_new_owners_root")]
                    #[doc = "This parameter will only take effect if the item is not in a shared drive and the request is attempting to transfer the ownership of the item. If set to true, the item will be moved to the new owner's My Drive root folder and all prior parents removed. If set to false, parents are not changed."]
                    pub move_to_new_owners_root: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: send_notification_emails () }",
                        setter(into)
                    )]
                    #[serde(rename = "sendNotificationEmails")]
                    #[serde(default = "query_parameters_defaults :: send_notification_emails")]
                    #[doc = "Whether to send notification emails when sharing to users or groups. This parameter is ignored and an email is sent if the role is owner."]
                    pub send_notification_emails: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn move_to_new_owners_root() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn send_notification_emails() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: remove_expiration () }",
                        setter(into)
                    )]
                    #[serde(rename = "removeExpiration")]
                    #[serde(default = "query_parameters_defaults :: remove_expiration")]
                    #[doc = "Whether to remove the expiration date."]
                    pub remove_expiration: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: transfer_ownership () }",
                        setter(into)
                    )]
                    #[serde(rename = "transferOwnership")]
                    #[serde(default = "query_parameters_defaults :: transfer_ownership")]
                    #[doc = "Whether changing a role to 'owner' downgrades the current owners to writers. Does nothing if the specified role is not 'owner'."]
                    pub transfer_ownership: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn remove_expiration() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn transfer_ownership() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: remove_expiration () }",
                        setter(into)
                    )]
                    #[serde(rename = "removeExpiration")]
                    #[serde(default = "query_parameters_defaults :: remove_expiration")]
                    #[doc = "Whether to remove the expiration date."]
                    pub remove_expiration: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_all_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsAllDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_all_drives")]
                    #[doc = "Whether the requesting application supports both My Drives and shared drives."]
                    pub supports_all_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: supports_team_drives () }",
                        setter(into)
                    )]
                    #[serde(rename = "supportsTeamDrives")]
                    #[serde(default = "query_parameters_defaults :: supports_team_drives")]
                    #[doc = "Deprecated use supportsAllDrives instead."]
                    pub supports_team_drives: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: transfer_ownership () }",
                        setter(into)
                    )]
                    #[serde(rename = "transferOwnership")]
                    #[serde(default = "query_parameters_defaults :: transfer_ownership")]
                    #[doc = "Whether changing a role to 'owner' downgrades the current owners to writers. Does nothing if the specified role is not 'owner'."]
                    pub transfer_ownership: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn remove_expiration() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn supports_team_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn transfer_ownership() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
    pub mod properties {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the property."]
                    pub visibility: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn visibility() -> ::std::string::String {
                        String::from("private")
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
                    #[builder(
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the property."]
                    pub visibility: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn visibility() -> ::std::string::String {
                        String::from("private")
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
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the property. Allowed values are PRIVATE and PUBLIC. (Default: PRIVATE)"]
                    pub visibility: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn visibility() -> ::std::string::String {
                        String::from("private")
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
                        default = "{ query_parameters_defaults :: visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "visibility")]
                    #[serde(default = "query_parameters_defaults :: visibility")]
                    #[doc = "The visibility of the property. Allowed values are PRIVATE and PUBLIC. (Default: PRIVATE)"]
                    pub visibility: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn visibility() -> ::std::string::String {
                        String::from("private")
                    }
                }
            }
        }
    }
    pub mod replies {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "If set, this will succeed when retrieving a deleted reply."]
                    pub include_deleted: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: include_deleted () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeDeleted")]
                    #[serde(default = "query_parameters_defaults :: include_deleted")]
                    #[doc = "If set, all replies, including deleted replies (with content stripped) will be returned."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of replies to include in the response, used for paging."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The continuation token, used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn include_deleted() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                }
            }
        }
    }
    pub mod revisions {
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
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of revisions to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token for revisions. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
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
    pub mod teamdrives {
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
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "requestId")]
                    #[doc = "An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned."]
                    pub request_id: ::std::string::String,
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
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of Team Drives to return."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token for Team Drives."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for searching Team Drives."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"10").unwrap()
                    }
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                        default = "{ query_parameters_defaults :: use_domain_admin_access () }",
                        setter(into)
                    )]
                    #[serde(rename = "useDomainAdminAccess")]
                    #[serde(default = "query_parameters_defaults :: use_domain_admin_access")]
                    #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs."]
                    pub use_domain_admin_access: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn use_domain_admin_access() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
    #[doc = "An item with user information and settings."]
    pub struct About {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalRoleInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about supported additional roles per file type. The most specific type takes precedence."]
        pub additional_role_info: ::std::option::Option<::std::vec::Vec<AboutAdditionalRoleInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canCreateDrives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user can create shared drives."]
        pub can_create_drives: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canCreateTeamDrives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canCreateDrives instead."]
        pub can_create_team_drives: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainSharingPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain sharing policy for the current user. Possible values are:  \n- allowed \n- allowedWithWarning \n- incomingOnly \n- disallowed"]
        pub domain_sharing_policy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveThemes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of themes that are supported for shared drives."]
        pub drive_themes: ::std::option::Option<::std::vec::Vec<AboutDriveThemes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the item."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The allowable export formats."]
        pub export_formats: ::std::option::Option<::std::vec::Vec<AboutExportFormats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of additional features enabled on this account."]
        pub features: ::std::option::Option<::std::vec::Vec<AboutFeatures>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderColorPalette")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The palette of allowable folder colors as RGB hex strings."]
        pub folder_color_palette: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The allowable import formats."]
        pub import_formats: ::std::option::Option<::std::vec::Vec<AboutImportFormats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isCurrentAppInstalled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean indicating whether the authenticated app is installed by the authenticated user."]
        pub is_current_app_installed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ about_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "about_defaults :: kind")]
        #[doc = "This is always drive#about."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/)."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largestChangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The largest change id."]
        pub largest_change_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxUploadSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of max upload sizes for each file type. The most specific type takes precedence."]
        pub max_upload_sizes: ::std::option::Option<::std::vec::Vec<AboutMaxUploadSizes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the current user."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current user's ID as visible in the permissions collection."]
        pub permission_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesByService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of storage quota used by different Google services."]
        pub quota_bytes_by_service:
            ::std::option::Option<::std::vec::Vec<AboutQuotaBytesByService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of quota bytes. This is only relevant when quotaType is LIMITED."]
        pub quota_bytes_total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of quota bytes used by Google Drive."]
        pub quota_bytes_used: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesUsedAggregate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of quota bytes used by all Google apps (Drive, Picasa, etc.)."]
        pub quota_bytes_used_aggregate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesUsedInTrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of quota bytes used by trashed items."]
        pub quota_bytes_used_in_trash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the user's storage quota. Possible values are:  \n- LIMITED \n- UNLIMITED"]
        pub quota_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingChangeIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of remaining change ids, limited to no more than 2500."]
        pub remaining_change_ids: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootFolderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the root folder."]
        pub root_folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this item."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveThemes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveThemes instead."]
        pub team_drive_themes: ::std::option::Option<::std::vec::Vec<AboutTeamDriveThemes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authenticated user."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl About {
        pub fn builder() -> AboutBuilder {
            AboutBuilder::default()
        }
    }
    mod about_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#about")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutAdditionalRoleInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roleSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The supported additional roles per primary role."]
        pub role_sets: ::std::option::Option<::std::vec::Vec<AboutAdditionalRoleInfoRoleSets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content type that this additional role info applies to."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AboutAdditionalRoleInfo {
        pub fn builder() -> AboutAdditionalRoleInfoBuilder {
            AboutAdditionalRoleInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutAdditionalRoleInfoRoleSets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The supported additional roles with the primary role."]
        pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A primary permission role."]
        pub primary_role: ::std::option::Option<::std::string::String>,
    }
    impl AboutAdditionalRoleInfoRoleSets {
        pub fn builder() -> AboutAdditionalRoleInfoRoleSetsBuilder {
            AboutAdditionalRoleInfoRoleSetsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutDriveThemes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to this theme's background image."]
        pub background_image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of this theme as an RGB hex string."]
        pub color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the theme."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl AboutDriveThemes {
        pub fn builder() -> AboutDriveThemesBuilder {
            AboutDriveThemesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutExportFormats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content type to convert from."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The possible content types to convert to."]
        pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AboutExportFormats {
        pub fn builder() -> AboutExportFormatsBuilder {
            AboutExportFormatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutFeatures {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the feature."]
        pub feature_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request limit rate for this feature, in queries per second."]
        pub feature_rate: ::std::option::Option<::std::primitive::f64>,
    }
    impl AboutFeatures {
        pub fn builder() -> AboutFeaturesBuilder {
            AboutFeaturesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutImportFormats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The imported file's content type to convert from."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The possible content types to convert to."]
        pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AboutImportFormats {
        pub fn builder() -> AboutImportFormatsBuilder {
            AboutImportFormatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutMaxUploadSizes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max upload size for this type."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file type."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AboutMaxUploadSizes {
        pub fn builder() -> AboutMaxUploadSizesBuilder {
            AboutMaxUploadSizesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutQuotaBytesByService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The storage quota bytes used by the service."]
        pub bytes_used: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's name, e.g. DRIVE, GMAIL, or PHOTOS."]
        pub service_name: ::std::option::Option<::std::string::String>,
    }
    impl AboutQuotaBytesByService {
        pub fn builder() -> AboutQuotaBytesByServiceBuilder {
            AboutQuotaBytesByServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AboutTeamDriveThemes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveThemes/backgroundImageLink instead."]
        pub background_image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveThemes/colorRgb instead."]
        pub color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveThemes/id instead."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl AboutTeamDriveThemes {
        pub fn builder() -> AboutTeamDriveThemesBuilder {
            AboutTeamDriveThemesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The apps resource provides a list of the apps that a user has installed, with information about each app's supported MIME types, file extensions, and other details."]
    pub struct App {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app is authorized to access data on the user's Drive."]
        pub authorized: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createInFolderTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The template url to create a new file with this app in a given folder. The template will contain {folderId} to be replaced by the folder to create the new file in."]
        pub create_in_folder_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to create a new file with this app."]
        pub create_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasDriveWideScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app has drive-wide scope. An app with drive-wide scope can access all files in the user's drive."]
        pub has_drive_wide_scope: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The various icons for the app."]
        pub icons: ::std::option::Option<::std::vec::Vec<AppIcons>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the app."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app is installed."]
        pub installed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ app_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "app_defaults :: kind")]
        #[doc = "This is always drive#app."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A long description of the app."]
        pub long_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the app."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of object this app creates (e.g. Chart). If empty, the app name should be used instead."]
        pub object_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openUrlTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The template url for opening files with this app. The template will contain {ids} and/or {exportIds} to be replaced by the actual file ids. See  Open Files  for the full documentation."]
        pub open_url_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryFileExtensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of primary file extensions."]
        pub primary_file_extensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryMimeTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of primary mime types."]
        pub primary_mime_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product listing for this app."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the product listing for this app."]
        pub product_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryFileExtensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of secondary file extensions."]
        pub secondary_file_extensions:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryMimeTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of secondary mime types."]
        pub secondary_mime_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of the app."]
        pub short_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsCreate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this app supports creating new objects."]
        pub supports_create: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsImport")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this app supports importing from Docs Editors."]
        pub supports_import: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsMultiOpen")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this app supports opening more than one file."]
        pub supports_multi_open: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsOfflineCreate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this app supports creating new files when offline."]
        pub supports_offline_create: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useByDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app is selected as the default handler for the types it supports."]
        pub use_by_default: ::std::option::Option<::std::primitive::bool>,
    }
    impl App {
        pub fn builder() -> AppBuilder {
            AppBuilder::default()
        }
    }
    mod app_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#app")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AppIcons {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category of the icon. Allowed values are:  \n- application - icon for the application \n- document - icon for a file associated with the app \n- documentShared - icon for a shared file associated with the app"]
        pub category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for the icon."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the icon. Represented as the maximum of the width and height."]
        pub size: ::std::option::Option<::std::primitive::i64>,
    }
    impl AppIcons {
        pub fn builder() -> AppIconsBuilder {
            AppIconsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of third-party applications which the user has installed or given access to Google Drive."]
    pub struct AppList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultAppIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of app IDs that the user has specified to use by default. The list is in reverse-priority order (lowest to highest)."]
        pub default_app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of apps."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<App>>>,
        #[builder(default = "{ app_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "app_list_defaults :: kind")]
        #[doc = "This is always drive#appList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl AppList {
        pub fn builder() -> AppListBuilder {
            AppListBuilder::default()
        }
    }
    mod app_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#appList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a change to a file or shared drive."]
    pub struct Change {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the change. Possible values are file and drive."]
        pub change_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted."]
        pub drive: ::std::option::Option<::std::boxed::Box<Drive>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the shared drive associated with this change."]
        pub drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "file")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated state of the file. Present if the type is file and the file has not been removed from this list of changes."]
        pub file: ::std::option::Option<::std::boxed::Box<File>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file associated with this change."]
        pub file_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the change."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ change_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_defaults :: kind")]
        #[doc = "This is always drive#change."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modificationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time of this modification."]
        pub modification_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this change."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use drive instead."]
        pub team_drive: ::std::option::Option<::std::boxed::Box<TeamDrive>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveId instead."]
        pub team_drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use changeType instead."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Change {
        pub fn builder() -> ChangeBuilder {
            ChangeBuilder::default()
        }
    }
    mod change_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#change")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of changes for a user."]
    pub struct ChangeList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
        #[builder(default = "{ change_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_list_defaults :: kind")]
        #[doc = "This is always drive#changeList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largestChangeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current largest change ID."]
        pub largest_change_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newStartPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting page token for future changes. This will be present only if the end of the current changes list has been reached."]
        pub new_start_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the next page of changes."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ChangeList {
        pub fn builder() -> ChangeListBuilder {
            ChangeListBuilder::default()
        }
    }
    mod change_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#changeList")
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
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
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
            String::from("api#channel")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of children of a file."]
    pub struct ChildList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of children. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChildReference>>>,
        #[builder(default = "{ child_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "child_list_defaults :: kind")]
        #[doc = "This is always drive#childList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the next page of children."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of children. This will be absent if the end of the children list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ChildList {
        pub fn builder() -> ChildListBuilder {
            ChildListBuilder::default()
        }
    }
    mod child_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#childList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a folder's child."]
    pub struct ChildReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the child."]
        pub child_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the child."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ child_reference_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "child_reference_defaults :: kind")]
        #[doc = "This is always drive#childReference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this reference."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ChildReference {
        pub fn builder() -> ChildReferenceBuilder {
            ChildReferenceBuilder::default()
        }
    }
    mod child_reference_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#childReference")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A comment on a file in Google Drive."]
    pub struct Comment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties."]
        pub anchor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of the comment. The author's email address and permission ID will not be populated."]
        pub author: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the comment."]
        pub comment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The context of the file which is being commented on."]
        pub context: ::std::option::Option<CommentContext>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when this comment was first created."]
        pub created_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file which this comment is addressing."]
        pub file_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the file which this comment is addressing."]
        pub file_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTML formatted content for this comment."]
        pub html_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ comment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_defaults :: kind")]
        #[doc = "This is always drive#comment."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when this comment or any of its replies were last modified."]
        pub modified_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replies to this post."]
        pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CommentReply>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this comment."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of this comment. Status can be changed by posting a reply to a comment with the desired status.  \n- \"open\" - The comment is still open. \n- \"resolved\" - The comment has been resolved by one of its replies."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl Comment {
        pub fn builder() -> CommentBuilder {
            CommentBuilder::default()
        }
    }
    mod comment_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#comment")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The context of the file which is being commented on."]
    pub struct CommentContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the context snippet."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data representation of the segment of the file being commented on. In the case of a text file for example, this would be the actual text that the comment is about."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CommentContext {
        pub fn builder() -> CommentContextBuilder {
            CommentContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of comments on a file in Google Drive."]
    pub struct CommentList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
        #[builder(default = "{ comment_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_list_defaults :: kind")]
        #[doc = "This is always drive#commentList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the next page of comments."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl CommentList {
        pub fn builder() -> CommentListBuilder {
            CommentListBuilder::default()
        }
    }
    mod comment_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#commentList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A comment on a file in Google Drive."]
    pub struct CommentReply {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of the reply. The author's email address and permission ID will not be populated."]
        pub author: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen)."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when this reply was first created."]
        pub created_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTML formatted content for this reply."]
        pub html_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ comment_reply_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_reply_defaults :: kind")]
        #[doc = "This is always drive#commentReply."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when this reply was last modified."]
        pub modified_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the reply."]
        pub reply_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action this reply performed to the parent comment. When creating a new reply this is the action to be perform to the parent comment. Possible values are:  \n- \"resolve\" - To resolve a comment. \n- \"reopen\" - To reopen (un-resolve) a comment."]
        pub verb: ::std::option::Option<::std::string::String>,
    }
    impl CommentReply {
        pub fn builder() -> CommentReplyBuilder {
            CommentReplyBuilder::default()
        }
    }
    mod comment_reply_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#commentReply")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of replies to a comment on a file in Google Drive."]
    pub struct CommentReplyList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CommentReply>>>,
        #[builder(default = "{ comment_reply_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_reply_list_defaults :: kind")]
        #[doc = "This is always drive#commentReplyList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the next page of replies."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl CommentReplyList {
        pub fn builder() -> CommentReplyListBuilder {
            CommentReplyListBuilder::default()
        }
    }
    mod comment_reply_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#commentReplyList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A restriction for accessing the content of the file."]
    pub struct ContentRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified."]
        pub read_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true."]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who set the content restriction. Only populated if readOnly is true."]
        pub restricting_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictionDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true."]
        pub restriction_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the content restriction. Currently the only possible value is globalContentRestriction."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ContentRestriction {
        pub fn builder() -> ContentRestrictionBuilder {
            ContentRestrictionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a shared drive."]
    pub struct Drive {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
        pub background_image_file: ::std::option::Option<DriveBackgroundImageFile>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short-lived link to this shared drive's background image."]
        pub background_image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Capabilities the current user has on this shared drive."]
        pub capabilities: ::std::option::Option<DriveCapabilities>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId."]
        pub color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the shared drive was created (RFC 3339 date-time)."]
        pub created_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hidden")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the shared drive is hidden from default view."]
        pub hidden: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this shared drive which is also the ID of the top level folder of this shared drive."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ drive_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "drive_defaults :: kind")]
        #[doc = "This is always drive#drive"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this shared drive."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of restrictions that apply to this shared drive or items inside this shared drive."]
        pub restrictions: ::std::option::Option<DriveRestrictions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
        pub theme_id: ::std::option::Option<::std::string::String>,
    }
    impl Drive {
        pub fn builder() -> DriveBuilder {
            DriveBuilder::default()
        }
    }
    mod drive_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#drive")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
    pub struct DriveBackgroundImageFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of an image file in Google Drive to use for the background image."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
        pub width: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xCoordinate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
        pub x_coordinate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yCoordinate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
        pub y_coordinate: ::std::option::Option<::std::primitive::f64>,
    }
    impl DriveBackgroundImageFile {
        pub fn builder() -> DriveBackgroundImageFileBuilder {
            DriveBackgroundImageFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Capabilities the current user has on this shared drive."]
    pub struct DriveCapabilities {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canAddChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add children to folders in this shared drive."]
        pub can_add_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive."]
        pub can_change_copy_requires_writer_permission_restriction:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the domainUsersOnly restriction of this shared drive."]
        pub can_change_domain_users_only_restriction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeDriveBackground")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the background of this shared drive."]
        pub can_change_drive_background: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeDriveMembersOnlyRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the driveMembersOnly restriction of this shared drive."]
        pub can_change_drive_members_only_restriction:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can comment on files in this shared drive."]
        pub can_comment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canCopy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can copy files in this shared drive."]
        pub can_copy: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDeleteChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete children from folders in this shared drive."]
        pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDeleteDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive."]
        pub can_delete_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDownload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can download files in this shared drive."]
        pub can_download: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canEdit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can edit files in this shared drive"]
        pub can_edit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canListChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can list the children of folders in this shared drive."]
        pub can_list_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canManageMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add members to this shared drive or remove them or change their role."]
        pub can_manage_members: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can read the revisions resource of files in this shared drive."]
        pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can rename files or folders in this shared drive."]
        pub can_rename: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRenameDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can rename this shared drive."]
        pub can_rename_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canShare")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can share files or folders in this shared drive."]
        pub can_share: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canTrashChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can trash children from folders in this shared drive."]
        pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
    }
    impl DriveCapabilities {
        pub fn builder() -> DriveCapabilitiesBuilder {
            DriveCapabilitiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of restrictions that apply to this shared drive or items inside this shared drive."]
    pub struct DriveRestrictions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adminManagedRestrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether administrative privileges on this shared drive are required to modify restrictions."]
        pub admin_managed_restrictions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyRequiresWriterPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive."]
        pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainUsersOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive."]
        pub domain_users_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveMembersOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether access to items inside this shared drive is restricted to its members."]
        pub drive_members_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl DriveRestrictions {
        pub fn builder() -> DriveRestrictionsBuilder {
            DriveRestrictionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of shared drives."]
    pub struct DriveList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Drive>>>,
        #[builder(default = "{ drive_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "drive_list_defaults :: kind")]
        #[doc = "This is always drive#driveList"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl DriveList {
        pub fn builder() -> DriveListBuilder {
            DriveListBuilder::default()
        }
    }
    mod drive_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#driveList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for a file."]
    pub struct File {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link for opening the file in a relevant Google editor or viewer."]
        pub alternate_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appDataContents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this file is in the Application Data folder."]
        pub app_data_contents: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: use capabilities/canComment."]
        pub can_comment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: use capabilities/canReadRevisions."]
        pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
        pub capabilities: ::std::option::Option<FileCapabilities>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentRestrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restrictions for accessing the content of the file. Only populated if such a restriction exists."]
        pub content_restrictions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContentRestriction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyRequiresWriterPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the options to copy, print, or download this file, should be disabled for readers and commenters."]
        pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: use capabilities/canCopy."]
        pub copyable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Create time for this file (formatted RFC 3339 timestamp)."]
        pub created_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultOpenWithLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used."]
        pub default_open_with_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of the file."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the shared drive the file resides in. Only populated for items in shared drives."]
        pub drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: use capabilities/canEdit."]
        pub editable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link for embedding the file."]
        pub embed_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the file."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitlyTrashed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this file has been explicitly trashed, as opposed to recursively trashed."]
        pub explicitly_trashed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links for exporting Docs Editors files to specific formats."]
        pub export_links:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileExtension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The final component of fullFileExtension with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
        pub file_extension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the file in bytes. This field is populated for files with content stored in Google Drive and for files in Docs Editors; it is not populated for shortcut files."]
        pub file_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderColorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Folder color as an RGB hex string if the file is a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. Not populated for items in shared drives."]
        pub folder_color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullFileExtension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full file extension; extracted from the title. May contain multiple concatenated extensions, such as \"tar.gz\". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
        pub full_file_extension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasAugmentedPermissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether there are permissions directly on this file. This field is only populated for items in shared drives."]
        pub has_augmented_permissions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasThumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field."]
        pub has_thumbnail: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headRevisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
        pub head_revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the file's icon."]
        pub icon_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageMediaMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content."]
        pub image_media_metadata: ::std::option::Option<FileImageMediaMetadata>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexableText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indexable text attributes for the file (can only be written)"]
        pub indexable_text: ::std::option::Option<FileIndexableText>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAppAuthorized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file was created or opened by the requesting app."]
        pub is_app_authorized: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ file_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "file_defaults :: kind")]
        #[doc = "The type of file. This is always drive#file."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A group of labels for the file."]
        pub labels: ::std::option::Option<FileLabels>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last user to modify this file."]
        pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the last user to modify this file."]
        pub last_modifying_user_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastViewedByMeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this file was viewed by the user (formatted RFC 3339 timestamp)."]
        pub last_viewed_by_me_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "markedViewedByMeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub marked_viewed_by_me_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files."]
        pub md5_checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedByMeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date."]
        pub modified_by_me_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set."]
        pub modified_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openWithLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used."]
        pub open_with_links:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalFilename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original filename of the uploaded content if available, or else the original value of the title field. This is only available for files with binary content in Google Drive."]
        pub original_filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownedByMe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file is owned by the current user. Not populated for items in shared drives."]
        pub owned_by_me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name(s) of the owner(s) of this file. Not populated for items in shared drives."]
        pub owner_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The owner(s) of this file. Not populated for items in shared drives."]
        pub owners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of parent folders which contain this file.\nIf not specified as part of an insert request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests can also use the addParents and removeParents parameters to modify the parents list."]
        pub parents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParentReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of permission IDs for users with access to this file."]
        pub permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of permissions for users with access to this file. Not populated for items in shared drives."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of properties."]
        pub properties: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Property>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of quota bytes used by this file."]
        pub quota_bytes_used: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this file."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shareable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: use capabilities/canShare."]
        pub shareable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shared")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been shared. Not populated for items in shared drives."]
        pub shared: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedWithMeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which this file was shared with the user (formatted RFC 3339 timestamp)."]
        pub shared_with_me_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User that shared the item with the current user, if available."]
        pub sharing_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortcutDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
        pub shortcut_details: ::std::option::Option<FileShortcutDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spaces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of spaces which contain the file. Supported values are 'drive', 'appDataFolder' and 'photos'."]
        pub spaces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveId instead."]
        pub team_drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated."]
        pub thumbnail: ::std::option::Option<FileThumbnail>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short-lived link to the file's thumbnail. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request."]
        pub thumbnail_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail version for use in thumbnail cache invalidation."]
        pub thumbnail_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives."]
        pub trashed_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives."]
        pub trashing_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permissions for the authenticated user on this file."]
        pub user_permission: ::std::option::Option<::std::boxed::Box<Permission>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user."]
        pub version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoMediaMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about video media. This will only be present for video types."]
        pub video_media_metadata: ::std::option::Option<FileVideoMediaMetadata>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webContentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials."]
        pub web_content_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webViewLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting."]
        pub web_view_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writersCanShare")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether writers can share the document with other users. Not populated for items in shared drives."]
        pub writers_can_share: ::std::option::Option<::std::primitive::bool>,
    }
    impl File {
        pub fn builder() -> FileBuilder {
            FileBuilder::default()
        }
    }
    mod file_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#file")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
    pub struct FileCapabilities {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canAddChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add children to this folder. This is always false when the item is not a folder."]
        pub can_add_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canAddFolderFromAnotherDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add a folder from another drive (different shared drive or My Drive) to this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
        pub can_add_folder_from_another_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canAddMyDriveParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add a parent for the item without removing an existing parent in the same request. Not populated for shared drive files."]
        pub can_add_my_drive_parent: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeCopyRequiresWriterPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this file."]
        pub can_change_copy_requires_writer_permission:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeRestrictedDownload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated"]
        pub can_change_restricted_download: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can comment on this file."]
        pub can_comment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canCopy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder."]
        pub can_copy: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDelete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete this file."]
        pub can_delete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDeleteChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
        pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDownload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can download this file."]
        pub can_download: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canEdit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can edit this file. Other factors may limit the type of changes a user can make to a file. For example, see canChangeCopyRequiresWriterPermission or canModifyContent."]
        pub can_edit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canListChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can list the children of this folder. This is always false when the item is not a folder."]
        pub can_list_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canModifyContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can modify the content of this file."]
        pub can_modify_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canModifyContentRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can modify restrictions on content of this file."]
        pub can_modify_content_restriction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveChildrenOutOfDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives."]
        pub can_move_children_out_of_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveChildrenOutOfTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveChildrenOutOfDrive instead."]
        pub can_move_children_out_of_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveChildrenWithinDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can move children of this folder within this drive. This is false when the item is not a folder. Note that a request to move the child may still fail depending on the current user's access to the child and to the destination folder."]
        pub can_move_children_within_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveChildrenWithinTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveChildrenWithinDrive instead."]
        pub can_move_children_within_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveItemIntoTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
        pub can_move_item_into_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveItemOutOfDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added."]
        pub can_move_item_out_of_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveItemOutOfTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
        pub can_move_item_out_of_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveItemWithinDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can move this item within this drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added and the parent that is being removed."]
        pub can_move_item_within_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveItemWithinTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveItemWithinDrive instead."]
        pub can_move_item_within_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canMoveTeamDriveItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead."]
        pub can_move_team_drive_item: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives."]
        pub can_read_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read."]
        pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canReadDrive instead."]
        pub can_read_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRemoveChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead."]
        pub can_remove_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRemoveMyDriveParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can remove a parent from the item without adding another parent in the same request. Not populated for shared drive files."]
        pub can_remove_my_drive_parent: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can rename this file."]
        pub can_rename: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canShare")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can modify the sharing settings for this file."]
        pub can_share: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canTrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can move this file to trash."]
        pub can_trash: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canTrashChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
        pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canUntrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can restore this file from trash."]
        pub can_untrash: ::std::option::Option<::std::primitive::bool>,
    }
    impl FileCapabilities {
        pub fn builder() -> FileCapabilitiesBuilder {
            FileCapabilitiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content."]
    pub struct FileImageMediaMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aperture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The aperture used to create the photo (f-number)."]
        pub aperture: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cameraMake")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The make of the camera used to create the photo."]
        pub camera_make: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cameraModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The model of the camera used to create the photo."]
        pub camera_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorSpace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color space of the photo."]
        pub color_space: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time the photo was taken (EXIF format timestamp)."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exposureBias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exposure bias of the photo (APEX value)."]
        pub exposure_bias: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exposureMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exposure mode used to create the photo."]
        pub exposure_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exposureTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of the exposure, in seconds."]
        pub exposure_time: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flashUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether a flash was used to create the photo."]
        pub flash_used: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "focalLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The focal length used to create the photo, in millimeters."]
        pub focal_length: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the image in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isoSpeed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ISO speed used to create the photo."]
        pub iso_speed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lens used to create the photo."]
        pub lens: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Geographic location information stored in the image."]
        pub location: ::std::option::Option<FileImageMediaMetadataLocation>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxApertureValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The smallest f-number of the lens at the focal length used to create the photo (APEX value)."]
        pub max_aperture_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meteringMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metering mode used to create the photo."]
        pub metering_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of clockwise 90 degree rotations applied from the image's original orientation."]
        pub rotation: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sensor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of sensor used to create the photo."]
        pub sensor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subjectDistance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The distance to the subject of the photo, in meters."]
        pub subject_distance: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whiteBalance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The white balance mode used to create the photo."]
        pub white_balance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the image in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl FileImageMediaMetadata {
        pub fn builder() -> FileImageMediaMetadataBuilder {
            FileImageMediaMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Geographic location information stored in the image."]
    pub struct FileImageMediaMetadataLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The altitude stored in the image."]
        pub altitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latitude stored in the image."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The longitude stored in the image."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl FileImageMediaMetadataLocation {
        pub fn builder() -> FileImageMediaMetadataLocationBuilder {
            FileImageMediaMetadataLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indexable text attributes for the file (can only be written)"]
    pub struct FileIndexableText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text to be indexed for this file."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl FileIndexableText {
        pub fn builder() -> FileIndexableTextBuilder {
            FileIndexableTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A group of labels for the file."]
    pub struct FileLabels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hidden")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub hidden: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been modified by this user."]
        pub modified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restricted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use copyRequiresWriterPermission instead."]
        pub restricted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this file is starred by the user."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file."]
        pub trashed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this file has been viewed by this user."]
        pub viewed: ::std::option::Option<::std::primitive::bool>,
    }
    impl FileLabels {
        pub fn builder() -> FileLabelsBuilder {
            FileLabelsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
    pub struct FileShortcutDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file that this shortcut points to."]
        pub target_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetMimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the file that this shortcut points to. The value of this field is a snapshot of the target's MIME type, captured when the shortcut is created."]
        pub target_mime_type: ::std::option::Option<::std::string::String>,
    }
    impl FileShortcutDetails {
        pub fn builder() -> FileShortcutDetailsBuilder {
            FileShortcutDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated."]
    pub struct FileThumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL-safe Base64 encoded bytes of the thumbnail image. It should conform to RFC 4648 section 5."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the thumbnail."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl FileThumbnail {
        pub fn builder() -> FileThumbnailBuilder {
            FileThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about video media. This will only be present for video types."]
    pub struct FileVideoMediaMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration of the video in milliseconds."]
        pub duration_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the video in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the video in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl FileVideoMediaMetadata {
        pub fn builder() -> FileVideoMediaMetadataBuilder {
            FileVideoMediaMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of files."]
    pub struct FileList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incompleteSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"default\" or \"drive\"."]
        pub incomplete_search: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
        #[builder(default = "{ file_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "file_list_defaults :: kind")]
        #[doc = "This is always drive#fileList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the next page of files."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl FileList {
        pub fn builder() -> FileListBuilder {
            FileListBuilder::default()
        }
    }
    mod file_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#fileList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of generated IDs which can be provided in insert requests"]
    pub struct GeneratedIds {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs generated for the requesting user in the specified space."]
        pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ generated_ids_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "generated_ids_defaults :: kind")]
        #[doc = "This is always drive#generatedIds"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "space")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of file that can be created with these IDs."]
        pub space: ::std::option::Option<::std::string::String>,
    }
    impl GeneratedIds {
        pub fn builder() -> GeneratedIdsBuilder {
            GeneratedIdsBuilder::default()
        }
    }
    mod generated_ids_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#generatedIds")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of a file's parents."]
    pub struct ParentList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of parents."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParentReference>>>,
        #[builder(default = "{ parent_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "parent_list_defaults :: kind")]
        #[doc = "This is always drive#parentList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ParentList {
        pub fn builder() -> ParentListBuilder {
            ParentListBuilder::default()
        }
    }
    mod parent_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#parentList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a file's parent."]
    pub struct ParentReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the parent."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRoot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the parent is the root folder."]
        pub is_root: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ parent_reference_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "parent_reference_defaults :: kind")]
        #[doc = "This is always drive#parentReference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the parent."]
        pub parent_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this reference."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ParentReference {
        pub fn builder() -> ParentReferenceBuilder {
            ParentReferenceBuilder::default()
        }
    }
    mod parent_reference_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#parentReference")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A permission for a file."]
    pub struct Permission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional roles for this user. Only commenter is currently allowed, though more may be supported in the future."]
        pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub auth_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is user, group or domain."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is user or group."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the permission."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions:  \n- They cannot be set on shared drive items \n- They can only be set on user and group permissions \n- The date must be in the future \n- The date cannot be more than a year in the future \n- The date can only be set on drive.permissions.update or drive.permissions.patch requests"]
        pub expiration_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the user this permission refers to, and identical to the permissionId in the About and Files resources. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ permission_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "permission_defaults :: kind")]
        #[doc = "This is always drive#permission."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name for this permission."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items."]
        pub permission_details: ::std::option::Option<::std::vec::Vec<PermissionPermissionDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the profile photo, if available."]
        pub photo_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary role for this user. While new values may be supported in the future, the following are currently allowed:  \n- owner \n- organizer \n- fileOrganizer \n- writer \n- reader"]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this permission."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrivePermissionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails instead."]
        pub team_drive_permission_details:
            ::std::option::Option<::std::vec::Vec<PermissionTeamDrivePermissionDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account type. Allowed values are:  \n- user \n- group \n- domain \n- anyone"]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored."]
        pub value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value."]
        pub view: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "withLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the link is required for this permission."]
        pub with_link: ::std::option::Option<::std::primitive::bool>,
    }
    impl Permission {
        pub fn builder() -> PermissionBuilder {
            PermissionBuilder::default()
        }
    }
    mod permission_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#permission")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PermissionPermissionDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional roles for this user. Only commenter is currently possible, though more may be supported in the future."]
        pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inherited")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this permission is inherited. This field is always populated. This is an output-only field."]
        pub inherited: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inheritedFrom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the item from which this permission is inherited. This is an output-only field."]
        pub inherited_from: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permission type for this user. While new values may be added in future, the following are currently possible:  \n- file \n- member"]
        pub permission_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary role for this user. While new values may be added in the future, the following are currently possible:  \n- organizer \n- fileOrganizer \n- writer \n- reader"]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl PermissionPermissionDetails {
        pub fn builder() -> PermissionPermissionDetailsBuilder {
            PermissionPermissionDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PermissionTeamDrivePermissionDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails/additionalRoles instead."]
        pub additional_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inherited")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails/inherited instead."]
        pub inherited: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inheritedFrom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails/inheritedFrom instead."]
        pub inherited_from: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails/role instead."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrivePermissionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails/permissionType instead."]
        pub team_drive_permission_type: ::std::option::Option<::std::string::String>,
    }
    impl PermissionTeamDrivePermissionDetails {
        pub fn builder() -> PermissionTeamDrivePermissionDetailsBuilder {
            PermissionTeamDrivePermissionDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An ID for a user or group as seen in Permission items."]
    pub struct PermissionId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permission ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ permission_id_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "permission_id_defaults :: kind")]
        #[doc = "This is always drive#permissionId."]
        pub kind: ::std::string::String,
    }
    impl PermissionId {
        pub fn builder() -> PermissionIdBuilder {
            PermissionIdBuilder::default()
        }
    }
    mod permission_id_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#permissionId")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of permissions associated with a file."]
    pub struct PermissionList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of permissions."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
        #[builder(default = "{ permission_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "permission_list_defaults :: kind")]
        #[doc = "This is always drive#permissionList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl PermissionList {
        pub fn builder() -> PermissionListBuilder {
            PermissionListBuilder::default()
        }
    }
    mod permission_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#permissionList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A key-value pair attached to a file that is either public or private to an application.\nThe following limits apply to file properties:  \n- Maximum of 100 properties total per file\n- Maximum of 30 private properties per app\n- Maximum of 30 public properties\n- Maximum of 124 bytes size limit on (key + value) string in UTF-8 encoding for a single property."]
    pub struct Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the property."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of this property."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ property_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "property_defaults :: kind")]
        #[doc = "This is always drive#property."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link back to this property."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of this property."]
        pub value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visibility of this property. Allowed values are PRIVATE and PUBLIC. (Default: PRIVATE). Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties."]
        pub visibility: ::std::option::Option<::std::string::String>,
    }
    impl Property {
        pub fn builder() -> PropertyBuilder {
            PropertyBuilder::default()
        }
    }
    mod property_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#property")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of properties, key-value pairs that are either public or private to an application."]
    pub struct PropertyList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of properties."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Property>>>,
        #[builder(default = "{ property_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "property_list_defaults :: kind")]
        #[doc = "This is always drive#propertyList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl PropertyList {
        pub fn builder() -> PropertyListBuilder {
            PropertyListBuilder::default()
        }
    }
    mod property_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#propertyList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A revision of a file."]
    pub struct Revision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the revision."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links for exporting Docs Editors files to specific formats."]
        pub export_links:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the revision in bytes. This will only be populated on files with content stored in Drive."]
        pub file_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the revision."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ revision_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "revision_defaults :: kind")]
        #[doc = "This is always drive#revision."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last user to modify this revision."]
        pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the last user to modify this revision."]
        pub last_modifying_user_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive."]
        pub md5_checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the revision."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this revision was modified (formatted RFC 3339 timestamp)."]
        pub modified_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalFilename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original filename when this revision was created. This will only be populated on files with content stored in Drive."]
        pub original_filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pinned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this revision is pinned to prevent automatic purging. This will only be populated and can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions."]
        pub pinned: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishAuto")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files."]
        pub publish_auto: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this revision is published. This is only populated and can only be modified for Docs Editors files."]
        pub published: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the published revision. This is only populated for Google Sites files."]
        pub published_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedOutsideDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files."]
        pub published_outside_domain: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this revision."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl Revision {
        pub fn builder() -> RevisionBuilder {
            RevisionBuilder::default()
        }
    }
    mod revision_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#revision")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of revisions of a file."]
    pub struct RevisionList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag of the list."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Revision>>>,
        #[builder(default = "{ revision_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "revision_list_defaults :: kind")]
        #[doc = "This is always drive#revisionList."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of revisions. This field will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link back to this list."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl RevisionList {
        pub fn builder() -> RevisionListBuilder {
            RevisionListBuilder::default()
        }
    }
    mod revision_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#revisionList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct StartPageToken {
        #[builder(default = "{ start_page_token_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "start_page_token_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#startPageToken\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting page token for listing changes."]
        pub start_page_token: ::std::option::Option<::std::string::String>,
    }
    impl StartPageToken {
        pub fn builder() -> StartPageTokenBuilder {
            StartPageTokenBuilder::default()
        }
    }
    mod start_page_token_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#startPageToken")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated: use the drive collection instead."]
    pub struct TeamDrive {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
        pub background_image_file: ::std::option::Option<TeamDriveBackgroundImageFile>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short-lived link to this Team Drive's background image."]
        pub background_image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Capabilities the current user has on this Team Drive."]
        pub capabilities: ::std::option::Option<TeamDriveCapabilities>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId."]
        pub color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the Team Drive was created (RFC 3339 date-time)."]
        pub created_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this Team Drive which is also the ID of the top level folder of this Team Drive."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ team_drive_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "team_drive_defaults :: kind")]
        #[doc = "This is always drive#teamDrive"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this Team Drive."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of restrictions that apply to this Team Drive or items inside this Team Drive."]
        pub restrictions: ::std::option::Option<TeamDriveRestrictions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.insert request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
        pub theme_id: ::std::option::Option<::std::string::String>,
    }
    impl TeamDrive {
        pub fn builder() -> TeamDriveBuilder {
            TeamDriveBuilder::default()
        }
    }
    mod team_drive_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#teamDrive")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
    pub struct TeamDriveBackgroundImageFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of an image file in Drive to use for the background image."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
        pub width: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xCoordinate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
        pub x_coordinate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yCoordinate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
        pub y_coordinate: ::std::option::Option<::std::primitive::f64>,
    }
    impl TeamDriveBackgroundImageFile {
        pub fn builder() -> TeamDriveBackgroundImageFileBuilder {
            TeamDriveBackgroundImageFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Capabilities the current user has on this Team Drive."]
    pub struct TeamDriveCapabilities {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canAddChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add children to folders in this Team Drive."]
        pub can_add_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive."]
        pub can_change_copy_requires_writer_permission_restriction:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the domainUsersOnly restriction of this Team Drive."]
        pub can_change_domain_users_only_restriction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeTeamDriveBackground")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the background of this Team Drive."]
        pub can_change_team_drive_background: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canChangeTeamMembersOnlyRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can change the teamMembersOnly restriction of this Team Drive."]
        pub can_change_team_members_only_restriction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can comment on files in this Team Drive."]
        pub can_comment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canCopy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can copy files in this Team Drive."]
        pub can_copy: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDeleteChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete children from folders in this Team Drive."]
        pub can_delete_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDeleteTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive."]
        pub can_delete_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDownload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can download files in this Team Drive."]
        pub can_download: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canEdit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can edit files in this Team Drive"]
        pub can_edit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canListChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can list the children of folders in this Team Drive."]
        pub can_list_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canManageMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can add members to this Team Drive or remove them or change their role."]
        pub can_manage_members: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReadRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can read the revisions resource of files in this Team Drive."]
        pub can_read_revisions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRemoveChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use canDeleteChildren or canTrashChildren instead."]
        pub can_remove_children: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can rename files or folders in this Team Drive."]
        pub can_rename: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRenameTeamDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can rename this Team Drive."]
        pub can_rename_team_drive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canShare")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can share files or folders in this Team Drive."]
        pub can_share: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canTrashChildren")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current user can trash children from folders in this Team Drive."]
        pub can_trash_children: ::std::option::Option<::std::primitive::bool>,
    }
    impl TeamDriveCapabilities {
        pub fn builder() -> TeamDriveCapabilitiesBuilder {
            TeamDriveCapabilitiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of restrictions that apply to this Team Drive or items inside this Team Drive."]
    pub struct TeamDriveRestrictions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adminManagedRestrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether administrative privileges on this Team Drive are required to modify restrictions."]
        pub admin_managed_restrictions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyRequiresWriterPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive."]
        pub copy_requires_writer_permission: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainUsersOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive."]
        pub domain_users_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamMembersOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether access to items inside this Team Drive is restricted to members of this Team Drive."]
        pub team_members_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl TeamDriveRestrictions {
        pub fn builder() -> TeamDriveRestrictionsBuilder {
            TeamDriveRestrictionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of Team Drives."]
    pub struct TeamDriveList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Team Drives."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeamDrive>>>,
        #[builder(default = "{ team_drive_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "team_drive_list_defaults :: kind")]
        #[doc = "This is always drive#teamDriveList"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of Team Drives."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl TeamDriveList {
        pub fn builder() -> TeamDriveListBuilder {
            TeamDriveListBuilder::default()
        }
    }
    mod team_drive_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#teamDriveList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a Drive user."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A plain text displayable name for this user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAuthenticatedUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this user is the same as the authenticated user for whom the request was made."]
        pub is_authenticated_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ user_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "user_defaults :: kind")]
        #[doc = "This is always drive#user."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's ID as visible in the permissions collection."]
        pub permission_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "picture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's profile picture."]
        pub picture: ::std::option::Option<UserPicture>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    mod user_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#user")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The user's profile picture."]
    pub struct UserPicture {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL that points to a profile picture of this user."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl UserPicture {
        pub fn builder() -> UserPictureBuilder {
            UserPictureBuilder::default()
        }
    }
}
