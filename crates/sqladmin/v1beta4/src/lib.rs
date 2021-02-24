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
    pub mod backup_runs {
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
                    #[doc = "Maximum number of backup runs per response."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
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
    pub mod flags {
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
                    #[serde(rename = "databaseVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Database type and version you want to retrieve flags for. By default, this method returns flags for all database types and versions."]
                    pub database_version: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod instances {
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
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression is in the form of field:value. For example, 'instanceType:CLOUD_SQL_INSTANCE'. Fields can be nested as needed as per their JSON representation, such as 'settings.userLabels.auto_start:true'. Multiple filter queries are space-separated. For example. 'state:RUNNABLE instanceType:CLOUD_SQL_INSTANCE'. By default, each expression is an AND expression. However, you can include AND and OR expressions explicitly."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return per response."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
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
    pub mod operations {
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
                    #[serde(rename = "instance")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Cloud SQL instance ID. This does not include the project ID."]
                    pub instance: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of operations per response."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A previously-returned page token representing part of the larger set of results to view."]
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
    pub mod projects {
        pub mod resources {
            pub mod instances {
                pub mod methods {
                    pub mod start_external_sync {
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
                            #[serde(rename = "skipVerification")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether to skip the verification step (VESS)."]
                            pub skip_verification: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "syncMode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "External sync mode."]
                            pub sync_mode: ::std::option::Option<QueryParametersSyncModeEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "External sync mode."]
                        pub enum QueryParametersSyncModeEnum {
                            #[serde(rename = "EXTERNAL_SYNC_MODE_UNSPECIFIED")]
                            #[doc = "Unknown external sync mode, will be defaulted to ONLINE mode"]
                            ExternalSyncModeUnspecified,
                            #[serde(rename = "ONLINE")]
                            #[doc = "Online external sync will set up replication after initial data external sync"]
                            Online,
                            #[serde(rename = "OFFLINE")]
                            #[doc = "Offline external sync only dumps and loads a one-time snapshot of the primary instance's data"]
                            Offline,
                        }
                        impl ::std::default::Default for QueryParametersSyncModeEnum {
                            fn default() -> Self {
                                Self::ExternalSyncModeUnspecified
                            }
                        }
                    }
                    pub mod verify_external_sync_settings {
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
                            #[serde(rename = "syncMode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "External sync mode"]
                            pub sync_mode: ::std::option::Option<QueryParametersSyncModeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "verifyConnectionOnly")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Flag to enable verifying connection only"]
                            pub verify_connection_only:
                                ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "External sync mode"]
                        pub enum QueryParametersSyncModeEnum {
                            #[serde(rename = "EXTERNAL_SYNC_MODE_UNSPECIFIED")]
                            #[doc = "Unknown external sync mode, will be defaulted to ONLINE mode"]
                            ExternalSyncModeUnspecified,
                            #[serde(rename = "ONLINE")]
                            #[doc = "Online external sync will set up replication after initial data external sync"]
                            Online,
                            #[serde(rename = "OFFLINE")]
                            #[doc = "Offline external sync only dumps and loads a one-time snapshot of the primary instance's data"]
                            Offline,
                        }
                        impl ::std::default::Default for QueryParametersSyncModeEnum {
                            fn default() -> Self {
                                Self::ExternalSyncModeUnspecified
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod users {
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
                    #[serde(rename = "host")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Host of the user in the instance."]
                    pub host: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Name of the user in the instance."]
                    pub name: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "host")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Host of the user in the instance."]
                    pub host: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Name of the user in the instance."]
                    pub name: ::std::option::Option<::std::string::String>,
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
    #[doc = "An entry for an Access Control list."]
    pub struct AclEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this access control entry expires in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#aclEntry*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A label to identify this entry."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The allowlisted value for the access control list."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl AclEntry {
        pub fn builder() -> AclEntryBuilder {
            AclEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Admin API warning message."]
    pub struct ApiWarning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code to uniquely identify the warning type."]
        pub code: ::std::option::Option<ApiWarningCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The warning message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The region name for REGION_UNREACHABLE warning."]
        pub region: ::std::option::Option<::std::string::String>,
    }
    impl ApiWarning {
        pub fn builder() -> ApiWarningBuilder {
            ApiWarningBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Code to uniquely identify the warning type."]
    pub enum ApiWarningCodeEnum {
        #[serde(rename = "SQL_API_WARNING_CODE_UNSPECIFIED")]
        #[doc = "An unknown or unset warning type from Cloud SQL API."]
        SqlApiWarningCodeUnspecified,
        #[serde(rename = "REGION_UNREACHABLE")]
        #[doc = "Warning when one or more regions are not reachable. The returned result set may be incomplete."]
        RegionUnreachable,
    }
    impl ::std::default::Default for ApiWarningCodeEnum {
        fn default() -> Self {
            Self::SqlApiWarningCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance backup configuration."]
    pub struct BackupConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupRetentionSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Backup retention settings."]
        pub backup_retention_settings:
            ::std::option::Option<::std::boxed::Box<BackupRetentionSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryLogEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(MySQL only) Whether binary log is enabled. If backup configuration is disabled, binarylog must be disabled as well."]
        pub binary_log_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this configuration is enabled."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#backupConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the backup"]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointInTimeRecoveryEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub point_in_time_recovery_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicationLogArchivingEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub replication_log_archiving_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time for the daily backup configuration in UTC timezone in the 24 hour format - *HH:MM*."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionLogRetentionDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of days of transaction logs we retain for point in time restore, from 1-7."]
        pub transaction_log_retention_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl BackupConfiguration {
        pub fn builder() -> BackupConfigurationBuilder {
            BackupConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Backup context."]
    pub struct BackupContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the backup."]
        pub backup_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#backupContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl BackupContext {
        pub fn builder() -> BackupContextBuilder {
            BackupContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "We currently only support backup retention by specifying the number of backups we will retain."]
    pub struct BackupRetentionSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retainedBackups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Depending on the value of retention_unit, this is used to determine if a backup needs to be deleted. If retention_unit is 'COUNT', we will retain this many backups."]
        pub retained_backups: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit that 'retained_backups' represents."]
        pub retention_unit: ::std::option::Option<BackupRetentionSettingsRetentionUnitEnum>,
    }
    impl BackupRetentionSettings {
        pub fn builder() -> BackupRetentionSettingsBuilder {
            BackupRetentionSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The unit that 'retained_backups' represents."]
    pub enum BackupRetentionSettingsRetentionUnitEnum {
        #[serde(rename = "RETENTION_UNIT_UNSPECIFIED")]
        #[doc = "Backup retention unit is unspecified, will be treated as COUNT."]
        RetentionUnitUnspecified,
        #[serde(rename = "COUNT")]
        #[doc = "Retention will be by count, eg. \"retain the most recent 7 backups\"."]
        Count,
    }
    impl ::std::default::Default for BackupRetentionSettingsRetentionUnitEnum {
        fn default() -> Self {
            Self::RetentionUnitUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A BackupRun resource."]
    pub struct BackupRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT."]
        pub backup_kind: ::std::option::Option<BackupRunBackupKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this run, only applicable to on-demand backups."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encryption configuration specific to a backup. Applies only to Second Generation instances."]
        pub disk_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<DiskEncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskEncryptionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encryption status specific to a backup. Applies only to Second Generation instances."]
        pub disk_encryption_status: ::std::option::Option<::std::boxed::Box<DiskEncryptionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the backup operation completed in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enqueuedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the run was enqueued in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub enqueued_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about why the backup operation failed. This is only present if the run has the FAILED status."]
        pub error: ::std::option::Option<::std::boxed::Box<OperationError>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this backup run. Unique only for a specific Cloud SQL instance."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the database instance."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#backupRun*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the backups."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the backup operation actually started in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of this run."]
        pub status: ::std::option::Option<BackupRunStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this run; can be either \"AUTOMATED\" or \"ON_DEMAND\". This field defaults to \"ON_DEMAND\" and is ignored, when specified for insert requests."]
        pub _type: ::std::option::Option<BackupRunTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of the backup window during which this the backup was attempted in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub window_start_time: ::std::option::Option<::std::string::String>,
    }
    impl BackupRun {
        pub fn builder() -> BackupRunBuilder {
            BackupRunBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT."]
    pub enum BackupRunBackupKindEnum {
        #[serde(rename = "SQL_BACKUP_KIND_UNSPECIFIED")]
        #[doc = "This is an unknown BackupKind."]
        SqlBackupKindUnspecified,
        #[serde(rename = "SNAPSHOT")]
        #[doc = "The snapshot based backups"]
        Snapshot,
        #[serde(rename = "PHYSICAL")]
        #[doc = "Physical backups"]
        Physical,
    }
    impl ::std::default::Default for BackupRunBackupKindEnum {
        fn default() -> Self {
            Self::SqlBackupKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of this run."]
    pub enum BackupRunStatusEnum {
        #[serde(rename = "SQL_BACKUP_RUN_STATUS_UNSPECIFIED")]
        #[doc = "The status of the run is unknown."]
        SqlBackupRunStatusUnspecified,
        #[serde(rename = "ENQUEUED")]
        #[doc = "The backup operation was enqueued."]
        Enqueued,
        #[serde(rename = "OVERDUE")]
        #[doc = "The backup is overdue across a given backup window. Indicates a problem. Example: Long-running operation in progress during the whole window."]
        Overdue,
        #[serde(rename = "RUNNING")]
        #[doc = "The backup is in progress."]
        Running,
        #[serde(rename = "FAILED")]
        #[doc = "The backup failed."]
        Failed,
        #[serde(rename = "SUCCESSFUL")]
        #[doc = "The backup was successful."]
        Successful,
        #[serde(rename = "SKIPPED")]
        #[doc = "The backup was skipped (without problems) for a given backup window. Example: Instance was idle."]
        Skipped,
        #[serde(rename = "DELETION_PENDING")]
        #[doc = "The backup is about to be deleted."]
        DeletionPending,
        #[serde(rename = "DELETION_FAILED")]
        #[doc = "The backup deletion failed."]
        DeletionFailed,
        #[serde(rename = "DELETED")]
        #[doc = "The backup has been deleted."]
        Deleted,
    }
    impl ::std::default::Default for BackupRunStatusEnum {
        fn default() -> Self {
            Self::SqlBackupRunStatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this run; can be either \"AUTOMATED\" or \"ON_DEMAND\". This field defaults to \"ON_DEMAND\" and is ignored, when specified for insert requests."]
    pub enum BackupRunTypeEnum {
        #[serde(rename = "SQL_BACKUP_RUN_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown BackupRun type."]
        SqlBackupRunTypeUnspecified,
        #[serde(rename = "AUTOMATED")]
        #[doc = "The backup schedule automatically triggers a backup."]
        Automated,
        #[serde(rename = "ON_DEMAND")]
        #[doc = "The user manually triggers a backup."]
        OnDemand,
    }
    impl ::std::default::Default for BackupRunTypeEnum {
        fn default() -> Self {
            Self::SqlBackupRunTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Backup run list results."]
    pub struct BackupRunsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of backup runs in reverse chronological order of the enqueued time."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BackupRun>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#backupRunsList*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl BackupRunsListResponse {
        pub fn builder() -> BackupRunsListResponseBuilder {
            BackupRunsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Binary log coordinates."]
    pub struct BinLogCoordinates {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binLogFileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the binary log file for a Cloud SQL instance."]
        pub bin_log_file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binLogPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position (offset) within the binary log file."]
        pub bin_log_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#binLogCoordinates*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl BinLogCoordinates {
        pub fn builder() -> BinLogCoordinatesBuilder {
            BinLogCoordinatesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance clone context."]
    pub struct CloneContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binLogCoordinates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Binary log coordinates, if specified, identify the position up to which the source instance is cloned. If not specified, the source instance is cloned up to the most recent binary log coordinates."]
        pub bin_log_coordinates: ::std::option::Option<::std::boxed::Box<BinLogCoordinates>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationInstanceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the Cloud SQL instance to be created as a clone."]
        pub destination_instance_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#cloneContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pitrTimestampMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub pitr_timestamp_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointInTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub point_in_time: ::std::option::Option<::std::string::String>,
    }
    impl CloneContext {
        pub fn builder() -> CloneContextBuilder {
            CloneContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a SQL database on the Cloud SQL instance."]
    pub struct Database {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "charset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud SQL charset value."]
        pub charset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud SQL collation value."]
        pub collation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated and will be removed from a future version of the API."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Cloud SQL instance. This does not include the project ID."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#database*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the database in the Cloud SQL instance. This does not include the project ID or instance name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "project")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable."]
        pub project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sqlserverDatabaseDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sqlserver_database_details:
            ::std::option::Option<::std::boxed::Box<SqlServerDatabaseDetails>>,
    }
    impl Database {
        pub fn builder() -> DatabaseBuilder {
            DatabaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database flags for Cloud SQL instances."]
    pub struct DatabaseFlags {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the flag. These flags are passed at instance startup, so include both server options and system variables for MySQL. Flags are specified with underscores, not hyphens. For more information, see Configuring Database Flags in the Cloud SQL documentation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the flag. Booleans are set to *on* for true and *off* for false. This field must be omitted if the flag doesn't take a value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DatabaseFlags {
        pub fn builder() -> DatabaseFlagsBuilder {
            DatabaseFlagsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Cloud SQL instance resource. Next field: 36"]
    pub struct DatabaseInstance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backendType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " *SECOND_GEN*: Cloud SQL database instance. *EXTERNAL*: A database server that is not managed by Google. This property is read-only; use the *tier* property in the *settings* object to determine the database type."]
        pub backend_type: ::std::option::Option<DatabaseInstanceBackendTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Connection name of the Cloud SQL instance used in connection strings."]
        pub connection_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentDiskSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current disk usage of the instance in bytes. This property has been deprecated. Use the \"cloudsql.googleapis.com/database/disk/bytes_used\" metric in Cloud Monitoring API instead. Please see this announcement for details."]
        pub current_disk_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The database engine type and version. The *databaseVersion* field cannot be changed after instance creation. MySQL instances: *MYSQL_8_0*, *MYSQL_5_7* (default), or *MYSQL_5_6*. PostgreSQL instances: *POSTGRES_9_6*, *POSTGRES_10*, *POSTGRES_11* or *POSTGRES_12* (default). SQL Server instances: *SQLSERVER_2017_STANDARD* (default), *SQLSERVER_2017_ENTERPRISE*, *SQLSERVER_2017_EXPRESS*, or *SQLSERVER_2017_WEB*."]
        pub database_version: ::std::option::Option<DatabaseInstanceDatabaseVersionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disk encryption configuration specific to an instance. Applies only to Second Generation instances."]
        pub disk_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<DiskEncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskEncryptionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disk encryption status specific to an instance. Applies only to Second Generation instances."]
        pub disk_encryption_status: ::std::option::Option<::std::boxed::Box<DiskEncryptionStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated and will be removed from a future version of the API. Use the *settings.settingsVersion* field instead."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failoverReplica")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name and status of the failover replica. This property is applicable only to Second Generation instances."]
        pub failover_replica: ::std::option::Option<DatabaseInstanceFailoverReplica>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gceZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone."]
        pub gce_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instance type. This can be one of the following. *CLOUD_SQL_INSTANCE*: A Cloud SQL instance that is not replicating from a primary instance. *ON_PREMISES_INSTANCE*: An instance running on the customer's premises. *READ_REPLICA_INSTANCE*: A Cloud SQL instance configured as a read-replica."]
        pub instance_type: ::std::option::Option<DatabaseInstanceInstanceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The assigned IP addresses for the instance."]
        pub ip_addresses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IpMapping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv6Address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances."]
        pub ipv6_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#instance*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterInstanceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the instance which will act as primary in the replication setup."]
        pub master_instance_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxDiskSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum disk size of the instance in bytes."]
        pub max_disk_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the Cloud SQL instance. This does not include the project ID."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onPremisesConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration specific to on-premises instances."]
        pub on_premises_configuration:
            ::std::option::Option<::std::boxed::Box<OnPremisesConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "project")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable."]
        pub project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographical region. Can be *us-central* (*FIRST_GEN* instances only) *us-central1* (*SECOND_GEN* instances only) *asia-east1* or *europe-west1*. Defaults to *us-central* or *us-central1* depending on the instance type. The region cannot be changed after instance creation."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration specific to failover replicas and read replicas."]
        pub replica_configuration: ::std::option::Option<::std::boxed::Box<ReplicaConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The replicas of the instance."]
        pub replica_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Initial root password. Use only on creation."]
        pub root_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "satisfiesPzs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status indicating if instance satisfies physical zone separation. Reserved for future use."]
        pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledMaintenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of any upcoming scheduled maintenance for this instance."]
        pub scheduled_maintenance:
            ::std::option::Option<::std::boxed::Box<SqlScheduledMaintenance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryGceZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone. Reserved for future use."]
        pub secondary_gce_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverCaCert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SSL configuration."]
        pub server_ca_cert: ::std::option::Option<::std::boxed::Box<SslCert>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service account email address assigned to the instance. This property is applicable only to Second Generation instances."]
        pub service_account_email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user settings."]
        pub settings: ::std::option::Option<::std::boxed::Box<Settings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current serving state of the Cloud SQL instance. This can be one of the following. *SQL_INSTANCE_STATE_UNSPECIFIED*: The state of the instance is unknown. *RUNNABLE*: The instance is running, or has been stopped by owner. *SUSPENDED*: The instance is not available, for example due to problems with billing. for example due to problems with billing. *PENDING_DELETE*: The instance is being deleted. *PENDING_CREATE*: The instance is being created. *MAINTENANCE*: The instance is down for maintenance. *FAILED*: The instance creation failed."]
        pub state: ::std::option::Option<DatabaseInstanceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspensionReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the instance state is SUSPENDED, the reason for the suspension."]
        pub suspension_reason:
            ::std::option::Option<::std::vec::Vec<DatabaseInstanceSuspensionReasonEnum>>,
    }
    impl DatabaseInstance {
        pub fn builder() -> DatabaseInstanceBuilder {
            DatabaseInstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = " *SECOND_GEN*: Cloud SQL database instance. *EXTERNAL*: A database server that is not managed by Google. This property is read-only; use the *tier* property in the *settings* object to determine the database type."]
    pub enum DatabaseInstanceBackendTypeEnum {
        #[serde(rename = "SQL_BACKEND_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown backend type for instance."]
        SqlBackendTypeUnspecified,
        #[serde(rename = "FIRST_GEN")]
        #[doc = "V1 speckle instance."]
        FirstGen,
        #[serde(rename = "SECOND_GEN")]
        #[doc = "V2 speckle instance."]
        SecondGen,
        #[serde(rename = "EXTERNAL")]
        #[doc = "On premises instance."]
        External,
    }
    impl ::std::default::Default for DatabaseInstanceBackendTypeEnum {
        fn default() -> Self {
            Self::SqlBackendTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The database engine type and version. The *databaseVersion* field cannot be changed after instance creation. MySQL instances: *MYSQL_8_0*, *MYSQL_5_7* (default), or *MYSQL_5_6*. PostgreSQL instances: *POSTGRES_9_6*, *POSTGRES_10*, *POSTGRES_11* or *POSTGRES_12* (default). SQL Server instances: *SQLSERVER_2017_STANDARD* (default), *SQLSERVER_2017_ENTERPRISE*, *SQLSERVER_2017_EXPRESS*, or *SQLSERVER_2017_WEB*."]
    pub enum DatabaseInstanceDatabaseVersionEnum {
        #[serde(rename = "SQL_DATABASE_VERSION_UNSPECIFIED")]
        #[doc = "This is an unknown database version."]
        SqlDatabaseVersionUnspecified,
        #[serde(rename = "MYSQL_5_1")]
        #[doc = "The database version is MySQL 5.1."]
        Mysql51,
        #[serde(rename = "MYSQL_5_5")]
        #[doc = "The database version is MySQL 5.5."]
        Mysql55,
        #[serde(rename = "MYSQL_5_6")]
        #[doc = "The database version is MySQL 5.6."]
        Mysql56,
        #[serde(rename = "MYSQL_5_7")]
        #[doc = "The database version is MySQL 5.7."]
        Mysql57,
        #[serde(rename = "POSTGRES_9_6")]
        #[doc = "The database version is PostgreSQL 9.6."]
        Postgres96,
        #[serde(rename = "POSTGRES_11")]
        #[doc = "The database version is PostgreSQL 11."]
        Postgres11,
        #[serde(rename = "SQLSERVER_2017_STANDARD")]
        #[doc = "The database version is SQL Server 2017 Standard."]
        Sqlserver2017Standard,
        #[serde(rename = "SQLSERVER_2017_ENTERPRISE")]
        #[doc = "The database version is SQL Server 2017 Enterprise."]
        Sqlserver2017Enterprise,
        #[serde(rename = "SQLSERVER_2017_EXPRESS")]
        #[doc = "The database version is SQL Server 2017 Express."]
        Sqlserver2017Express,
        #[serde(rename = "SQLSERVER_2017_WEB")]
        #[doc = "The database version is SQL Server 2017 Web."]
        Sqlserver2017Web,
        #[serde(rename = "POSTGRES_10")]
        #[doc = "The database version is PostgreSQL 10."]
        Postgres10,
        #[serde(rename = "POSTGRES_12")]
        #[doc = "The database version is PostgreSQL 12."]
        Postgres12,
        #[serde(rename = "MYSQL_8_0")]
        #[doc = "The database version is MySQL 8."]
        Mysql80,
        #[serde(rename = "POSTGRES_13")]
        #[doc = "The database version is PostgreSQL 13."]
        Postgres13,
    }
    impl ::std::default::Default for DatabaseInstanceDatabaseVersionEnum {
        fn default() -> Self {
            Self::SqlDatabaseVersionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The name and status of the failover replica. This property is applicable only to Second Generation instances."]
    pub struct DatabaseInstanceFailoverReplica {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "available")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The availability status of the failover replica. A false status indicates that the failover replica is out of sync. The primary instance can only failover to the failover replica when the status is true."]
        pub available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the failover replica. If specified at instance creation, a failover replica is created for the instance. The name doesn't include the project ID. This property is applicable only to Second Generation instances."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl DatabaseInstanceFailoverReplica {
        pub fn builder() -> DatabaseInstanceFailoverReplicaBuilder {
            DatabaseInstanceFailoverReplicaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The instance type. This can be one of the following. *CLOUD_SQL_INSTANCE*: A Cloud SQL instance that is not replicating from a primary instance. *ON_PREMISES_INSTANCE*: An instance running on the customer's premises. *READ_REPLICA_INSTANCE*: A Cloud SQL instance configured as a read-replica."]
    pub enum DatabaseInstanceInstanceTypeEnum {
        #[serde(rename = "SQL_INSTANCE_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown Cloud SQL instance type."]
        SqlInstanceTypeUnspecified,
        #[serde(rename = "CLOUD_SQL_INSTANCE")]
        #[doc = "A regular Cloud SQL instance."]
        CloudSqlInstance,
        #[serde(rename = "ON_PREMISES_INSTANCE")]
        #[doc = "An instance running on the customer's premises that is not managed by Cloud SQL."]
        OnPremisesInstance,
        #[serde(rename = "READ_REPLICA_INSTANCE")]
        #[doc = "A Cloud SQL instance acting as a read-replica."]
        ReadReplicaInstance,
    }
    impl ::std::default::Default for DatabaseInstanceInstanceTypeEnum {
        fn default() -> Self {
            Self::SqlInstanceTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current serving state of the Cloud SQL instance. This can be one of the following. *SQL_INSTANCE_STATE_UNSPECIFIED*: The state of the instance is unknown. *RUNNABLE*: The instance is running, or has been stopped by owner. *SUSPENDED*: The instance is not available, for example due to problems with billing. for example due to problems with billing. *PENDING_DELETE*: The instance is being deleted. *PENDING_CREATE*: The instance is being created. *MAINTENANCE*: The instance is down for maintenance. *FAILED*: The instance creation failed."]
    pub enum DatabaseInstanceStateEnum {
        #[serde(rename = "SQL_INSTANCE_STATE_UNSPECIFIED")]
        #[doc = "The state of the instance is unknown."]
        SqlInstanceStateUnspecified,
        #[serde(rename = "RUNNABLE")]
        #[doc = "The instance is running, or has been stopped by owner."]
        Runnable,
        #[serde(rename = "SUSPENDED")]
        #[doc = "The instance is not available, for example due to problems with billing."]
        Suspended,
        #[serde(rename = "PENDING_DELETE")]
        #[doc = "The instance is being deleted."]
        PendingDelete,
        #[serde(rename = "PENDING_CREATE")]
        #[doc = "The instance is being created."]
        PendingCreate,
        #[serde(rename = "MAINTENANCE")]
        #[doc = "The instance is down for maintenance."]
        Maintenance,
        #[serde(rename = "FAILED")]
        #[doc = "The creation of the instance failed or a fatal error occurred during maintenance."]
        Failed,
    }
    impl ::std::default::Default for DatabaseInstanceStateEnum {
        fn default() -> Self {
            Self::SqlInstanceStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum DatabaseInstanceSuspensionReasonEnum {
        #[serde(rename = "SQL_SUSPENSION_REASON_UNSPECIFIED")]
        #[doc = "This is an unknown suspension reason."]
        SqlSuspensionReasonUnspecified,
        #[serde(rename = "BILLING_ISSUE")]
        #[doc = "The instance is suspended due to billing issues (for example:, GCP account issue)"]
        BillingIssue,
        #[serde(rename = "LEGAL_ISSUE")]
        #[doc = "The instance is suspended due to illegal content (for example:, child pornography, copyrighted material, etc.)."]
        LegalIssue,
        #[serde(rename = "OPERATIONAL_ISSUE")]
        #[doc = "The instance is causing operational issues (for example:, causing the database to crash)."]
        OperationalIssue,
        #[serde(rename = "KMS_KEY_ISSUE")]
        #[doc = "The KMS key used by the instance is either revoked or denied access to"]
        KmsKeyIssue,
    }
    impl ::std::default::Default for DatabaseInstanceSuspensionReasonEnum {
        fn default() -> Self {
            Self::SqlSuspensionReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database list response."]
    pub struct DatabasesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of database resources in the instance."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Database>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#databasesList*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl DatabasesListResponse {
        pub fn builder() -> DatabasesListResponseBuilder {
            DatabasesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-replica configuration for connecting to the on-premises primary instance."]
    pub struct DemoteMasterConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#demoteMasterConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mysqlReplicaConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MySQL specific configuration when replicating from a MySQL on-premises primary instance. Replication configuration information such as the username, password, certificates, and keys are not stored in the instance metadata. The configuration information is used only to set up the replication connection and is stored by MySQL in a file named *master.info* in the data directory."]
        pub mysql_replica_configuration:
            ::std::option::Option<::std::boxed::Box<DemoteMasterMySqlReplicaConfiguration>>,
    }
    impl DemoteMasterConfiguration {
        pub fn builder() -> DemoteMasterConfigurationBuilder {
            DemoteMasterConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance demote primary instance context."]
    pub struct DemoteMasterContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#demoteMasterContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterInstanceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the instance which will act as on-premises primary instance in the replication setup."]
        pub master_instance_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration specific to read-replicas replicating from the on-premises primary instance."]
        pub replica_configuration:
            ::std::option::Option<::std::boxed::Box<DemoteMasterConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifyGtidConsistency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Verify GTID consistency for demote operation. Default value: *True*. Second Generation instances only. Setting this flag to false enables you to bypass GTID consistency check between on-premises primary instance and Cloud SQL instance during the demotion operation but also exposes you to the risk of future replication failures. Change the value only if you know the reason for the GTID divergence and are confident that doing so will not cause any replication issues."]
        pub verify_gtid_consistency: ::std::option::Option<::std::primitive::bool>,
    }
    impl DemoteMasterContext {
        pub fn builder() -> DemoteMasterContextBuilder {
            DemoteMasterContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-replica configuration specific to MySQL databases."]
    pub struct DemoteMasterMySqlReplicaConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        pub ca_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's x509 certificate."]
        pub client_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate. The format of the replica's private key can be either PKCS #1 or PKCS #8."]
        pub client_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#demoteMasterMysqlReplicaConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password for the replication connection."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username for the replication connection."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl DemoteMasterMySqlReplicaConfiguration {
        pub fn builder() -> DemoteMasterMySqlReplicaConfigurationBuilder {
            DemoteMasterMySqlReplicaConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deny Maintenance Periods. This specifies a date range during when all CSA rollout will be denied."]
    pub struct DenyMaintenancePeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "\"deny maintenance period\" end date. If the year of the end date is empty, the year of the start date also must be empty. In this case, it means the deny maintenance period recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "\"deny maintenance period\" start date. If the year of the start date is empty, the year of the end date also must be empty. In this case, it means the deny maintenance period recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time in UTC when the \"deny maintenance period\" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00"]
        pub time: ::std::option::Option<::std::string::String>,
    }
    impl DenyMaintenancePeriod {
        pub fn builder() -> DenyMaintenancePeriodBuilder {
            DenyMaintenancePeriodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Disk encryption configuration for an instance."]
    pub struct DiskEncryptionConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#diskEncryptionConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of KMS key for disk encryption"]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
    }
    impl DiskEncryptionConfiguration {
        pub fn builder() -> DiskEncryptionConfigurationBuilder {
            DiskEncryptionConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Disk encryption status for an instance."]
    pub struct DiskEncryptionStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#diskEncryptionStatus*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyVersionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "KMS key version used to encrypt the Cloud SQL instance resource"]
        pub kms_key_version_name: ::std::option::Option<::std::string::String>,
    }
    impl DiskEncryptionStatus {
        pub fn builder() -> DiskEncryptionStatusBuilder {
            DiskEncryptionStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance export context."]
    pub struct ExportContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvExportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for exporting data as CSV. *MySQL* and *PostgreSQL* instances only."]
        pub csv_export_options: ::std::option::Option<ExportContextCsvExportOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Databases to be exported. *MySQL instances:* If *fileType* is *SQL* and no database is specified, all databases are exported, except for the *mysql* system database. If *fileType* is *CSV*, you can specify one database, either by using this property or by using the *csvExportOptions.selectQuery* property, which takes precedence over this property. *PostgreSQL instances:* You must specify one database to be exported. If *fileType* is *CSV*, this database must match the one specified in the *csvExportOptions.selectQuery* property."]
        pub databases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file type for the specified uri. *SQL*: The file contains SQL statements. *CSV*: The file contains CSV data. *BAK*: The file contains backup data for a SQL Server instance."]
        pub file_type: ::std::option::Option<ExportContextFileTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#exportContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option for export offload."]
        pub offload: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sqlExportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for exporting data as SQL statements."]
        pub sql_export_options: ::std::option::Option<ExportContextSqlExportOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to the file in Google Cloud Storage where the export will be stored. The URI is in the form *gs://bucketName/fileName*. If the file already exists, the request succeeds, but the operation fails. If *fileType* is *SQL* and the filename ends with .gz, the contents are compressed."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl ExportContext {
        pub fn builder() -> ExportContextBuilder {
            ExportContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for exporting data as CSV. *MySQL* and *PostgreSQL* instances only."]
    pub struct ExportContextCsvExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The select query used to extract the data."]
        pub select_query: ::std::option::Option<::std::string::String>,
    }
    impl ExportContextCsvExportOptions {
        pub fn builder() -> ExportContextCsvExportOptionsBuilder {
            ExportContextCsvExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The file type for the specified uri. *SQL*: The file contains SQL statements. *CSV*: The file contains CSV data. *BAK*: The file contains backup data for a SQL Server instance."]
    pub enum ExportContextFileTypeEnum {
        #[serde(rename = "SQL_FILE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown file type."]
        SqlFileTypeUnspecified,
        #[serde(rename = "SQL")]
        #[doc = "File containing SQL statements."]
        Sql,
        #[serde(rename = "CSV")]
        #[doc = "File in CSV format."]
        Csv,
        #[serde(rename = "BAK")]
        #[doc = ""]
        Bak,
    }
    impl ::std::default::Default for ExportContextFileTypeEnum {
        fn default() -> Self {
            Self::SqlFileTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for exporting data as SQL statements."]
    pub struct ExportContextSqlExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mysqlExportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for exporting from MySQL."]
        pub mysql_export_options:
            ::std::option::Option<ExportContextSqlExportOptionsMysqlExportOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Export only schemas."]
        pub schema_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tables to export, or that were exported, from the specified database. If you specify tables, specify one and only one database. For PostgreSQL instances, you can specify only one table."]
        pub tables: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ExportContextSqlExportOptions {
        pub fn builder() -> ExportContextSqlExportOptionsBuilder {
            ExportContextSqlExportOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for exporting from MySQL."]
    pub struct ExportContextSqlExportOptionsMysqlExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to include SQL statement required to set up replication. If set to *1*, the dump file includes a CHANGE MASTER TO statement with the binary log coordinates, and --set-gtid-purged is set to ON. If set to *2*, the CHANGE MASTER TO statement is written as a SQL comment and has no effect. If set to any value other than *1*, --set-gtid-purged is set to OFF."]
        pub master_data: ::std::option::Option<::std::primitive::i64>,
    }
    impl ExportContextSqlExportOptionsMysqlExportOptions {
        pub fn builder() -> ExportContextSqlExportOptionsMysqlExportOptionsBuilder {
            ExportContextSqlExportOptionsMysqlExportOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance failover context."]
    pub struct FailoverContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#failoverContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingsVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current settings version of this instance. Request will be rejected if this version doesn't match the current settings version."]
        pub settings_version: ::std::option::Option<::std::string::String>,
    }
    impl FailoverContext {
        pub fn builder() -> FailoverContextBuilder {
            FailoverContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A flag resource."]
    pub struct Flag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedIntValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use this field if only certain integers are accepted. Can be combined with min_value and max_value to add additional values."]
        pub allowed_int_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedStringValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For *STRING* flags, a list of strings that the value can be set to."]
        pub allowed_string_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appliesTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The database version this flag applies to. Can be *MYSQL_8_0*, *MYSQL_5_6*, or *MYSQL_5_7*."]
        pub applies_to: ::std::option::Option<::std::vec::Vec<FlagAppliesToEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inBeta")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the flag is considered in beta."]
        pub in_beta: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#flag*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For *INTEGER* flags, the maximum allowed value."]
        pub max_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For *INTEGER* flags, the minimum allowed value."]
        pub min_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the name of the flag. Flag names always use underscores, not hyphens, for example: *max_allowed_packet*"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiresRestart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether changing this flag will trigger a database restart. Only applicable to Second Generation instances."]
        pub requires_restart: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the flag. Flags are typed to being *BOOLEAN*, *STRING*, *INTEGER* or *NONE*. *NONE* is used for flags which do not take a value, such as *skip_grant_tables*."]
        pub _type: ::std::option::Option<FlagTypeEnum>,
    }
    impl Flag {
        pub fn builder() -> FlagBuilder {
            FlagBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum FlagAppliesToEnum {
        #[serde(rename = "SQL_DATABASE_VERSION_UNSPECIFIED")]
        #[doc = "This is an unknown database version."]
        SqlDatabaseVersionUnspecified,
        #[serde(rename = "MYSQL_5_1")]
        #[doc = "The database version is MySQL 5.1."]
        Mysql51,
        #[serde(rename = "MYSQL_5_5")]
        #[doc = "The database version is MySQL 5.5."]
        Mysql55,
        #[serde(rename = "MYSQL_5_6")]
        #[doc = "The database version is MySQL 5.6."]
        Mysql56,
        #[serde(rename = "MYSQL_5_7")]
        #[doc = "The database version is MySQL 5.7."]
        Mysql57,
        #[serde(rename = "POSTGRES_9_6")]
        #[doc = "The database version is PostgreSQL 9.6."]
        Postgres96,
        #[serde(rename = "POSTGRES_11")]
        #[doc = "The database version is PostgreSQL 11."]
        Postgres11,
        #[serde(rename = "SQLSERVER_2017_STANDARD")]
        #[doc = "The database version is SQL Server 2017 Standard."]
        Sqlserver2017Standard,
        #[serde(rename = "SQLSERVER_2017_ENTERPRISE")]
        #[doc = "The database version is SQL Server 2017 Enterprise."]
        Sqlserver2017Enterprise,
        #[serde(rename = "SQLSERVER_2017_EXPRESS")]
        #[doc = "The database version is SQL Server 2017 Express."]
        Sqlserver2017Express,
        #[serde(rename = "SQLSERVER_2017_WEB")]
        #[doc = "The database version is SQL Server 2017 Web."]
        Sqlserver2017Web,
        #[serde(rename = "POSTGRES_10")]
        #[doc = "The database version is PostgreSQL 10."]
        Postgres10,
        #[serde(rename = "POSTGRES_12")]
        #[doc = "The database version is PostgreSQL 12."]
        Postgres12,
        #[serde(rename = "MYSQL_8_0")]
        #[doc = "The database version is MySQL 8."]
        Mysql80,
        #[serde(rename = "POSTGRES_13")]
        #[doc = "The database version is PostgreSQL 13."]
        Postgres13,
    }
    impl ::std::default::Default for FlagAppliesToEnum {
        fn default() -> Self {
            Self::SqlDatabaseVersionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the flag. Flags are typed to being *BOOLEAN*, *STRING*, *INTEGER* or *NONE*. *NONE* is used for flags which do not take a value, such as *skip_grant_tables*."]
    pub enum FlagTypeEnum {
        #[serde(rename = "SQL_FLAG_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown flag type."]
        SqlFlagTypeUnspecified,
        #[serde(rename = "BOOLEAN")]
        #[doc = "Boolean type flag."]
        Boolean,
        #[serde(rename = "STRING")]
        #[doc = "String type flag."]
        String,
        #[serde(rename = "INTEGER")]
        #[doc = "Integer type flag."]
        Integer,
        #[serde(rename = "NONE")]
        #[doc = "Flag type used for a server startup option."]
        None,
        #[serde(rename = "MYSQL_TIMEZONE_OFFSET")]
        #[doc = "Type introduced specically for MySQL TimeZone offset. Accept a string value with the format [-12:59, 13:00]."]
        MysqlTimezoneOffset,
        #[serde(rename = "FLOAT")]
        #[doc = "Float type flag."]
        Float,
        #[serde(rename = "REPEATED_STRING")]
        #[doc = "Comma-separated list of the strings in a SqlFlagType enum."]
        RepeatedString,
    }
    impl ::std::default::Default for FlagTypeEnum {
        fn default() -> Self {
            Self::SqlFlagTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Flags list response."]
    pub struct FlagsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of flags."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Flag>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#flagsList*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl FlagsListResponse {
        pub fn builder() -> FlagsListResponseBuilder {
            FlagsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance import context."]
    pub struct ImportContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bakImportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import parameters specific to SQL Server .BAK files"]
        pub bak_import_options: ::std::option::Option<ImportContextBakImportOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvImportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for importing data as CSV."]
        pub csv_import_options: ::std::option::Option<ImportContextCsvImportOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "database")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target database for the import. If *fileType* is *SQL*, this field is required only if the import file does not specify a database, and is overridden by any database specification in the import file. If *fileType* is *CSV*, one database must be specified."]
        pub database: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file type for the specified uri. *SQL*: The file contains SQL statements. *CSV*: The file contains CSV data."]
        pub file_type: ::std::option::Option<ImportContextFileTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The PostgreSQL user for this import operation. PostgreSQL instances only."]
        pub import_user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#importContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the import file in Cloud Storage, in the form *gs://bucketName/fileName*. Compressed gzip files (.gz) are supported when *fileType* is *SQL*. The instance must have write permissions to the bucket and read access to the file."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl ImportContext {
        pub fn builder() -> ImportContextBuilder {
            ImportContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Import parameters specific to SQL Server .BAK files"]
    pub struct ImportContextBakImportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub encryption_options:
            ::std::option::Option<ImportContextBakImportOptionsEncryptionOptions>,
    }
    impl ImportContextBakImportOptions {
        pub fn builder() -> ImportContextBakImportOptionsBuilder {
            ImportContextBakImportOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ImportContextBakImportOptionsEncryptionOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the Certificate (.cer) in Cloud Storage, in the form *gs://bucketName/fileName*. The instance must have write permissions to the bucket and read access to the file."]
        pub cert_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pvkPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password that encrypts the private key"]
        pub pvk_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pvkPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the Certificate Private Key (.pvk) in Cloud Storage, in the form *gs://bucketName/fileName*. The instance must have write permissions to the bucket and read access to the file."]
        pub pvk_path: ::std::option::Option<::std::string::String>,
    }
    impl ImportContextBakImportOptionsEncryptionOptions {
        pub fn builder() -> ImportContextBakImportOptionsEncryptionOptionsBuilder {
            ImportContextBakImportOptionsEncryptionOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for importing data as CSV."]
    pub struct ImportContextCsvImportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns to which CSV data is imported. If not specified, all columns of the database table are loaded with CSV data."]
        pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table to which CSV data is imported."]
        pub table: ::std::option::Option<::std::string::String>,
    }
    impl ImportContextCsvImportOptions {
        pub fn builder() -> ImportContextCsvImportOptionsBuilder {
            ImportContextCsvImportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The file type for the specified uri. *SQL*: The file contains SQL statements. *CSV*: The file contains CSV data."]
    pub enum ImportContextFileTypeEnum {
        #[serde(rename = "SQL_FILE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown file type."]
        SqlFileTypeUnspecified,
        #[serde(rename = "SQL")]
        #[doc = "File containing SQL statements."]
        Sql,
        #[serde(rename = "CSV")]
        #[doc = "File in CSV format."]
        Csv,
        #[serde(rename = "BAK")]
        #[doc = ""]
        Bak,
    }
    impl ::std::default::Default for ImportContextFileTypeEnum {
        fn default() -> Self {
            Self::SqlFileTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Insights configuration. This specifies when Cloud SQL Insights feature is enabled and optional configuration."]
    pub struct InsightsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryInsightsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Query Insights feature is enabled."]
        pub query_insights_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryStringLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum query length stored in bytes. Default value: 1024 bytes. Range: 256-4500 bytes. Query length more than this field value will be truncated to this value. When unset, query length will be the default value."]
        pub query_string_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordApplicationTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Query Insights will record application tags from query when enabled."]
        pub record_application_tags: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordClientAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Query Insights will record client address when enabled."]
        pub record_client_address: ::std::option::Option<::std::primitive::bool>,
    }
    impl InsightsConfig {
        pub fn builder() -> InsightsConfigBuilder {
            InsightsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance clone request."]
    pub struct InstancesCloneRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloneContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the clone operation."]
        pub clone_context: ::std::option::Option<::std::boxed::Box<CloneContext>>,
    }
    impl InstancesCloneRequest {
        pub fn builder() -> InstancesCloneRequestBuilder {
            InstancesCloneRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database demote primary instance request."]
    pub struct InstancesDemoteMasterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "demoteMasterContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the demoteMaster operation."]
        pub demote_master_context: ::std::option::Option<::std::boxed::Box<DemoteMasterContext>>,
    }
    impl InstancesDemoteMasterRequest {
        pub fn builder() -> InstancesDemoteMasterRequestBuilder {
            InstancesDemoteMasterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance export request."]
    pub struct InstancesExportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the export operation."]
        pub export_context: ::std::option::Option<::std::boxed::Box<ExportContext>>,
    }
    impl InstancesExportRequest {
        pub fn builder() -> InstancesExportRequestBuilder {
            InstancesExportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instance failover request."]
    pub struct InstancesFailoverRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failoverContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Failover Context."]
        pub failover_context: ::std::option::Option<::std::boxed::Box<FailoverContext>>,
    }
    impl InstancesFailoverRequest {
        pub fn builder() -> InstancesFailoverRequestBuilder {
            InstancesFailoverRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance import request."]
    pub struct InstancesImportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the import operation."]
        pub import_context: ::std::option::Option<::std::boxed::Box<ImportContext>>,
    }
    impl InstancesImportRequest {
        pub fn builder() -> InstancesImportRequestBuilder {
            InstancesImportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instances list response."]
    pub struct InstancesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of database instance resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatabaseInstance>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#instancesList*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of warnings that occurred while handling the request."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiWarning>>>,
    }
    impl InstancesListResponse {
        pub fn builder() -> InstancesListResponseBuilder {
            InstancesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instances ListServerCas response."]
    pub struct InstancesListServerCasResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub active_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of server CA certificates for the instance."]
        pub certs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SslCert>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#instancesListServerCas*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl InstancesListServerCasResponse {
        pub fn builder() -> InstancesListServerCasResponseBuilder {
            InstancesListServerCasResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance restore backup request."]
    pub struct InstancesRestoreBackupRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restoreBackupContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters required to perform the restore backup operation."]
        pub restore_backup_context: ::std::option::Option<::std::boxed::Box<RestoreBackupContext>>,
    }
    impl InstancesRestoreBackupRequest {
        pub fn builder() -> InstancesRestoreBackupRequestBuilder {
            InstancesRestoreBackupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rotate Server CA request."]
    pub struct InstancesRotateServerCaRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotateServerCaContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the rotate server CA operation."]
        pub rotate_server_ca_context:
            ::std::option::Option<::std::boxed::Box<RotateServerCaContext>>,
    }
    impl InstancesRotateServerCaRequest {
        pub fn builder() -> InstancesRotateServerCaRequestBuilder {
            InstancesRotateServerCaRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instance truncate log request."]
    pub struct InstancesTruncateLogRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "truncateLogContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains details about the truncate log operation."]
        pub truncate_log_context: ::std::option::Option<::std::boxed::Box<TruncateLogContext>>,
    }
    impl InstancesTruncateLogRequest {
        pub fn builder() -> InstancesTruncateLogRequestBuilder {
            InstancesTruncateLogRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "IP Management configuration."]
    pub struct IpConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedNetworks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of external networks that are allowed to connect to the instance using the IP. In 'CIDR' notation, also known as 'slash' notation (for example: *192.168.100.0/24*)."]
        pub authorized_networks:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AclEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv4Enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the instance is assigned a public IP address or not."]
        pub ipv4_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, */projects/myProject/global/networks/default*. This setting can be updated, but it cannot be removed after it is set."]
        pub private_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireSsl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether SSL connections over IP are enforced or not."]
        pub require_ssl: ::std::option::Option<::std::primitive::bool>,
    }
    impl IpConfiguration {
        pub fn builder() -> IpConfigurationBuilder {
            IpConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance IP Mapping."]
    pub struct IpMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address assigned."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeToRetire")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The due time for this IP to be retired in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*. This field is only available when the IP is scheduled to be retired."]
        pub time_to_retire: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this IP address. A *PRIMARY* address is a public address that can accept incoming connections. A *PRIVATE* address is a private address that can accept incoming connections. An *OUTGOING* address is the source address of connections originating from the instance, if supported."]
        pub _type: ::std::option::Option<IpMappingTypeEnum>,
    }
    impl IpMapping {
        pub fn builder() -> IpMappingBuilder {
            IpMappingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this IP address. A *PRIMARY* address is a public address that can accept incoming connections. A *PRIVATE* address is a private address that can accept incoming connections. An *OUTGOING* address is the source address of connections originating from the instance, if supported."]
    pub enum IpMappingTypeEnum {
        #[serde(rename = "SQL_IP_ADDRESS_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown IP address type."]
        SqlIpAddressTypeUnspecified,
        #[serde(rename = "PRIMARY")]
        #[doc = "IP address the customer is supposed to connect to. Usually this is the load balancer's IP address"]
        Primary,
        #[serde(rename = "OUTGOING")]
        #[doc = "Source IP address of the connection a read replica establishes to its external primary instance. This IP address can be allowlisted by the customer in case it has a firewall that filters incoming connection to its on premises primary instance."]
        Outgoing,
        #[serde(rename = "PRIVATE")]
        #[doc = "Private IP used when using private IPs and network peering."]
        Private,
        #[serde(rename = "MIGRATED_1ST_GEN")]
        #[doc = "V1 IP of a migrated instance. We want the user to decommission this IP as soon as the migration is complete. Note: V1 instances with V1 ip addresses will be counted as PRIMARY."]
        Migrated1StGen,
    }
    impl ::std::default::Default for IpMappingTypeEnum {
        fn default() -> Self {
            Self::SqlIpAddressTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Preferred location. This specifies where a Cloud SQL instance is located, either in a specific Compute Engine zone, or co-located with an App Engine application. Note that if the preferred location is not available, the instance will be located as close as possible within the region. Only one location may be specified."]
    pub struct LocationPreference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followGaeApplication")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The App Engine application to follow, it must be in the same region as the Cloud SQL instance."]
        pub follow_gae_application: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#locationPreference*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The preferred Compute Engine zone for the secondary/failover (for example: us-central1-a, us-central1-b, etc.). Reserved for future use."]
        pub secondary_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The preferred Compute Engine zone (for example: us-central1-a, us-central1-b, etc.)."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl LocationPreference {
        pub fn builder() -> LocationPreferenceBuilder {
            LocationPreferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Maintenance window. This specifies when a Cloud SQL instance is restarted for system maintenance purposes."]
    pub struct MaintenanceWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "day of week (1-7), starting on Monday."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "hour of day - 0 to 23."]
        pub hour: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#maintenanceWindow*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTrack")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maintenance timing setting: *canary* (Earlier) or *stable* (Later). Learn more."]
        pub update_track: ::std::option::Option<MaintenanceWindowUpdateTrackEnum>,
    }
    impl MaintenanceWindow {
        pub fn builder() -> MaintenanceWindowBuilder {
            MaintenanceWindowBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Maintenance timing setting: *canary* (Earlier) or *stable* (Later). Learn more."]
    pub enum MaintenanceWindowUpdateTrackEnum {
        #[serde(rename = "SQL_UPDATE_TRACK_UNSPECIFIED")]
        #[doc = "This is an unknown maintenance timing preference."]
        SqlUpdateTrackUnspecified,
        #[serde(rename = "canary")]
        #[doc = "For instance update that requires a restart, this update track indicates your instance prefer to restart for new version early in maintenance window."]
        Canary,
        #[serde(rename = "stable")]
        #[doc = "For instance update that requires a restart, this update track indicates your instance prefer to let Cloud SQL choose the timing of restart (within its Maintenance window, if applicable)."]
        Stable,
    }
    impl ::std::default::Default for MaintenanceWindowUpdateTrackEnum {
        fn default() -> Self {
            Self::SqlUpdateTrackUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-replica configuration specific to MySQL databases."]
    pub struct MySqlReplicaConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        pub ca_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's x509 certificate."]
        pub client_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate."]
        pub client_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectRetryInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seconds to wait between connect retries. MySQL's default is 60 seconds."]
        pub connect_retry_interval: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dumpFilePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to a SQL dump file in Google Cloud Storage from which the replica instance is to be created. The URI is in the form gs://bucketName/fileName. Compressed gzip files (.gz) are also supported. Dumps have the binlog co-ordinates from which replication begins. This can be accomplished by setting --master-data to 1 when using mysqldump."]
        pub dump_file_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#mysqlReplicaConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterHeartbeatPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Interval in milliseconds between replication heartbeats."]
        pub master_heartbeat_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password for the replication connection."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sslCipher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of permissible ciphers to use for SSL encryption."]
        pub ssl_cipher: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username for the replication connection."]
        pub username: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifyServerCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not to check the primary instance's Common Name value in the certificate that it sends during the SSL handshake."]
        pub verify_server_certificate: ::std::option::Option<::std::primitive::bool>,
    }
    impl MySqlReplicaConfiguration {
        pub fn builder() -> MySqlReplicaConfigurationBuilder {
            MySqlReplicaConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "On-premises instance configuration."]
    pub struct OnPremisesConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the trusted CA's x509 certificate."]
        pub ca_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's x509 certificate."]
        pub client_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate."]
        pub client_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dumpFilePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dump file to create the Cloud SQL replica."]
        pub dump_file_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostPort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The host and port of the on-premises instance in host:port format"]
        pub host_port: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#onPremisesConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password for connecting to on-premises instance."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username for connecting to on-premises instance."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl OnPremisesConfiguration {
        pub fn builder() -> OnPremisesConfigurationBuilder {
            OnPremisesConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Operation resource. For successful operations that return an Operation resource, only the fields relevant to the operation are populated in the resource. Next field: 18"]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The context for backup operation, if applicable."]
        pub backup_context: ::std::option::Option<::std::boxed::Box<BackupContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation finished in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If errors occurred during processing of this operation, this field will be populated."]
        pub error: ::std::option::Option<::std::boxed::Box<OperationErrors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The context for export operation, if applicable."]
        pub export_context: ::std::option::Option<::std::boxed::Box<ExportContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The context for import operation, if applicable."]
        pub import_context: ::std::option::Option<::std::boxed::Box<ImportContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation was enqueued in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#operation*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the operation. Valid values are: *CREATE* *DELETE* *UPDATE* *RESTART* *IMPORT* *EXPORT* *BACKUP_VOLUME* *RESTORE_VOLUME* *CREATE_USER* *DELETE_USER* *CREATE_DATABASE* *DELETE_DATABASE*"]
        pub operation_type: ::std::option::Option<OperationOperationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation actually started in UTC timezone in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of an operation. Valid values are: *PENDING* *RUNNING* *DONE* *SQL_OPERATION_STATUS_UNSPECIFIED*"]
        pub status: ::std::option::Option<OperationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the database instance related to this operation."]
        pub target_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetProject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project ID of the target instance related to this operation."]
        pub target_project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user who initiated this operation."]
        pub user: ::std::option::Option<::std::string::String>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the operation. Valid values are: *CREATE* *DELETE* *UPDATE* *RESTART* *IMPORT* *EXPORT* *BACKUP_VOLUME* *RESTORE_VOLUME* *CREATE_USER* *DELETE_USER* *CREATE_DATABASE* *DELETE_DATABASE*"]
    pub enum OperationOperationTypeEnum {
        #[serde(rename = "SQL_OPERATION_TYPE_UNSPECIFIED")]
        #[doc = "Unknown operation type."]
        SqlOperationTypeUnspecified,
        #[serde(rename = "IMPORT")]
        #[doc = "Imports data into a Cloud SQL instance."]
        Import,
        #[serde(rename = "EXPORT")]
        #[doc = "Exports data from a Cloud SQL instance to a Cloud Storage bucket."]
        Export,
        #[serde(rename = "CREATE")]
        #[doc = "Creates a new Cloud SQL instance."]
        Create,
        #[serde(rename = "UPDATE")]
        #[doc = "Updates the settings of a Cloud SQL instance."]
        Update,
        #[serde(rename = "DELETE")]
        #[doc = "Deletes a Cloud SQL instance."]
        Delete,
        #[serde(rename = "RESTART")]
        #[doc = "Restarts the Cloud SQL instance."]
        Restart,
        #[serde(rename = "BACKUP")]
        #[doc = ""]
        Backup,
        #[serde(rename = "SNAPSHOT")]
        #[doc = ""]
        Snapshot,
        #[serde(rename = "BACKUP_VOLUME")]
        #[doc = "Performs instance backup."]
        BackupVolume,
        #[serde(rename = "DELETE_VOLUME")]
        #[doc = "Deletes an instance backup."]
        DeleteVolume,
        #[serde(rename = "RESTORE_VOLUME")]
        #[doc = "Restores an instance backup."]
        RestoreVolume,
        #[serde(rename = "INJECT_USER")]
        #[doc = "Injects a privileged user in mysql for MOB instances."]
        InjectUser,
        #[serde(rename = "CLONE")]
        #[doc = "Clones a Cloud SQL instance."]
        Clone,
        #[serde(rename = "STOP_REPLICA")]
        #[doc = "Stops replication on a Cloud SQL read replica instance."]
        StopReplica,
        #[serde(rename = "START_REPLICA")]
        #[doc = "Starts replication on a Cloud SQL read replica instance."]
        StartReplica,
        #[serde(rename = "PROMOTE_REPLICA")]
        #[doc = "Promotes a Cloud SQL replica instance."]
        PromoteReplica,
        #[serde(rename = "CREATE_REPLICA")]
        #[doc = "Creates a Cloud SQL replica instance."]
        CreateReplica,
        #[serde(rename = "CREATE_USER")]
        #[doc = "Creates a new user in a Cloud SQL instance."]
        CreateUser,
        #[serde(rename = "DELETE_USER")]
        #[doc = "Deletes a user from a Cloud SQL instance."]
        DeleteUser,
        #[serde(rename = "UPDATE_USER")]
        #[doc = "Updates an existing user in a Cloud SQL instance."]
        UpdateUser,
        #[serde(rename = "CREATE_DATABASE")]
        #[doc = "Creates a database in the Cloud SQL instance."]
        CreateDatabase,
        #[serde(rename = "DELETE_DATABASE")]
        #[doc = "Deletes a database in the Cloud SQL instance."]
        DeleteDatabase,
        #[serde(rename = "UPDATE_DATABASE")]
        #[doc = "Updates a database in the Cloud SQL instance."]
        UpdateDatabase,
        #[serde(rename = "FAILOVER")]
        #[doc = "Performs failover of an HA-enabled Cloud SQL failover replica."]
        Failover,
        #[serde(rename = "DELETE_BACKUP")]
        #[doc = "Deletes the backup taken by a backup run."]
        DeleteBackup,
        #[serde(rename = "RECREATE_REPLICA")]
        #[doc = ""]
        RecreateReplica,
        #[serde(rename = "TRUNCATE_LOG")]
        #[doc = "Truncates a general or slow log table in MySQL."]
        TruncateLog,
        #[serde(rename = "DEMOTE_MASTER")]
        #[doc = "Demotes the stand-alone instance to be a Cloud SQL read replica for an external database server."]
        DemoteMaster,
        #[serde(rename = "MAINTENANCE")]
        #[doc = "Indicates that the instance is currently in maintenance. Maintenance typically causes the instance to be unavailable for 1-3 minutes."]
        Maintenance,
        #[serde(rename = "ENABLE_PRIVATE_IP")]
        #[doc = "This field is deprecated, and will be removed in future version of API."]
        EnablePrivateIp,
        #[serde(rename = "DEFER_MAINTENANCE")]
        #[doc = ""]
        DeferMaintenance,
        #[serde(rename = "CREATE_CLONE")]
        #[doc = "Creates clone instance."]
        CreateClone,
        #[serde(rename = "RESCHEDULE_MAINTENANCE")]
        #[doc = "Reschedule maintenance to another time."]
        RescheduleMaintenance,
        #[serde(rename = "START_EXTERNAL_SYNC")]
        #[doc = "Starts external sync of a Cloud SQL EM replica to an external primary instance."]
        StartExternalSync,
    }
    impl ::std::default::Default for OperationOperationTypeEnum {
        fn default() -> Self {
            Self::SqlOperationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of an operation. Valid values are: *PENDING* *RUNNING* *DONE* *SQL_OPERATION_STATUS_UNSPECIFIED*"]
    pub enum OperationStatusEnum {
        #[serde(rename = "SQL_OPERATION_STATUS_UNSPECIFIED")]
        #[doc = "The state of the operation is unknown."]
        SqlOperationStatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been queued, but has not started yet."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation completed."]
        Done,
    }
    impl ::std::default::Default for OperationStatusEnum {
        fn default() -> Self {
            Self::SqlOperationStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance operation error."]
    pub struct OperationError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the specific error that occurred."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#operationError*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the error encountered."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl OperationError {
        pub fn builder() -> OperationErrorBuilder {
            OperationErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance operation errors list wrapper."]
    pub struct OperationErrors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of errors encountered while processing this operation."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperationError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#operationErrors*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OperationErrors {
        pub fn builder() -> OperationErrorsBuilder {
            OperationErrorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance list operations response."]
    pub struct OperationsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of operation resources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#operationsList*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl OperationsListResponse {
        pub fn builder() -> OperationsListResponseBuilder {
            OperationsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-replica configuration for connecting to the primary instance."]
    pub struct ReplicaConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failoverTarget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies if the replica is the failover target. If the field is set to *true* the replica will be designated as a failover replica. In case the primary instance fails, the replica instance will be promoted as the new primary instance. Only one replica can be specified as failover target, and the replica has to be in different zone with the primary instance."]
        pub failover_target: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#replicaConfiguration*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mysqlReplicaConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MySQL specific configuration when replicating from a MySQL on-premises primary instance. Replication configuration information such as the username, password, certificates, and keys are not stored in the instance metadata. The configuration information is used only to set up the replication connection and is stored by MySQL in a file named *master.info* in the data directory."]
        pub mysql_replica_configuration:
            ::std::option::Option<::std::boxed::Box<MySqlReplicaConfiguration>>,
    }
    impl ReplicaConfiguration {
        pub fn builder() -> ReplicaConfigurationBuilder {
            ReplicaConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Reschedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rescheduleType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the reschedule."]
        pub reschedule_type: ::std::option::Option<RescheduleRescheduleTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Timestamp when the maintenance shall be rescheduled to if reschedule_type=SPECIFIC_TIME, in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub schedule_time: ::std::option::Option<::std::string::String>,
    }
    impl Reschedule {
        pub fn builder() -> RescheduleBuilder {
            RescheduleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of the reschedule."]
    pub enum RescheduleRescheduleTypeEnum {
        #[serde(rename = "RESCHEDULE_TYPE_UNSPECIFIED")]
        #[doc = ""]
        RescheduleTypeUnspecified,
        #[serde(rename = "IMMEDIATE")]
        #[doc = "If the user wants to schedule the maintenance to happen now."]
        Immediate,
        #[serde(rename = "NEXT_AVAILABLE_WINDOW")]
        #[doc = "If the user wants to use the existing maintenance policy to find the next available window."]
        NextAvailableWindow,
        #[serde(rename = "SPECIFIC_TIME")]
        #[doc = "If the user wants to reschedule the maintenance to a specific time."]
        SpecificTime,
    }
    impl ::std::default::Default for RescheduleRescheduleTypeEnum {
        fn default() -> Self {
            Self::RescheduleTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance restore from backup context. Backup context contains source instance id and project id."]
    pub struct RestoreBackupContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupRunId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the backup run to restore from."]
        pub backup_run_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the instance that the backup was taken from."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#restoreBackupContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "project")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full project ID of the source instance."]
        pub project: ::std::option::Option<::std::string::String>,
    }
    impl RestoreBackupContext {
        pub fn builder() -> RestoreBackupContextBuilder {
            RestoreBackupContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instance rotate server CA context."]
    pub struct RotateServerCaContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#rotateServerCaContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the next version to be rotated to. If left unspecified, will be rotated to the most recently added server CA version."]
        pub next_version: ::std::option::Option<::std::string::String>,
    }
    impl RotateServerCaContext {
        pub fn builder() -> RotateServerCaContextBuilder {
            RotateServerCaContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database instance settings."]
    pub struct Settings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activationPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: *ALWAYS*: The instance is on, and remains so even in the absence of connection requests. *NEVER*: The instance is off; it is not activated, even if a connection request arrives."]
        pub activation_policy: ::std::option::Option<SettingsActivationPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeDirectoryConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Active Directory configuration, relevant only for Cloud SQL for SQL Server."]
        pub active_directory_config:
            ::std::option::Option<::std::boxed::Box<SqlActiveDirectoryConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedGaeApplications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The App Engine app IDs that can access this instance. (Deprecated) Applied to First Generation instances only."]
        pub authorized_gae_applications:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availabilityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Availability type. Potential values: *ZONAL*: The instance serves data from only one zone. Outages in that zone affect data accessibility. *REGIONAL*: The instance can serve data from more than one zone in a region (it is highly available). For more information, see Overview of the High Availability Configuration."]
        pub availability_type: ::std::option::Option<SettingsAvailabilityTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The daily backup configuration for the instance."]
        pub backup_configuration: ::std::option::Option<::std::boxed::Box<BackupConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of server Instance collation."]
        pub collation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crashSafeReplicationEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration specific to read replica instances. Indicates whether database flags for crash-safe replication are enabled. This property was only applicable to First Generation instances."]
        pub crash_safe_replication_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDiskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of data disk, in GB. The data disk size minimum is 10GB."]
        pub data_disk_size_gb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDiskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of data disk: PD_SSD (default) or PD_HDD. Not used for First Generation instances."]
        pub data_disk_type: ::std::option::Option<SettingsDataDiskTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseFlags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The database flags passed to the instance at startup."]
        pub database_flags:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatabaseFlags>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseReplicationEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration specific to read replica instances. Indicates whether replication is enabled or not."]
        pub database_replication_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "denyMaintenancePeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deny maintenance periods"]
        pub deny_maintenance_periods:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DenyMaintenancePeriod>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insightsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Insights configuration, for now relevant only for Postgres."]
        pub insights_config: ::std::option::Option<::std::boxed::Box<InsightsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled for Second Generation instances."]
        pub ip_configuration: ::std::option::Option<::std::boxed::Box<IpConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#settings*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location preference settings. This allows the instance to be located as near as possible to either an App Engine app or Compute Engine zone for better performance. App Engine co-location was only applicable to First Generation instances."]
        pub location_preference: ::std::option::Option<::std::boxed::Box<LocationPreference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenanceWindow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maintenance window for this instance. This specifies when the instance can be restarted for maintenance purposes."]
        pub maintenance_window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pricingPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pricing plan for this instance. This can be either *PER_USE* or *PACKAGE*. Only *PER_USE* is supported for Second Generation instances."]
        pub pricing_plan: ::std::option::Option<SettingsPricingPlanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of replication this instance uses. This can be either *ASYNCHRONOUS* or *SYNCHRONOUS*. (Deprecated_ This property was only applicable to First Generation instances."]
        pub replication_type: ::std::option::Option<SettingsReplicationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingsVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of instance settings. This is a required field for update method to make sure concurrent updates are handled properly. During update, use the most recent settingsVersion value for this instance and do not try to update this value."]
        pub settings_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageAutoResize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration to increase storage size automatically. The default value is true."]
        pub storage_auto_resize: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageAutoResizeLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
        pub storage_auto_resize_limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tier (or machine type) for this instance, for example *db-n1-standard-1* (MySQL instances) or *db-custom-1-3840* (PostgreSQL instances)."]
        pub tier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided labels, represented as a dictionary where each label is a single key value pair."]
        pub user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl Settings {
        pub fn builder() -> SettingsBuilder {
            SettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: *ALWAYS*: The instance is on, and remains so even in the absence of connection requests. *NEVER*: The instance is off; it is not activated, even if a connection request arrives."]
    pub enum SettingsActivationPolicyEnum {
        #[serde(rename = "SQL_ACTIVATION_POLICY_UNSPECIFIED")]
        #[doc = "Unknown activation plan."]
        SqlActivationPolicyUnspecified,
        #[serde(rename = "ALWAYS")]
        #[doc = "The instance is always up and running."]
        Always,
        #[serde(rename = "NEVER")]
        #[doc = "The instance never starts."]
        Never,
        #[serde(rename = "ON_DEMAND")]
        #[doc = "The instance starts upon receiving requests."]
        OnDemand,
    }
    impl ::std::default::Default for SettingsActivationPolicyEnum {
        fn default() -> Self {
            Self::SqlActivationPolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Availability type. Potential values: *ZONAL*: The instance serves data from only one zone. Outages in that zone affect data accessibility. *REGIONAL*: The instance can serve data from more than one zone in a region (it is highly available). For more information, see Overview of the High Availability Configuration."]
    pub enum SettingsAvailabilityTypeEnum {
        #[serde(rename = "SQL_AVAILABILITY_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown Availability type."]
        SqlAvailabilityTypeUnspecified,
        #[serde(rename = "ZONAL")]
        #[doc = "Zonal available instance."]
        Zonal,
        #[serde(rename = "REGIONAL")]
        #[doc = "Regional available instance."]
        Regional,
    }
    impl ::std::default::Default for SettingsAvailabilityTypeEnum {
        fn default() -> Self {
            Self::SqlAvailabilityTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of data disk: PD_SSD (default) or PD_HDD. Not used for First Generation instances."]
    pub enum SettingsDataDiskTypeEnum {
        #[serde(rename = "SQL_DATA_DISK_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown data disk type."]
        SqlDataDiskTypeUnspecified,
        #[serde(rename = "PD_SSD")]
        #[doc = "An SSD data disk."]
        PdSsd,
        #[serde(rename = "PD_HDD")]
        #[doc = "An HDD data disk."]
        PdHdd,
        #[serde(rename = "OBSOLETE_LOCAL_SSD")]
        #[doc = "This field is deprecated and will be removed from a future version of the API."]
        ObsoleteLocalSsd,
    }
    impl ::std::default::Default for SettingsDataDiskTypeEnum {
        fn default() -> Self {
            Self::SqlDataDiskTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The pricing plan for this instance. This can be either *PER_USE* or *PACKAGE*. Only *PER_USE* is supported for Second Generation instances."]
    pub enum SettingsPricingPlanEnum {
        #[serde(rename = "SQL_PRICING_PLAN_UNSPECIFIED")]
        #[doc = "This is an unknown pricing plan for this instance."]
        SqlPricingPlanUnspecified,
        #[serde(rename = "PACKAGE")]
        #[doc = "The instance is billed at a monthly flat rate."]
        Package,
        #[serde(rename = "PER_USE")]
        #[doc = "The instance is billed per usage."]
        PerUse,
    }
    impl ::std::default::Default for SettingsPricingPlanEnum {
        fn default() -> Self {
            Self::SqlPricingPlanUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of replication this instance uses. This can be either *ASYNCHRONOUS* or *SYNCHRONOUS*. (Deprecated_ This property was only applicable to First Generation instances."]
    pub enum SettingsReplicationTypeEnum {
        #[serde(rename = "SQL_REPLICATION_TYPE_UNSPECIFIED")]
        #[doc = "This is an unknown replication type for a Cloud SQL instance."]
        SqlReplicationTypeUnspecified,
        #[serde(rename = "SYNCHRONOUS")]
        #[doc = "The synchronous replication mode for First Generation instances. It is the default value."]
        Synchronous,
        #[serde(rename = "ASYNCHRONOUS")]
        #[doc = "The asynchronous replication mode for First Generation instances. It provides a slight performance gain, but if an outage occurs while this option is set to asynchronous, you can lose up to a few seconds of updates to your data."]
        Asynchronous,
    }
    impl ::std::default::Default for SettingsReplicationTypeEnum {
        fn default() -> Self {
            Self::SqlReplicationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Active Directory configuration, relevant only for Cloud SQL for SQL Server."]
    pub struct SqlActiveDirectoryConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the domain (e.g., mydomain.com)."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always sql#activeDirectoryConfig."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl SqlActiveDirectoryConfig {
        pub fn builder() -> SqlActiveDirectoryConfigBuilder {
            SqlActiveDirectoryConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "External primary instance migration setting error."]
    pub struct SqlExternalSyncSettingError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the error encountered."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#migrationSettingError*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the specific error that occurred."]
        pub _type: ::std::option::Option<SqlExternalSyncSettingErrorTypeEnum>,
    }
    impl SqlExternalSyncSettingError {
        pub fn builder() -> SqlExternalSyncSettingErrorBuilder {
            SqlExternalSyncSettingErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Identifies the specific error that occurred."]
    pub enum SqlExternalSyncSettingErrorTypeEnum {
        #[serde(rename = "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED")]
        #[doc = ""]
        SqlExternalSyncSettingErrorTypeUnspecified,
        #[serde(rename = "CONNECTION_FAILURE")]
        #[doc = ""]
        ConnectionFailure,
        #[serde(rename = "BINLOG_NOT_ENABLED")]
        #[doc = ""]
        BinlogNotEnabled,
        #[serde(rename = "INCOMPATIBLE_DATABASE_VERSION")]
        #[doc = ""]
        IncompatibleDatabaseVersion,
        #[serde(rename = "REPLICA_ALREADY_SETUP")]
        #[doc = ""]
        ReplicaAlreadySetup,
        #[serde(rename = "INSUFFICIENT_PRIVILEGE")]
        #[doc = ""]
        InsufficientPrivilege,
        #[serde(rename = "UNSUPPORTED_MIGRATION_TYPE")]
        #[doc = "Unsupported migration type."]
        UnsupportedMigrationType,
        #[serde(rename = "NO_PGLOGICAL_INSTALLED")]
        #[doc = "No pglogical extension installed on databases, applicable for postgres."]
        NoPglogicalInstalled,
        #[serde(rename = "PGLOGICAL_NODE_ALREADY_EXISTS")]
        #[doc = "pglogical node already exists on databases, applicable for postgres."]
        PglogicalNodeAlreadyExists,
        #[serde(rename = "INVALID_WAL_LEVEL")]
        #[doc = "The value of parameter wal_level is not set to logical."]
        InvalidWalLevel,
        #[serde(rename = "INVALID_SHARED_PRELOAD_LIBRARY")]
        #[doc = "The value of parameter shared_preload_libraries does not include pglogical."]
        InvalidSharedPreloadLibrary,
        #[serde(rename = "INSUFFICIENT_MAX_REPLICATION_SLOTS")]
        #[doc = "The value of parameter max_replication_slots is not sufficient."]
        InsufficientMaxReplicationSlots,
        #[serde(rename = "INSUFFICIENT_MAX_WAL_SENDERS")]
        #[doc = "The value of parameter max_wal_senders is not sufficient."]
        InsufficientMaxWalSenders,
        #[serde(rename = "INSUFFICIENT_MAX_WORKER_PROCESSES")]
        #[doc = "The value of parameter max_worker_processes is not sufficient."]
        InsufficientMaxWorkerProcesses,
        #[serde(rename = "UNSUPPORTED_EXTENSIONS")]
        #[doc = "Extensions installed are either not supported or having unsupported versions"]
        UnsupportedExtensions,
        #[serde(rename = "INVALID_RDS_LOGICAL_REPLICATION")]
        #[doc = "The value of parameter rds.logical_replication is not set to 1."]
        InvalidRdsLogicalReplication,
        #[serde(rename = "INVALID_LOGGING_SETUP")]
        #[doc = "The primary instance logging setup doesn't allow EM sync."]
        InvalidLoggingSetup,
        #[serde(rename = "INVALID_DB_PARAM")]
        #[doc = "The primary instance database parameter setup doesn't allow EM sync."]
        InvalidDbParam,
        #[serde(rename = "UNSUPPORTED_GTID_MODE")]
        #[doc = "The gtid_mode is not supported, applicable for MySQL."]
        UnsupportedGtidMode,
        #[serde(rename = "SQLSERVER_AGENT_NOT_RUNNING")]
        #[doc = "SQL Server Agent is not running."]
        SqlserverAgentNotRunning,
        #[serde(rename = "UNSUPPORTED_TABLE_DEFINITION")]
        #[doc = "The table definition is not support due to missing primary key or replica identity, applicable for postgres."]
        UnsupportedTableDefinition,
        #[serde(rename = "UNSUPPORTED_DEFINER")]
        #[doc = "The customer has a definer that will break EM setup."]
        UnsupportedDefiner,
    }
    impl ::std::default::Default for SqlExternalSyncSettingErrorTypeEnum {
        fn default() -> Self {
            Self::SqlExternalSyncSettingErrorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reschedule options for maintenance windows."]
    pub struct SqlInstancesRescheduleMaintenanceRequestBody {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reschedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the reschedule the user wants."]
        pub reschedule: ::std::option::Option<::std::boxed::Box<Reschedule>>,
    }
    impl SqlInstancesRescheduleMaintenanceRequestBody {
        pub fn builder() -> SqlInstancesRescheduleMaintenanceRequestBodyBuilder {
            SqlInstancesRescheduleMaintenanceRequestBodyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instance verify external sync settings response."]
    pub struct SqlInstancesVerifyExternalSyncSettingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of migration violations."]
        pub errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SqlExternalSyncSettingError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#migrationSettingErrorList*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl SqlInstancesVerifyExternalSyncSettingsResponse {
        pub fn builder() -> SqlInstancesVerifyExternalSyncSettingsResponseBuilder {
            SqlInstancesVerifyExternalSyncSettingsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Any scheduled maintenancce for this instance."]
    pub struct SqlScheduledMaintenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canDefer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub can_defer: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReschedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the scheduled maintenance can be rescheduled."]
        pub can_reschedule: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of any upcoming scheduled maintenance for this instance."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl SqlScheduledMaintenance {
        pub fn builder() -> SqlScheduledMaintenanceBuilder {
            SqlScheduledMaintenanceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Sql Server database on the Cloud SQL instance."]
    pub struct SqlServerDatabaseDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compatibilityLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of SQL Server with which the database is to be made compatible"]
        pub compatibility_level: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recoveryModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recovery model of a SQL Server database"]
        pub recovery_model: ::std::option::Option<::std::string::String>,
    }
    impl SqlServerDatabaseDetails {
        pub fn builder() -> SqlServerDatabaseDetailsBuilder {
            SqlServerDatabaseDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Sql Server user on the Cloud SQL instance."]
    pub struct SqlServerUserDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the user has been disabled"]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server roles for this user"]
        pub server_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl SqlServerUserDetails {
        pub fn builder() -> SqlServerUserDetailsBuilder {
            SqlServerUserDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCerts Resource"]
    pub struct SslCert {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM representation."]
        pub cert: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certSerialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serial number, as extracted from the certificate."]
        pub cert_serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User supplied name. Constrained to [a-zA-Z.-_ ]+."]
        pub common_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the certificate was created in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the certificate expires in RFC 3339 format, for example *2012-11-15T16:19:00.094Z*."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the database instance."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#sslCert*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha1Fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sha1 Fingerprint."]
        pub sha1_fingerprint: ::std::option::Option<::std::string::String>,
    }
    impl SslCert {
        pub fn builder() -> SslCertBuilder {
            SslCertBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCertDetail."]
    pub struct SslCertDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public information about the cert."]
        pub cert_info: ::std::option::Option<::std::boxed::Box<SslCert>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certPrivateKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The private key for the client cert, in pem format. Keep private in order to protect your security."]
        pub cert_private_key: ::std::option::Option<::std::string::String>,
    }
    impl SslCertDetail {
        pub fn builder() -> SslCertDetailBuilder {
            SslCertDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCerts create ephemeral certificate request."]
    pub struct SslCertsCreateEphemeralRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access_token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Access token to include in the signed certificate."]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "public_key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM encoded public key to include in the signed certificate."]
        pub public_key: ::std::option::Option<::std::string::String>,
    }
    impl SslCertsCreateEphemeralRequest {
        pub fn builder() -> SslCertsCreateEphemeralRequestBuilder {
            SslCertsCreateEphemeralRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCerts insert request."]
    pub struct SslCertsInsertRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User supplied name. Must be a distinct name from the other certificates for this instance."]
        pub common_name: ::std::option::Option<::std::string::String>,
    }
    impl SslCertsInsertRequest {
        pub fn builder() -> SslCertsInsertRequestBuilder {
            SslCertsInsertRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCert insert response."]
    pub struct SslCertsInsertResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new client certificate and private key."]
        pub client_cert: ::std::option::Option<::std::boxed::Box<SslCertDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#sslCertsInsert*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation to track the ssl certs insert request."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverCaCert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server Certificate Authority's certificate. If this is missing you can force a new one to be generated by calling resetSslConfig method on instances resource."]
        pub server_ca_cert: ::std::option::Option<::std::boxed::Box<SslCert>>,
    }
    impl SslCertsInsertResponse {
        pub fn builder() -> SslCertsInsertResponseBuilder {
            SslCertsInsertResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SslCerts list response."]
    pub struct SslCertsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of client certificates for the instance."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SslCert>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#sslCertsList*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl SslCertsListResponse {
        pub fn builder() -> SslCertsListResponseBuilder {
            SslCertsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Google Cloud SQL service tier resource."]
    pub struct Tier {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "DiskQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum disk size of this tier in bytes."]
        pub disk_quota: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "RAM")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum RAM usage of this tier in bytes."]
        pub ram: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#tier*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The applicable regions for this tier."]
        pub region: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for the machine type, for example, db-n1-standard-1. For related information, see Pricing."]
        pub tier: ::std::option::Option<::std::string::String>,
    }
    impl Tier {
        pub fn builder() -> TierBuilder {
            TierBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tiers list response."]
    pub struct TiersListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of tiers."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tier>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#tiersList*."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl TiersListResponse {
        pub fn builder() -> TiersListResponseBuilder {
            TiersListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Database Instance truncate log context."]
    pub struct TruncateLogContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#truncateLogContext*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of log to truncate. Valid values are *MYSQL_GENERAL_TABLE* and *MYSQL_SLOW_TABLE*."]
        pub log_type: ::std::option::Option<::std::string::String>,
    }
    impl TruncateLogContext {
        pub fn builder() -> TruncateLogContextBuilder {
            TruncateLogContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Cloud SQL user resource."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated and will be removed from a future version of the API."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The host name from which the user can connect. For *insert* operations, host defaults to an empty string. For *update* operations, host is specified as part of the request URL. The host name cannot be updated after insertion."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for *update* since it is already specified on the URL."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#user*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user in the Cloud SQL instance. Can be omitted for *update* since it is already specified in the URL."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password for the user."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "project")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for *update* since it is already specified on the URL."]
        pub project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sqlserverUserDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sqlserver_user_details: ::std::option::Option<::std::boxed::Box<SqlServerUserDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type."]
        pub _type: ::std::option::Option<UserTypeEnum>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type."]
    pub enum UserTypeEnum {
        #[serde(rename = "BUILT_IN")]
        #[doc = "The database's built-in user type."]
        BuiltIn,
        #[serde(rename = "CLOUD_IAM_USER")]
        #[doc = "Cloud IAM user."]
        CloudIamUser,
        #[serde(rename = "CLOUD_IAM_SERVICE_ACCOUNT")]
        #[doc = "Cloud IAM service account."]
        CloudIamServiceAccount,
    }
    impl ::std::default::Default for UserTypeEnum {
        fn default() -> Self {
            Self::BuiltIn
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User list response."]
    pub struct UsersListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of user resources in the instance."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<User>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is always *sql#usersList*."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl UsersListResponse {
        pub fn builder() -> UsersListResponseBuilder {
            UsersListResponseBuilder::default()
        }
    }
}
