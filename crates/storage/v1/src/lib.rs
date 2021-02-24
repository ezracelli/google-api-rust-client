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
    pub mod bucket_access_controls {
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod buckets {
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
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only deletes the bucket if its metageneration matches this value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only deletes the bucket if its metageneration does not match this value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit owner, acl and defaultObjectAcl properties."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod get_iam_policy {
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
                    #[serde(rename = "optionsRequestedPolicyVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The IAM policy format version to be returned. If the optionsRequestedPolicyVersion is for an older version that doesn't support part of the requested IAM policy, the request fails."]
                    pub options_requested_policy_version:
                        ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this bucket."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedDefaultObjectAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of default object access controls to this bucket."]
                    pub predefined_default_object_acl:
                        ::std::option::Option<QueryParametersPredefinedDefaultObjectAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "project")]
                    #[doc = "A valid API project identifier."]
                    pub project: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this bucket."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "private")]
                    #[doc = "Project team owners get OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
                    PublicRead,
                    #[serde(rename = "publicReadWrite")]
                    #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
                    PublicReadWrite,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of default object access controls to this bucket."]
                pub enum QueryParametersPredefinedDefaultObjectAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedDefaultObjectAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit owner, acl and defaultObjectAcl properties."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
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
                    #[doc = "Maximum number of buckets to return in a single response. The service will use this parameter or 1,000 items, whichever is smaller."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "prefix")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to buckets whose names begin with this prefix."]
                    pub prefix: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "project")]
                    #[doc = "A valid API project identifier."]
                    pub project: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit owner, acl and defaultObjectAcl properties."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod lock_retention_policy {
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
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[doc = "Makes the operation conditional on whether bucket's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this bucket."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedDefaultObjectAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of default object access controls to this bucket."]
                    pub predefined_default_object_acl:
                        ::std::option::Option<QueryParametersPredefinedDefaultObjectAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this bucket."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "private")]
                    #[doc = "Project team owners get OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
                    PublicRead,
                    #[serde(rename = "publicReadWrite")]
                    #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
                    PublicReadWrite,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of default object access controls to this bucket."]
                pub enum QueryParametersPredefinedDefaultObjectAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedDefaultObjectAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit owner, acl and defaultObjectAcl properties."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod set_iam_policy {
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod test_iam_permissions {
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
                    #[serde(rename = "permissions")]
                    #[doc = "Permissions to test."]
                    pub permissions: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this bucket."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedDefaultObjectAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of default object access controls to this bucket."]
                    pub predefined_default_object_acl:
                        ::std::option::Option<QueryParametersPredefinedDefaultObjectAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this bucket."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Project team owners get OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "private")]
                    #[doc = "Project team owners get OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Project team owners get OWNER access, and allUsers get READER access."]
                    PublicRead,
                    #[serde(rename = "publicReadWrite")]
                    #[doc = "Project team owners get OWNER access, and allUsers get WRITER access."]
                    PublicReadWrite,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of default object access controls to this bucket."]
                pub enum QueryParametersPredefinedDefaultObjectAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedDefaultObjectAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit owner, acl and defaultObjectAcl properties."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
        }
    }
    pub mod default_object_access_controls {
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, only return default ACL listing if the bucket's current metageneration matches this value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, only return default ACL listing if the bucket's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod notifications {
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod object_access_controls {
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod objects {
        pub mod methods {
            pub mod compose {
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
                    #[serde(rename = "destinationPredefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to the destination object."]
                    pub destination_predefined_acl:
                        ::std::option::Option<QueryParametersDestinationPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "kmsKeyName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
                    pub kms_key_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to the destination object."]
                pub enum QueryParametersDestinationPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersDestinationPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "destinationKmsKeyName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
                    pub destination_kms_key_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "destinationPredefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to the destination object."]
                    pub destination_predefined_acl:
                        ::std::option::Option<QueryParametersDestinationPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current generation matches the given value."]
                    pub if_source_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current generation does not match the given value."]
                    pub if_source_generation_not_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current metageneration matches the given value."]
                    pub if_source_metageneration_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current metageneration does not match the given value."]
                    pub if_source_metageneration_not_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sourceGeneration")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of the source object (as opposed to the latest version, the default)."]
                    pub source_generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to the destination object."]
                pub enum QueryParametersDestinationPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersDestinationPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod get_iam_policy {
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "contentEncoding")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to setting the contentEncoding metadata property. This can be useful when uploading an object with uploadType=media to indicate the encoding of the content being uploaded."]
                    pub content_encoding: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "kmsKeyName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
                    pub kms_key_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."]
                    pub name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this object."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this object."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
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
                    #[serde(rename = "delimiter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted."]
                    pub delimiter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endOffset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names are lexicographically before endOffset. If startOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive)."]
                    pub end_offset: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeTrailingDelimiter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes."]
                    pub include_trailing_delimiter: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "prefix")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names begin with this prefix."]
                    pub prefix: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startOffset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names are lexicographically equal to or after startOffset. If endOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive)."]
                    pub start_offset: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "versions")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning."]
                    pub versions: ::std::option::Option<::std::primitive::bool>,
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
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this object."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request, for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this object."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod rewrite {
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
                    #[serde(rename = "destinationKmsKeyName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any."]
                    pub destination_kms_key_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "destinationPredefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to the destination object."]
                    pub destination_predefined_acl:
                        ::std::option::Option<QueryParametersDestinationPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the destination object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current generation matches the given value."]
                    pub if_source_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current generation does not match the given value."]
                    pub if_source_generation_not_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current metageneration matches the given value."]
                    pub if_source_metageneration_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifSourceMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the source object's current metageneration does not match the given value."]
                    pub if_source_metageneration_not_match:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxBytesRewrittenPerCall")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of bytes that will be rewritten per rewrite request. Most callers shouldn't need to specify this parameter - it is primarily in place to support testing. If specified the value must be an integral multiple of 1 MiB (1048576). Also, this only applies to requests where the source and destination span locations and/or storage classes. Finally, this value must not change across rewrite calls else you'll get an error that the rewriteToken is invalid."]
                    pub max_bytes_rewritten_per_call: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "rewriteToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite response 'done' flag is true. Calls that provide a rewriteToken can omit all other request fields, but if included those fields must match the values provided in the first rewrite request."]
                    pub rewrite_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sourceGeneration")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of the source object (as opposed to the latest version, the default)."]
                    pub source_generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to the destination object."]
                pub enum QueryParametersDestinationPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersDestinationPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod set_iam_policy {
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod test_iam_permissions {
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "permissions")]
                    #[doc = "Permissions to test."]
                    pub permissions: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "generation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If present, selects a specific revision of this object (as opposed to the latest version, the default)."]
                    pub generation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object."]
                    pub if_generation_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifGenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object."]
                    pub if_generation_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration matches the given value."]
                    pub if_metageneration_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "ifMetagenerationNotMatch")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Makes the operation conditional on whether the object's current metageneration does not match the given value."]
                    pub if_metageneration_not_match: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "predefinedAcl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Apply a predefined set of access controls to this object."]
                    pub predefined_acl: ::std::option::Option<QueryParametersPredefinedAclEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to full."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Apply a predefined set of access controls to this object."]
                pub enum QueryParametersPredefinedAclEnum {
                    #[serde(rename = "authenticatedRead")]
                    #[doc = "Object owner gets OWNER access, and allAuthenticatedUsers get READER access."]
                    AuthenticatedRead,
                    #[serde(rename = "bucketOwnerFullControl")]
                    #[doc = "Object owner gets OWNER access, and project team owners get OWNER access."]
                    BucketOwnerFullControl,
                    #[serde(rename = "bucketOwnerRead")]
                    #[doc = "Object owner gets OWNER access, and project team owners get READER access."]
                    BucketOwnerRead,
                    #[serde(rename = "private")]
                    #[doc = "Object owner gets OWNER access."]
                    Private,
                    #[serde(rename = "projectPrivate")]
                    #[doc = "Object owner gets OWNER access, and project team members get access according to their roles."]
                    ProjectPrivate,
                    #[serde(rename = "publicRead")]
                    #[doc = "Object owner gets OWNER access, and allUsers get READER access."]
                    PublicRead,
                }
                impl ::std::default::Default for QueryParametersPredefinedAclEnum {
                    fn default() -> Self {
                        Self::AuthenticatedRead
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to full."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
            pub mod watch_all {
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
                    #[serde(rename = "delimiter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted."]
                    pub delimiter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endOffset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names are lexicographically before endOffset. If startOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive)."]
                    pub end_offset: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeTrailingDelimiter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes."]
                    pub include_trailing_delimiter: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "prefix")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names begin with this prefix."]
                    pub prefix: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set of properties to return. Defaults to noAcl."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "provisionalUserProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                    pub provisional_user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startOffset")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter results to objects whose names are lexicographically equal to or after startOffset. If endOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive)."]
                    pub start_offset: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProject")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The project to be billed for this request. Required for Requester Pays buckets."]
                    pub user_project: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "versions")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning."]
                    pub versions: ::std::option::Option<::std::primitive::bool>,
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
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Set of properties to return. Defaults to noAcl."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Include all properties."]
                    Full,
                    #[serde(rename = "noAcl")]
                    #[doc = "Omit the owner, acl property."]
                    NoAcl,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod resources {
            pub mod hmac_keys {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceAccountEmail")]
                            #[doc = "Email address of the service account."]
                            pub service_account_email: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
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
                            #[doc = "Maximum number of items to return in a single page of responses. The service uses this parameter or 250 items, whichever is smaller. The max number of items per page will also be limited by the number of distinct service accounts in the response. If the number of service accounts in a single response is too high, the page will truncated and a next page token will be returned."]
                            pub max_results: ::std::primitive::i64,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A previously-returned page token representing part of the larger set of results to view."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceAccountEmail")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If present, only keys for the given service account are returned."]
                            pub service_account_email: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "showDeletedKeys")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether or not to show keys in the DELETED state."]
                            pub show_deleted_keys: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn max_results() -> ::std::primitive::i64 {
                                serde_json::from_str(&"250").unwrap()
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod service_account {
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "provisionalUserProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request if the target bucket is requester-pays bucket."]
                            pub provisional_user_project:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userProject")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The project to be billed for this request."]
                            pub user_project: ::std::option::Option<::std::string::String>,
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
    #[doc = "A bucket."]
    pub struct Bucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Access controls on the bucket."]
        pub acl: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BucketAccessControl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's billing configuration."]
        pub billing: ::std::option::Option<BucketBilling>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's Cross-Origin Resource Sharing (CORS) configuration."]
        pub cors: ::std::option::Option<::std::vec::Vec<BucketCors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultEventBasedHold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed."]
        pub default_event_based_hold: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultObjectAcl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default access controls to apply to new objects when no ACL is provided."]
        pub default_object_acl:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encryption configuration for a bucket."]
        pub encryption: ::std::option::Option<BucketEncryption>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for the bucket."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iamConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's IAM configuration."]
        pub iam_configuration: ::std::option::Option<BucketIamConfiguration>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the bucket. For buckets, the id and name properties are the same."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ bucket_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "bucket_defaults :: kind")]
        #[doc = "The kind of item this is. For buckets, this is always storage#bucket."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided labels, in key/value pairs."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lifecycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's lifecycle configuration. See lifecycle management for more information."]
        pub lifecycle: ::std::option::Option<BucketLifecycle>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the bucket location."]
        pub location_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs."]
        pub logging: ::std::option::Option<BucketLogging>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metageneration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata generation of this bucket."]
        pub metageneration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the bucket."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The owner of the bucket. This is always the project team's owner group."]
        pub owner: ::std::option::Option<BucketOwner>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project number of the project the bucket belongs to."]
        pub project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error."]
        pub retention_policy: ::std::option::Option<BucketRetentionPolicy>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "satisfiesPZS")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this bucket."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes."]
        pub storage_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeCreated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the bucket in RFC 3339 format."]
        pub time_created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The modification time of the bucket in RFC 3339 format."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's versioning configuration."]
        pub versioning: ::std::option::Option<BucketVersioning>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "website")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information."]
        pub website: ::std::option::Option<BucketWebsite>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoneAffinity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zone or zones from which the bucket is intended to use zonal quota. Requests for data from outside the specified affinities are still allowed but won't be able to use zonal quota. The zone or zones need to be within the bucket location otherwise the requests will fail with a 400 Bad Request response."]
        pub zone_affinity: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Bucket {
        pub fn builder() -> BucketBuilder {
            BucketBuilder::default()
        }
    }
    mod bucket_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#bucket")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's billing configuration."]
    pub struct BucketBilling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requesterPays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When set to true, Requester Pays is enabled for this bucket."]
        pub requester_pays: ::std::option::Option<::std::primitive::bool>,
    }
    impl BucketBilling {
        pub fn builder() -> BucketBillingBuilder {
            BucketBillingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketCors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAgeSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses."]
        pub max_age_seconds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
        pub method: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "origin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
        pub origin: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
        pub response_header: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BucketCors {
        pub fn builder() -> BucketCorsBuilder {
            BucketCorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encryption configuration for a bucket."]
    pub struct BucketEncryption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultKmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified."]
        pub default_kms_key_name: ::std::option::Option<::std::string::String>,
    }
    impl BucketEncryption {
        pub fn builder() -> BucketEncryptionBuilder {
            BucketEncryptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's IAM configuration."]
    pub struct BucketIamConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketPolicyOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature."]
        pub bucket_policy_only: ::std::option::Option<BucketIamConfigurationBucketPolicyOnly>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicAccessPrevention")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's Public Access Prevention configuration. Currently, 'unspecified' and 'enforced' are supported."]
        pub public_access_prevention: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniformBucketLevelAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bucket's uniform bucket-level access configuration."]
        pub uniform_bucket_level_access:
            ::std::option::Option<BucketIamConfigurationUniformBucketLevelAccess>,
    }
    impl BucketIamConfiguration {
        pub fn builder() -> BucketIamConfigurationBuilder {
            BucketIamConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature."]
    pub struct BucketIamConfigurationBucketPolicyOnly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lockedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable."]
        pub locked_time: ::std::option::Option<::std::string::String>,
    }
    impl BucketIamConfigurationBucketPolicyOnly {
        pub fn builder() -> BucketIamConfigurationBucketPolicyOnlyBuilder {
            BucketIamConfigurationBucketPolicyOnlyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's uniform bucket-level access configuration."]
    pub struct BucketIamConfigurationUniformBucketLevelAccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, access is controlled only by bucket-level or above IAM policies."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lockedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable."]
        pub locked_time: ::std::option::Option<::std::string::String>,
    }
    impl BucketIamConfigurationUniformBucketLevelAccess {
        pub fn builder() -> BucketIamConfigurationUniformBucketLevelAccessBuilder {
            BucketIamConfigurationUniformBucketLevelAccessBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's lifecycle configuration. See lifecycle management for more information."]
    pub struct BucketLifecycle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken."]
        pub rule: ::std::option::Option<::std::vec::Vec<BucketLifecycleRule>>,
    }
    impl BucketLifecycle {
        pub fn builder() -> BucketLifecycleBuilder {
            BucketLifecycleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketLifecycleRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action to take."]
        pub action: ::std::option::Option<BucketLifecycleRuleAction>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition(s) under which the action will be taken."]
        pub condition: ::std::option::Option<BucketLifecycleRuleCondition>,
    }
    impl BucketLifecycleRule {
        pub fn builder() -> BucketLifecycleRuleBuilder {
            BucketLifecycleRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The action to take."]
    pub struct BucketLifecycleRuleAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target storage class. Required iff the type of the action is SetStorageClass."]
        pub storage_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the action. Currently, only Delete and SetStorageClass are supported."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl BucketLifecycleRuleAction {
        pub fn builder() -> BucketLifecycleRuleActionBuilder {
            BucketLifecycleRuleActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The condition(s) under which the action will be taken."]
    pub struct BucketLifecycleRuleCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "age")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Age of an object (in days). This condition is satisfied when an object reaches the specified age."]
        pub age: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when an object is created before midnight of the specified date in UTC."]
        pub created_before: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTimeBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when the custom time on an object is before this date in UTC."]
        pub custom_time_before: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysSinceCustomTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days elapsed since the user-specified timestamp set on an object. The condition is satisfied if the days elapsed is at least this number. If no custom timestamp is specified on an object, the condition does not apply."]
        pub days_since_custom_time: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysSinceNoncurrentTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days elapsed since the noncurrent timestamp of an object. The condition is satisfied if the days elapsed is at least this number. This condition is relevant only for versioned objects. The value of the field must be a nonnegative integer. If it's zero, the object version will become eligible for Lifecycle action as soon as it becomes noncurrent."]
        pub days_since_noncurrent_time: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isLive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relevant only for versioned objects. If the value is true, this condition matches live objects; if the value is false, it matches archived objects."]
        pub is_live: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchesPattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A regular expression that satisfies the RE2 syntax. This condition is satisfied when the name of the object matches the RE2 pattern. Note: This feature is currently in the \"Early Access\" launch stage and is only available to a whitelisted set of users; that means that this feature may be changed in backward-incompatible ways and that it is not guaranteed to be released."]
        pub matches_pattern: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchesStorageClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, and DURABLE_REDUCED_AVAILABILITY."]
        pub matches_storage_class: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noncurrentTimeBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A date in RFC 3339 format with only the date part (for instance, \"2013-01-15\"). This condition is satisfied when the noncurrent time on an object is before this date in UTC. This condition is relevant only for versioned objects."]
        pub noncurrent_time_before: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numNewerVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object."]
        pub num_newer_versions: ::std::option::Option<::std::primitive::i64>,
    }
    impl BucketLifecycleRuleCondition {
        pub fn builder() -> BucketLifecycleRuleConditionBuilder {
            BucketLifecycleRuleConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs."]
    pub struct BucketLogging {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination bucket where the current bucket's logs should be placed."]
        pub log_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logObjectPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A prefix for log object names."]
        pub log_object_prefix: ::std::option::Option<::std::string::String>,
    }
    impl BucketLogging {
        pub fn builder() -> BucketLoggingBuilder {
            BucketLoggingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The owner of the bucket. This is always the project team's owner group."]
    pub struct BucketOwner {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity, in the form project-owner-projectId."]
        pub entity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID for the entity."]
        pub entity_id: ::std::option::Option<::std::string::String>,
    }
    impl BucketOwner {
        pub fn builder() -> BucketOwnerBuilder {
            BucketOwnerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error."]
    pub struct BucketRetentionPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format."]
        pub effective_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isLocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Once locked, an object retention policy cannot be modified."]
        pub is_locked: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes."]
        pub retention_period: ::std::option::Option<::std::string::String>,
    }
    impl BucketRetentionPolicy {
        pub fn builder() -> BucketRetentionPolicyBuilder {
            BucketRetentionPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's versioning configuration."]
    pub struct BucketVersioning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "While set to true, versioning is fully enabled for this bucket."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl BucketVersioning {
        pub fn builder() -> BucketVersioningBuilder {
            BucketVersioningBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information."]
    pub struct BucketWebsite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mainPageSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the requested object path is missing, the service will ensure the path has a trailing '/', append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages."]
        pub main_page_suffix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notFoundPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result."]
        pub not_found_page: ::std::option::Option<::std::string::String>,
    }
    impl BucketWebsite {
        pub fn builder() -> BucketWebsiteBuilder {
            BucketWebsiteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An access-control entry."]
    pub struct BucketAccessControl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the bucket."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain associated with the entity, if any."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address associated with the entity, if any."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
        pub entity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID for the entity, if any."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the access-control entry."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ bucket_access_control_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "bucket_access_control_defaults :: kind")]
        #[doc = "The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectTeam")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project team associated with the entity, if any."]
        pub project_team: ::std::option::Option<BucketAccessControlProjectTeam>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The access permission for the entity."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to this access-control entry."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl BucketAccessControl {
        pub fn builder() -> BucketAccessControlBuilder {
            BucketAccessControlBuilder::default()
        }
    }
    mod bucket_access_control_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#bucketAccessControl")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The project team associated with the entity, if any."]
    pub struct BucketAccessControlProjectTeam {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project number."]
        pub project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "team")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The team."]
        pub team: ::std::option::Option<::std::string::String>,
    }
    impl BucketAccessControlProjectTeam {
        pub fn builder() -> BucketAccessControlProjectTeamBuilder {
            BucketAccessControlProjectTeamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An access-control list."]
    pub struct BucketAccessControls {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BucketAccessControl>>>,
        #[builder(
            default = "{ bucket_access_controls_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "bucket_access_controls_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls."]
        pub kind: ::std::string::String,
    }
    impl BucketAccessControls {
        pub fn builder() -> BucketAccessControlsBuilder {
            BucketAccessControlsBuilder::default()
        }
    }
    mod bucket_access_controls_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#bucketAccessControls")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of buckets."]
    pub struct Buckets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bucket>>>,
        #[builder(default = "{ buckets_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "buckets_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of buckets, this is always storage#buckets."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Buckets {
        pub fn builder() -> BucketsBuilder {
            BucketsBuilder::default()
        }
    }
    mod buckets_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#buckets")
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
    #[doc = "A Compose request."]
    pub struct ComposeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of the resulting object."]
        pub destination: ::std::option::Option<::std::boxed::Box<Object>>,
        #[builder(default = "{ compose_request_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "compose_request_defaults :: kind")]
        #[doc = "The kind of item this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of source objects that will be concatenated into a single object."]
        pub source_objects: ::std::option::Option<::std::vec::Vec<ComposeRequestSourceObjects>>,
    }
    impl ComposeRequest {
        pub fn builder() -> ComposeRequestBuilder {
            ComposeRequestBuilder::default()
        }
    }
    mod compose_request_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#composeRequest")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ComposeRequestSourceObjects {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generation of this object to use as the source."]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source object's name. All source objects must reside in the same bucket."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectPreconditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conditions that must be met for this operation to execute."]
        pub object_preconditions:
            ::std::option::Option<ComposeRequestSourceObjectsObjectPreconditions>,
    }
    impl ComposeRequestSourceObjects {
        pub fn builder() -> ComposeRequestSourceObjectsBuilder {
            ComposeRequestSourceObjectsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Conditions that must be met for this operation to execute."]
    pub struct ComposeRequestSourceObjectsObjectPreconditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ifGenerationMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail."]
        pub if_generation_match: ::std::option::Option<::std::string::String>,
    }
    impl ComposeRequestSourceObjectsObjectPreconditions {
        pub fn builder() -> ComposeRequestSourceObjectsObjectPreconditionsBuilder {
            ComposeRequestSourceObjectsObjectPreconditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an expression text. Example: title: \"User account presence\" description: \"Determines whether the request has a user account\" expression: \"size(request.user) > 0\""]
    pub struct Expr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Expr {
        pub fn builder() -> ExprBuilder {
            ExprBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template to produce a JSON-style HMAC Key resource for Create responses."]
    pub struct HmacKey {
        #[builder(default = "{ hmac_key_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "hmac_key_defaults :: kind")]
        #[doc = "The kind of item this is. For HMAC keys, this is always storage#hmacKey."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key metadata."]
        pub metadata: ::std::option::Option<::std::boxed::Box<HmacKeyMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HMAC secret key material."]
        pub secret: ::std::option::Option<::std::string::String>,
    }
    impl HmacKey {
        pub fn builder() -> HmacKeyBuilder {
            HmacKeyBuilder::default()
        }
    }
    mod hmac_key_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#hmacKey")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template to produce a JSON-style HMAC Key metadata resource."]
    pub struct HmacKeyMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the HMAC Key."]
        pub access_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for the HMAC key."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the HMAC key, including the Project ID and the Access ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ hmac_key_metadata_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "hmac_key_metadata_defaults :: kind")]
        #[doc = "The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID owning the service account to which the key authenticates."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the key's associated service account."]
        pub service_account_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeCreated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the HMAC key in RFC 3339 format."]
        pub time_created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last modification time of the HMAC key metadata in RFC 3339 format."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl HmacKeyMetadata {
        pub fn builder() -> HmacKeyMetadataBuilder {
            HmacKeyMetadataBuilder::default()
        }
    }
    mod hmac_key_metadata_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#hmacKeyMetadata")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of hmacKeys."]
    pub struct HmacKeysMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HmacKeyMetadata>>>,
        #[builder(default = "{ hmac_keys_metadata_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "hmac_keys_metadata_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of hmacKeys, this is always storage#hmacKeysMetadata."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl HmacKeysMetadata {
        pub fn builder() -> HmacKeysMetadataBuilder {
            HmacKeysMetadataBuilder::default()
        }
    }
    mod hmac_keys_metadata_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#hmacKeysMetadata")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A subscription to receive Google PubSub notifications."]
    pub struct Notification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "custom_attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription."]
        pub custom_attributes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for this subscription notification."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event_types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, only send notifications about listed event types. If empty, sent notifications for all event types."]
        pub event_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the notification."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ notification_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "notification_defaults :: kind")]
        #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "object_name_prefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, only apply this notification configuration to object names that begin with this prefix."]
        pub object_name_prefix: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ notification_defaults :: payload_format () }",
            setter(into)
        )]
        #[serde(rename = "payload_format")]
        #[serde(default = "notification_defaults :: payload_format")]
        #[doc = "The desired content of the Payload."]
        pub payload_format: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical URL of this notification."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'"]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl Notification {
        pub fn builder() -> NotificationBuilder {
            NotificationBuilder::default()
        }
    }
    mod notification_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#notification")
        }
        pub fn payload_format() -> ::std::string::String {
            String::from("JSON_API_V1")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of notification subscriptions."]
    pub struct Notifications {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Notification>>>,
        #[builder(default = "{ notifications_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "notifications_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of notifications, this is always storage#notifications."]
        pub kind: ::std::string::String,
    }
    impl Notifications {
        pub fn builder() -> NotificationsBuilder {
            NotificationsBuilder::default()
        }
    }
    mod notifications_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#notifications")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object."]
    pub struct Object {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Access controls on the object."]
        pub acl: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the bucket containing this object."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600."]
        pub cache_control: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of underlying components that make up this object. Components are accumulated by compose operations."]
        pub component_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content-Disposition of the object data."]
        pub content_disposition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content-Encoding of the object data."]
        pub content_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content-Language of the object data."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices."]
        pub crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A timestamp in RFC 3339 format specified by the user for an object."]
        pub custom_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerEncryption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata of customer-supplied encryption key, if the object is encrypted by such a key."]
        pub customer_encryption: ::std::option::Option<ObjectCustomerEncryption>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for the object."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventBasedHold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false."]
        pub event_based_hold: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content generation of this object. Used for object versioning."]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the object, including the bucket name, object name, and generation number."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ object_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "object_defaults :: kind")]
        #[doc = "The kind of item this is. For objects, this is always storage#object."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not currently supported. Specifying the parameter causes the request to fail with status code 400 - Bad Request."]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices."]
        pub md5_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Media download link."]
        pub media_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata, in key/value pairs."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metageneration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object."]
        pub metageneration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the object. Required if not specified by URL parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The owner of the object. This will always be the uploader of the object."]
        pub owner: ::std::option::Option<ObjectOwner>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionExpirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold)."]
        pub retention_expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to this object."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content-Length of the data in bytes."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Storage class of the object."]
        pub storage_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "temporaryHold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object."]
        pub temporary_hold: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeCreated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the object in RFC 3339 format."]
        pub time_created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeDeleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted."]
        pub time_deleted: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeStorageClassUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated."]
        pub time_storage_class_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The modification time of the object metadata in RFC 3339 format."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Object {
        pub fn builder() -> ObjectBuilder {
            ObjectBuilder::default()
        }
    }
    mod object_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#object")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of customer-supplied encryption key, if the object is encrypted by such a key."]
    pub struct ObjectCustomerEncryption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionAlgorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encryption algorithm."]
        pub encryption_algorithm: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keySha256")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SHA256 hash value of the encryption key."]
        pub key_sha256: ::std::option::Option<::std::string::String>,
    }
    impl ObjectCustomerEncryption {
        pub fn builder() -> ObjectCustomerEncryptionBuilder {
            ObjectCustomerEncryptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The owner of the object. This will always be the uploader of the object."]
    pub struct ObjectOwner {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity, in the form user-userId."]
        pub entity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID for the entity."]
        pub entity_id: ::std::option::Option<::std::string::String>,
    }
    impl ObjectOwner {
        pub fn builder() -> ObjectOwnerBuilder {
            ObjectOwnerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An access-control entry."]
    pub struct ObjectAccessControl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the bucket."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain associated with the entity, if any."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address associated with the entity, if any."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity holding the permission, in one of the following forms: \n- user-userId \n- user-email \n- group-groupId \n- group-email \n- domain-domain \n- project-team-projectId \n- allUsers \n- allAuthenticatedUsers Examples: \n- The user liz@example.com would be user-liz@example.com. \n- The group example@googlegroups.com would be group-example@googlegroups.com. \n- To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com."]
        pub entity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID for the entity, if any."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1 Entity tag for the access-control entry."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content generation of the object, if applied to an object."]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the access-control entry."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ object_access_control_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "object_access_control_defaults :: kind")]
        #[doc = "The kind of item this is. For object access control entries, this is always storage#objectAccessControl."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "object")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the object, if applied to an object."]
        pub object: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectTeam")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project team associated with the entity, if any."]
        pub project_team: ::std::option::Option<ObjectAccessControlProjectTeam>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The access permission for the entity."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to this access-control entry."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ObjectAccessControl {
        pub fn builder() -> ObjectAccessControlBuilder {
            ObjectAccessControlBuilder::default()
        }
    }
    mod object_access_control_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#objectAccessControl")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The project team associated with the entity, if any."]
    pub struct ObjectAccessControlProjectTeam {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project number."]
        pub project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "team")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The team."]
        pub team: ::std::option::Option<::std::string::String>,
    }
    impl ObjectAccessControlProjectTeam {
        pub fn builder() -> ObjectAccessControlProjectTeamBuilder {
            ObjectAccessControlProjectTeamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An access-control list."]
    pub struct ObjectAccessControls {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAccessControl>>>,
        #[builder(
            default = "{ object_access_controls_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "object_access_controls_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls."]
        pub kind: ::std::string::String,
    }
    impl ObjectAccessControls {
        pub fn builder() -> ObjectAccessControlsBuilder {
            ObjectAccessControlsBuilder::default()
        }
    }
    mod object_access_controls_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#objectAccessControls")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of objects."]
    pub struct Objects {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Object>>>,
        #[builder(default = "{ objects_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "objects_defaults :: kind")]
        #[doc = "The kind of item this is. For lists of objects, this is always storage#objects."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prefixes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter."]
        pub prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Objects {
        pub fn builder() -> ObjectsBuilder {
            ObjectsBuilder::default()
        }
    }
    mod objects_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#objects")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bucket/object IAM policy."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An association between a role, which comes with a set of permissions, and members who may assume that role."]
        pub bindings: ::std::option::Option<::std::vec::Vec<PolicyBindings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP 1.1  Entity tag for the policy."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ policy_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "policy_defaults :: kind")]
        #[doc = "The kind of item this is. For policies, this is always storage#policy. This field is ignored on input."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the resource to which this policy belongs. Will be of the form projects/_/buckets/bucket for buckets, and projects/_/buckets/bucket/objects/object for objects. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IAM policy format version."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    mod policy_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#policy")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PolicyBindings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. NOTE: an unsatisfied condition will not allow user access via current binding. Different bindings, including their conditions, are examined independently."]
        pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of identifiers for members who may assume the provided role. Recognized identifiers are as follows:  \n- allUsers — A special identifier that represents anyone on the internet; with or without a Google account.  \n- allAuthenticatedUsers — A special identifier that represents anyone who is authenticated with a Google account or a service account.  \n- user:emailid — An email address that represents a specific account. For example, user:alice@gmail.com or user:joe@example.com.  \n- serviceAccount:emailid — An email address that represents a service account. For example,  serviceAccount:my-other-app@appspot.gserviceaccount.com .  \n- group:emailid — An email address that represents a Google group. For example, group:admins@example.com.  \n- domain:domain — A Google Apps domain name that represents all the users of that domain. For example, domain:google.com or domain:example.com.  \n- projectOwner:projectid — Owners of the given project. For example, projectOwner:my-example-project  \n- projectEditor:projectid — Editors of the given project. For example, projectEditor:my-example-project  \n- projectViewer:projectid — Viewers of the given project. For example, projectViewer:my-example-project"]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role to which members belong. Two types of roles are supported: new IAM roles, which grant permissions that do not map directly to those provided by ACLs, and legacy IAM roles, which do map directly to ACL permissions. All roles are of the format roles/storage.specificRole.\nThe new IAM roles are:  \n- roles/storage.admin — Full control of Google Cloud Storage resources.  \n- roles/storage.objectViewer — Read-Only access to Google Cloud Storage objects.  \n- roles/storage.objectCreator — Access to create objects in Google Cloud Storage.  \n- roles/storage.objectAdmin — Full control of Google Cloud Storage objects.   The legacy IAM roles are:  \n- roles/storage.legacyObjectReader — Read-only access to objects without listing. Equivalent to an ACL entry on an object with the READER role.  \n- roles/storage.legacyObjectOwner — Read/write access to existing objects without listing. Equivalent to an ACL entry on an object with the OWNER role.  \n- roles/storage.legacyBucketReader — Read access to buckets with object listing. Equivalent to an ACL entry on a bucket with the READER role.  \n- roles/storage.legacyBucketWriter — Read access to buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the WRITER role.  \n- roles/storage.legacyBucketOwner — Read and write access to existing buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the OWNER role."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl PolicyBindings {
        pub fn builder() -> PolicyBindingsBuilder {
            PolicyBindingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rewrite response."]
    pub struct RewriteResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ rewrite_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "rewrite_response_defaults :: kind")]
        #[doc = "The kind of item this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total size of the object being copied in bytes. This property is always present in the response."]
        pub object_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes."]
        pub resource: ::std::option::Option<::std::boxed::Box<Object>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rewriteToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy."]
        pub rewrite_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesRewritten")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response."]
        pub total_bytes_rewritten: ::std::option::Option<::std::string::String>,
    }
    impl RewriteResponse {
        pub fn builder() -> RewriteResponseBuilder {
            RewriteResponseBuilder::default()
        }
    }
    mod rewrite_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#rewriteResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A subscription to receive Google PubSub notifications."]
    pub struct ServiceAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email_address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the notification."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ service_account_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "service_account_defaults :: kind")]
        #[doc = "The kind of item this is. For notifications, this is always storage#notification."]
        pub kind: ::std::string::String,
    }
    impl ServiceAccount {
        pub fn builder() -> ServiceAccountBuilder {
            ServiceAccountBuilder::default()
        }
    }
    mod service_account_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#serviceAccount")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A storage.(buckets|objects).testIamPermissions response."]
    pub struct TestIamPermissionsResponse {
        #[builder(
            default = "{ test_iam_permissions_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "test_iam_permissions_response_defaults :: kind")]
        #[doc = "The kind of item this is."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets or objects. The supported permissions are as follows:  \n- storage.buckets.delete — Delete bucket.  \n- storage.buckets.get — Read bucket metadata.  \n- storage.buckets.getIamPolicy — Read bucket IAM policy.  \n- storage.buckets.create — Create bucket.  \n- storage.buckets.list — List buckets.  \n- storage.buckets.setIamPolicy — Update bucket IAM policy.  \n- storage.buckets.update — Update bucket metadata.  \n- storage.objects.delete — Delete object.  \n- storage.objects.get — Read object data and metadata.  \n- storage.objects.getIamPolicy — Read object IAM policy.  \n- storage.objects.create — Create object.  \n- storage.objects.list — List objects.  \n- storage.objects.setIamPolicy — Update object IAM policy.  \n- storage.objects.update — Update object metadata."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestIamPermissionsResponse {
        pub fn builder() -> TestIamPermissionsResponseBuilder {
            TestIamPermissionsResponseBuilder::default()
        }
    }
    mod test_iam_permissions_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("storage#testIamPermissionsResponse")
        }
    }
}
