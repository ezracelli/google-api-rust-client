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
    pub mod changes {
        pub mod methods {
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
                        default = "{ query_parameters_defaults :: include_removed () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeRemoved")]
                    #[serde(default = "query_parameters_defaults :: include_removed")]
                    #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
                    pub include_removed: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_team_drive_items () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeTeamDriveItems")]
                    #[serde(default = "query_parameters_defaults :: include_team_drive_items")]
                    #[doc = "Deprecated use includeItemsFromAllDrives instead."]
                    pub include_team_drive_items: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of changes to return per page."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."]
                    pub page_token: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: restrict_to_my_drive () }",
                        setter(into)
                    )]
                    #[serde(rename = "restrictToMyDrive")]
                    #[serde(default = "query_parameters_defaults :: restrict_to_my_drive")]
                    #[doc = "Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive."]
                    pub restrict_to_my_drive: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: spaces () }",
                        setter(into)
                    )]
                    #[serde(rename = "spaces")]
                    #[serde(default = "query_parameters_defaults :: spaces")]
                    #[doc = "A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::string::String,
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
                    pub fn include_items_from_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_removed() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_team_drive_items() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn restrict_to_my_drive() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn spaces() -> ::std::string::String {
                        String::from("drive")
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
                        default = "{ query_parameters_defaults :: include_removed () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeRemoved")]
                    #[serde(default = "query_parameters_defaults :: include_removed")]
                    #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
                    pub include_removed: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: include_team_drive_items () }",
                        setter(into)
                    )]
                    #[serde(rename = "includeTeamDriveItems")]
                    #[serde(default = "query_parameters_defaults :: include_team_drive_items")]
                    #[doc = "Deprecated use includeItemsFromAllDrives instead."]
                    pub include_team_drive_items: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of changes to return per page."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method."]
                    pub page_token: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: restrict_to_my_drive () }",
                        setter(into)
                    )]
                    #[serde(rename = "restrictToMyDrive")]
                    #[serde(default = "query_parameters_defaults :: restrict_to_my_drive")]
                    #[doc = "Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive."]
                    pub restrict_to_my_drive: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: spaces () }",
                        setter(into)
                    )]
                    #[serde(rename = "spaces")]
                    #[serde(default = "query_parameters_defaults :: spaces")]
                    #[doc = "A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::string::String,
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
                    pub fn include_items_from_all_drives() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn include_removed() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn include_team_drive_items() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn restrict_to_my_drive() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn spaces() -> ::std::string::String {
                        String::from("drive")
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
                    #[doc = "Whether to return deleted comments. Deleted comments will not include their original content."]
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
                    #[doc = "Whether to include deleted comments. Deleted comments will not include their original content."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of comments to return per page."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startModifiedTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time)."]
                    pub start_modified_time: ::std::option::Option<::std::string::String>,
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
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                }
            }
        }
    }
    pub mod drives {
        pub mod methods {
            pub mod create {
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
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "Maximum number of shared drives to return."]
                    pub page_size: ::std::primitive::i64,
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
                    pub fn page_size() -> ::std::primitive::i64 {
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
                        default = "{ query_parameters_defaults :: enforce_single_parent () }",
                        setter(into)
                    )]
                    #[serde(rename = "enforceSingleParent")]
                    #[serde(default = "query_parameters_defaults :: enforce_single_parent")]
                    #[doc = "Deprecated. Copying files into multiple folders is no longer supported. Use shortcuts instead."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: ignore_default_visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "ignoreDefaultVisibility")]
                    #[serde(default = "query_parameters_defaults :: ignore_default_visibility")]
                    #[doc = "Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders."]
                    pub ignore_default_visibility: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: keep_revision_forever () }",
                        setter(into)
                    )]
                    #[serde(rename = "keepRevisionForever")]
                    #[serde(default = "query_parameters_defaults :: keep_revision_forever")]
                    #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions."]
                    pub keep_revision_forever: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
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
                    pub fn ignore_default_visibility() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn keep_revision_forever() -> ::std::primitive::bool {
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
            pub mod create {
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
                    #[doc = "Deprecated. Creating files in multiple folders is no longer supported."]
                    pub enforce_single_parent: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: ignore_default_visibility () }",
                        setter(into)
                    )]
                    #[serde(rename = "ignoreDefaultVisibility")]
                    #[serde(default = "query_parameters_defaults :: ignore_default_visibility")]
                    #[doc = "Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders."]
                    pub ignore_default_visibility: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includePermissionsForView")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which additional view's permissions to include in the response. Only 'published' is supported."]
                    pub include_permissions_for_view: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: keep_revision_forever () }",
                        setter(into)
                    )]
                    #[serde(rename = "keepRevisionForever")]
                    #[serde(default = "query_parameters_defaults :: keep_revision_forever")]
                    #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions."]
                    pub keep_revision_forever: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
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
                        default = "{ query_parameters_defaults :: use_content_as_indexable_text () }",
                        setter(into)
                    )]
                    #[serde(rename = "useContentAsIndexableText")]
                    #[serde(
                        default = "query_parameters_defaults :: use_content_as_indexable_text"
                    )]
                    #[doc = "Whether to use the uploaded content as indexable text."]
                    pub use_content_as_indexable_text: ::std::primitive::bool,
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
                    pub fn ignore_default_visibility() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn keep_revision_forever() -> ::std::primitive::bool {
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
                    #[builder(default = "{ query_parameters_defaults :: count () }", setter(into))]
                    #[serde(rename = "count")]
                    #[serde(default = "query_parameters_defaults :: count")]
                    #[doc = "The number of IDs to return."]
                    pub count: ::std::primitive::i64,
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
                    pub fn count() -> ::std::primitive::i64 {
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
                    #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
                    pub acknowledge_abuse: ::std::primitive::bool,
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
                    pub fn acknowledge_abuse() -> ::std::primitive::bool {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "corpora")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency."]
                    pub corpora: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "corpus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The source of files to list. Deprecated: use 'corpora' instead."]
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A query for filtering the file results. See the \"Search for Files\" guide for supported syntax."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: spaces () }",
                        setter(into)
                    )]
                    #[serde(rename = "spaces")]
                    #[serde(default = "query_parameters_defaults :: spaces")]
                    #[doc = "A comma-separated list of spaces to query within the corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
                    pub spaces: ::std::string::String,
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
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"100").unwrap()
                    }
                    pub fn spaces() -> ::std::string::String {
                        String::from("drive")
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
                #[doc = "The source of files to list. Deprecated: use 'corpora' instead."]
                pub enum QueryParametersCorpusEnum {
                    #[serde(rename = "domain")]
                    #[doc = "Files shared to the user's domain."]
                    Domain,
                    #[serde(rename = "user")]
                    #[doc = "Files owned by or shared to the user. If a user has permissions on a Shared Drive, the files inside it won't be retrieved unless the user has created, opened, or shared the file."]
                    User,
                }
                impl ::std::default::Default for QueryParametersCorpusEnum {
                    fn default() -> Self {
                        Self::Domain
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
                    #[doc = "A comma-separated list of parent IDs to add."]
                    pub add_parents: ::std::option::Option<::std::string::String>,
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
                    #[builder(
                        default = "{ query_parameters_defaults :: keep_revision_forever () }",
                        setter(into)
                    )]
                    #[serde(rename = "keepRevisionForever")]
                    #[serde(default = "query_parameters_defaults :: keep_revision_forever")]
                    #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions."]
                    pub keep_revision_forever: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ocrLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
                    pub ocr_language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "removeParents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A comma-separated list of parent IDs to remove."]
                    pub remove_parents: ::std::option::Option<::std::string::String>,
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
                        default = "{ query_parameters_defaults :: use_content_as_indexable_text () }",
                        setter(into)
                    )]
                    #[serde(rename = "useContentAsIndexableText")]
                    #[serde(
                        default = "query_parameters_defaults :: use_content_as_indexable_text"
                    )]
                    #[doc = "Whether to use the uploaded content as indexable text."]
                    pub use_content_as_indexable_text: ::std::primitive::bool,
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
                    pub fn keep_revision_forever() -> ::std::primitive::bool {
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
                    #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
                    pub acknowledge_abuse: ::std::primitive::bool,
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
                    pub fn acknowledge_abuse() -> ::std::primitive::bool {
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
            pub mod create {
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
                    #[doc = "A plain text custom message to include in the notification email."]
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotificationEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers."]
                    pub send_notification_email: ::std::option::Option<::std::primitive::bool>,
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
                    #[doc = "Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect."]
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
                    pub fn enforce_single_parent() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn move_to_new_owners_root() -> ::std::primitive::bool {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
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
                    #[doc = "Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect."]
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
                    #[doc = "Whether to return deleted replies. Deleted replies will not include their original content."]
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
                    #[doc = "Whether to include deleted replies. Deleted replies will not include their original content."]
                    pub include_deleted: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of replies to return per page."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
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
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                }
            }
        }
    }
    pub mod revisions {
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
                        default = "{ query_parameters_defaults :: acknowledge_abuse () }",
                        setter(into)
                    )]
                    #[serde(rename = "acknowledgeAbuse")]
                    #[serde(default = "query_parameters_defaults :: acknowledge_abuse")]
                    #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
                    pub acknowledge_abuse: ::std::primitive::bool,
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
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "The maximum number of revisions to return per page."]
                    pub page_size: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn page_size() -> ::std::primitive::i64 {
                        serde_json::from_str(&"200").unwrap()
                    }
                }
            }
        }
    }
    pub mod teamdrives {
        pub mod methods {
            pub mod create {
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
                        default = "{ query_parameters_defaults :: page_size () }",
                        setter(into)
                    )]
                    #[serde(rename = "pageSize")]
                    #[serde(default = "query_parameters_defaults :: page_size")]
                    #[doc = "Maximum number of Team Drives to return."]
                    pub page_size: ::std::primitive::i64,
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
                    pub fn page_size() -> ::std::primitive::i64 {
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
    #[doc = "Information about the user, the user's Drive, and system capabilities."]
    pub struct About {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appInstalled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user has installed the requesting app."]
        pub app_installed: ::std::option::Option<::std::primitive::bool>,
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
        #[serde(rename = "driveThemes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of themes that are supported for shared drives."]
        pub drive_themes: ::std::option::Option<::std::vec::Vec<AboutDriveThemes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of source MIME type to possible targets for all supported exports."]
        pub export_formats: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::vec::Vec<::std::string::String>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderColorPalette")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currently supported folder colors as RGB hex strings."]
        pub folder_color_palette: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of source MIME type to possible targets for all supported imports."]
        pub import_formats: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::vec::Vec<::std::string::String>>,
        >,
        #[builder(default = "{ about_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "about_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#about\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxImportSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of maximum import sizes by MIME type, in bytes."]
        pub max_import_sizes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxUploadSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum upload size in bytes."]
        pub max_upload_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's storage quota limits and usage. All fields are measured in bytes."]
        pub storage_quota: ::std::option::Option<AboutStorageQuota>,
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
    #[doc = "The user's storage quota limits and usage. All fields are measured in bytes."]
    pub struct AboutStorageQuota {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The usage limit, if applicable. This will not be present if the user has unlimited storage."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total usage across all services."]
        pub usage: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usageInDrive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The usage by all files in Google Drive."]
        pub usage_in_drive: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usageInDriveTrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The usage by trashed files in Google Drive."]
        pub usage_in_drive_trash: ::std::option::Option<::std::string::String>,
    }
    impl AboutStorageQuota {
        pub fn builder() -> AboutStorageQuotaBuilder {
            AboutStorageQuotaBuilder::default()
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
    #[doc = "A change to a file or shared drive."]
    pub struct Change {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the change. Possible values are file and drive."]
        pub change_type: ::std::option::Option<::std::string::String>,
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
        #[doc = "The ID of the file which has changed."]
        pub file_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ change_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#change\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access."]
        pub removed: ::std::option::Option<::std::primitive::bool>,
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
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time of this change (RFC 3339 date-time)."]
        pub time: ::std::option::Option<::std::string::String>,
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
        #[serde(rename = "changes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub changes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
        #[builder(default = "{ change_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#changeList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newStartPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting page token for future changes. This will be present only if the end of the current changes list has been reached."]
        pub new_start_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
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
    #[doc = "A comment on a file."]
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
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the comment was created (RFC 3339 date-time)."]
        pub created_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the comment has been deleted. A deleted comment has no content."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content of the comment with HTML formatting."]
        pub html_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the comment."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ comment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#comment\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the comment or any of its replies was modified (RFC 3339 date-time)."]
        pub modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotedFileContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment."]
        pub quoted_file_content: ::std::option::Option<CommentQuotedFileContent>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full list of replies to the comment in chronological order."]
        pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Reply>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolved")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the comment has been resolved by one of its replies."]
        pub resolved: ::std::option::Option<::std::primitive::bool>,
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
    #[doc = "The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment."]
    pub struct CommentQuotedFileContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the quoted content."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quoted content itself. This is interpreted as plain text if set through the API."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CommentQuotedFileContent {
        pub fn builder() -> CommentQuotedFileContentBuilder {
            CommentQuotedFileContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of comments on a file."]
    pub struct CommentList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub comments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
        #[builder(default = "{ comment_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#commentList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
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
        #[serde(rename = "restrictionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true."]
        pub restriction_time: ::std::option::Option<::std::string::String>,
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
        #[serde(rename = "createdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the shared drive was created (RFC 3339 date-time)."]
        pub created_time: ::std::option::Option<::std::string::String>,
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
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#drive\"."]
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
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
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
        #[serde(rename = "drives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub drives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Drive>>>,
        #[builder(default = "{ drive_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "drive_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#driveList\"."]
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
        #[serde(rename = "appProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of arbitrary key-value pairs which are private to the requesting app.\nEntries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties."]
        pub app_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
        pub capabilities: ::std::option::Option<FileCapabilities>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the content of the file. These fields are never populated in responses."]
        pub content_hints: ::std::option::Option<FileContentHints>,
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
        #[serde(rename = "createdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the file was created (RFC 3339 date-time)."]
        pub created_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of the file."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the shared drive the file resides in. Only populated for items in shared drives."]
        pub drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitlyTrashed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder."]
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
        #[doc = "The final component of fullFileExtension. This is only available for files with binary content in Google Drive."]
        pub file_extension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderColorRgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.\nIf an unsupported color is specified, the closest color in the palette will be used instead."]
        pub folder_color_rgb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullFileExtension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full file extension extracted from the name field. May contain multiple concatenated extensions, such as \"tar.gz\". This is only available for files with binary content in Google Drive.\nThis is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension."]
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
        #[doc = "The ID of the file's head revision. This is currently only available for files with binary content in Google Drive."]
        pub head_revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A static, unauthenticated link to the file's icon."]
        pub icon_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageMediaMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional metadata about image media, if available."]
        pub image_media_metadata: ::std::option::Option<FileImageMediaMetadata>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAppAuthorized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file was created or opened by the requesting app."]
        pub is_app_authorized: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ file_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "file_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#file\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last user to modify the file."]
        pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive."]
        pub md5_checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the file.\nGoogle Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.\nIf a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedByMe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been modified by this user."]
        pub modified_by_me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedByMeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the file was modified by the user (RFC 3339 date-time)."]
        pub modified_by_me_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the file was modified by anyone (RFC 3339 date-time).\nNote that setting modifiedTime will also update modifiedByMeTime for the user."]
        pub modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalFilename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive."]
        pub original_filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownedByMe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user owns the file. Not populated for items in shared drives."]
        pub owned_by_me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The owners of the file. Currently, only certain legacy files may have more than one owner. Not populated for items in shared drives."]
        pub owners: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of the parent folders which contain the file.\nIf not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list."]
        pub parents: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of permission IDs for users with access to this file."]
        pub permission_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of arbitrary key-value pairs which are visible to all apps.\nEntries with null values are cleared in update and copy requests."]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaBytesUsed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled."]
        pub quota_bytes_used: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shared")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been shared. Not populated for items in shared drives."]
        pub shared: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedWithMeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the file was shared with the user, if applicable (RFC 3339 date-time)."]
        pub shared_with_me_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who shared the file with the requesting user, if applicable."]
        pub sharing_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortcutDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut."]
        pub shortcut_details: ::std::option::Option<FileShortcutDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the file's content in bytes. This is applicable to binary files in Google Drive and Google Docs files."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spaces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'."]
        pub spaces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user has starred the file."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use driveId instead."]
        pub team_drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request."]
        pub thumbnail_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail version for use in thumbnail cache invalidation."]
        pub thumbnail_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file."]
        pub trashed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives."]
        pub trashed_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trashingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives."]
        pub trashing_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user."]
        pub version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoMediaMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional metadata about video media. This may not be available immediately upon upload."]
        pub video_media_metadata: ::std::option::Option<FileVideoMediaMetadata>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewedByMe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the file has been viewed by this user."]
        pub viewed_by_me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewedByMeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the file was viewed by the user (RFC 3339 date-time)."]
        pub viewed_by_me_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewersCanCopyContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use copyRequiresWriterPermission instead."]
        pub viewers_can_copy_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webContentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive."]
        pub web_content_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webViewLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link for opening the file in a relevant Google editor or viewer in a browser."]
        pub web_view_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writersCanShare")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives."]
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
        #[serde(rename = "canChangeViewersCanCopyContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated"]
        pub can_change_viewers_can_copy_content: ::std::option::Option<::std::primitive::bool>,
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
    #[doc = "Additional information about the content of the file. These fields are never populated in responses."]
    pub struct FileContentHints {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexableText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements."]
        pub indexable_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail."]
        pub thumbnail: ::std::option::Option<FileContentHintsThumbnail>,
    }
    impl FileContentHints {
        pub fn builder() -> FileContentHintsBuilder {
            FileContentHintsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail."]
    pub struct FileContentHintsThumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5)."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the thumbnail."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl FileContentHintsThumbnail {
        pub fn builder() -> FileContentHintsThumbnailBuilder {
            FileContentHintsThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional metadata about image media, if available."]
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
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time the photo was taken (EXIF DateTime)."]
        pub time: ::std::option::Option<::std::string::String>,
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
    #[doc = "Additional metadata about video media. This may not be available immediately upon upload."]
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
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incompleteSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"user\" or \"drive\"."]
        pub incomplete_search: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ file_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "file_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#fileList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
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
    #[doc = "A list of generated file IDs which can be provided in create requests."]
    pub struct GeneratedIds {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs generated for the requesting user in the specified space."]
        pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ generated_ids_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "generated_ids_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#generatedIds\"."]
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
    #[doc = "A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy."]
    pub struct Permission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowFileDiscovery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone."]
        pub allow_file_discovery: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The \"pretty\" name of the value of the permission. The following is a list of examples for each type of permission:  \n- user - User's full name, as defined for their Google account, such as \"Joe Smith.\" \n- group - Name of the Google Group, such as \"The Company Administrators.\" \n- domain - String domain name, such as \"thecompany.com.\" \n- anyone - No displayName is present."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain to which this permission refers."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user or group to which this permission refers."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  \n- They can only be set on user and group permissions \n- The time must be in the future \n- The time cannot be more than a year in the future"]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId. IDs should be treated as opaque values."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ permission_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "permission_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permission\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items."]
        pub permission_details: ::std::option::Option<::std::vec::Vec<PermissionPermissionDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the user's profile photo, if available."]
        pub photo_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  \n- owner \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrivePermissionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated - use permissionDetails instead."]
        pub team_drive_permission_details:
            ::std::option::Option<::std::vec::Vec<PermissionTeamDrivePermissionDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the grantee. Valid values are:  \n- user \n- group \n- domain \n- anyone  When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value."]
        pub view: ::std::option::Option<::std::string::String>,
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
        #[doc = "The primary role for this user. While new values may be added in the future, the following are currently possible:  \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
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
    #[doc = "A list of permissions for a file."]
    pub struct PermissionList {
        #[builder(default = "{ permission_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "permission_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permissionList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
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
    #[doc = "A reply to a comment on a file."]
    pub struct Reply {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action the reply performed to the parent comment. Valid values are:  \n- resolve \n- reopen"]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of the reply. The author's email address and permission ID will not be populated."]
        pub author: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the reply was created (RFC 3339 date-time)."]
        pub created_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the reply has been deleted. A deleted reply has no content."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content of the reply with HTML formatting."]
        pub html_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the reply."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ reply_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "reply_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#reply\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the reply was modified (RFC 3339 date-time)."]
        pub modified_time: ::std::option::Option<::std::string::String>,
    }
    impl Reply {
        pub fn builder() -> ReplyBuilder {
            ReplyBuilder::default()
        }
    }
    mod reply_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#reply")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of replies to a comment on a file."]
    pub struct ReplyList {
        #[builder(default = "{ reply_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "reply_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#replyList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Reply>>>,
    }
    impl ReplyList {
        pub fn builder() -> ReplyListBuilder {
            ReplyListBuilder::default()
        }
    }
    mod reply_list_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("drive#replyList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for a revision to a file."]
    pub struct Revision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links for exporting Docs Editors files to specific formats."]
        pub export_links:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the revision."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keepForever")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.\nThis field is only applicable to files with binary content in Drive."]
        pub keep_forever: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ revision_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "revision_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revision\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyingUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last user to modify this revision."]
        pub last_modifying_user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive."]
        pub md5_checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the revision."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the revision was modified (RFC 3339 date-time)."]
        pub modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalFilename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original filename used to create this revision. This is only applicable to files with binary content in Drive."]
        pub original_filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishAuto")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files."]
        pub publish_auto: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this revision is published. This is only applicable to Docs Editors files."]
        pub published: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the published revision. This is only populated for Google Sites files."]
        pub published_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedOutsideDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this revision is published outside the domain. This is only applicable to Docs Editors files."]
        pub published_outside_domain: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the revision's content in bytes. This is only applicable to files with binary content in Drive."]
        pub size: ::std::option::Option<::std::string::String>,
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
        #[builder(default = "{ revision_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "revision_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revisionList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub revisions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Revision>>>,
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
        #[serde(rename = "createdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the Team Drive was created (RFC 3339 date-time)."]
        pub created_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this Team Drive which is also the ID of the top level folder of this Team Drive."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ team_drive_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "team_drive_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDrive\"."]
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
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
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
        #[builder(default = "{ team_drive_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "team_drive_list_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDriveList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDrives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        pub team_drives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeamDrive>>>,
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
        #[doc = "The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ user_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "user_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#user\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "me")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this user is the requesting user."]
        pub me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's ID as visible in Permission resources."]
        pub permission_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the user's profile photo, if available."]
        pub photo_link: ::std::option::Option<::std::string::String>,
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
}
