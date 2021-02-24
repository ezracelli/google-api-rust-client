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
    pub mod enterprises {
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
                    #[serde(rename = "agreementAccepted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This feature is not generally available yet. Whether the managed Google Play Agreement is presented and agreed."]
                    pub agreement_accepted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "enterpriseToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The enterprise token appended to the callback URL."]
                    pub enterprise_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projectId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
                    pub project_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "signupUrlName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of the SignupUrl used to sign up for the enterprise."]
                    pub signup_url_name: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This feature is not generally available yet. The requested page size. The actual page size may be fixed to a min or max value."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This feature is not generally available yet. A token identifying a page of results returned by the server."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projectId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. This feature is not generally available yet. The ID of the Cloud project of the EMM the enterprises belongs to."]
                    pub project_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This feature is not generally available yet. View that specify that partial response should be returned."]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "This feature is not generally available yet. View that specify that partial response should be returned."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "ENTERPRISE_VIEW_UNSPECIFIED")]
                    #[doc = "This feature is not generally available yet. The API will default to the BASIC view for the List method."]
                    EnterpriseViewUnspecified,
                    #[serde(rename = "BASIC")]
                    #[doc = "This feature is not generally available yet. Includes name and enterprise_display_name fields."]
                    Basic,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::EnterpriseViewUnspecified
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
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod applications {
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
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The preferred language for localized application info, as a BCP47 tag (e.g. \"en-US\", \"de\"). If not specified the default language of the application will be used."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod devices {
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "wipeDataFlags")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional flags that control the device wiping behavior."]
                            pub wipe_data_flags:
                                ::std::option::Option<QueryParametersWipeDataFlagsEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "wipeReasonMessage")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A short message displayed to the user before wiping the work profile on personal devices. This has no effect on company owned devices. The maximum message length is 200 characters."]
                            pub wipe_reason_message: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Optional flags that control the device wiping behavior."]
                        pub enum QueryParametersWipeDataFlagsEnum {
                            #[serde(rename = "WIPE_DATA_FLAG_UNSPECIFIED")]
                            #[doc = "This value is ignored."]
                            WipeDataFlagUnspecified,
                            #[serde(rename = "PRESERVE_RESET_PROTECTION_DATA")]
                            #[doc = "Preserve the factory reset protection data on the device."]
                            PreserveResetProtectionData,
                            #[serde(rename = "WIPE_EXTERNAL_STORAGE")]
                            #[doc = "Additionally wipe the device's external storage (such as SD cards)."]
                            WipeExternalStorage,
                        }
                        impl ::std::default::Default for QueryParametersWipeDataFlagsEnum {
                            fn default() -> Self {
                                Self::WipeDataFlagUnspecified
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results returned by the server."]
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list filter."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page token."]
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
            pub mod policies {
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
                            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results returned by the server."]
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod web_apps {
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
                            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results returned by the server."]
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
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
    pub mod signup_urls {
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
                    #[serde(rename = "callbackUrl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The callback URL that the admin will be redirected to after successfully creating an enterprise. Before redirecting there the system will add a query parameter to this URL named enterpriseToken which will contain an opaque token to be used for the create enterprise request. The URL will be parsed then reformatted in order to add the enterpriseToken parameter, so there may be some minor formatting changes."]
                    pub callback_url: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projectId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
                    pub project_id: ::std::option::Option<::std::string::String>,
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
    #[doc = "Security policies set to the most secure values by default. To maintain the security posture of a device, we don't recommend overriding any of the default values."]
    pub struct AdvancedSecurityOverrides {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonCriteriaMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC). Enabling Common Criteria Mode increases certain security components on a device, including AES-GCM encryption of Bluetooth Long Term Keys, and Wi-Fi configuration stores.Warning: Common Criteria Mode enforces a strict security model typically only required for IT products used in national security systems and other highly sensitive organizations. Standard device use may be affected. Only enabled if required."]
        pub common_criteria_mode:
            ::std::option::Option<AdvancedSecurityOverridesCommonCriteriaModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "untrustedAppsPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy for untrusted apps (apps from unknown sources) enforced on the device. Replaces install_unknown_sources_allowed (deprecated)."]
        pub untrusted_apps_policy:
            ::std::option::Option<AdvancedSecurityOverridesUntrustedAppsPolicyEnum>,
    }
    impl AdvancedSecurityOverrides {
        pub fn builder() -> AdvancedSecurityOverridesBuilder {
            AdvancedSecurityOverridesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC). Enabling Common Criteria Mode increases certain security components on a device, including AES-GCM encryption of Bluetooth Long Term Keys, and Wi-Fi configuration stores.Warning: Common Criteria Mode enforces a strict security model typically only required for IT products used in national security systems and other highly sensitive organizations. Standard device use may be affected. Only enabled if required."]
    pub enum AdvancedSecurityOverridesCommonCriteriaModeEnum {
        #[serde(rename = "COMMON_CRITERIA_MODE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to COMMON_CRITERIA_MODE_DISABLED."]
        CommonCriteriaModeUnspecified,
        #[serde(rename = "COMMON_CRITERIA_MODE_DISABLED")]
        #[doc = "Default. Disables Common Criteria Mode."]
        CommonCriteriaModeDisabled,
        #[serde(rename = "COMMON_CRITERIA_MODE_ENABLED")]
        #[doc = "Enables Common Criteria Mode."]
        CommonCriteriaModeEnabled,
    }
    impl ::std::default::Default for AdvancedSecurityOverridesCommonCriteriaModeEnum {
        fn default() -> Self {
            Self::CommonCriteriaModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The policy for untrusted apps (apps from unknown sources) enforced on the device. Replaces install_unknown_sources_allowed (deprecated)."]
    pub enum AdvancedSecurityOverridesUntrustedAppsPolicyEnum {
        #[serde(rename = "UNTRUSTED_APPS_POLICY_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to DISALLOW_INSTALL."]
        UntrustedAppsPolicyUnspecified,
        #[serde(rename = "DISALLOW_INSTALL")]
        #[doc = "Default. Disallow untrusted app installs on entire device."]
        DisallowInstall,
        #[serde(rename = "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY")]
        #[doc = "For devices with work profiles, allow untrusted app installs in the device's personal profile only."]
        AllowInstallInPersonalProfileOnly,
        #[serde(rename = "ALLOW_INSTALL_DEVICE_WIDE")]
        #[doc = "Allow untrusted app installs on entire device."]
        AllowInstallDeviceWide,
    }
    impl ::std::default::Default for AdvancedSecurityOverridesUntrustedAppsPolicyEnum {
        fn default() -> Self {
            Self::UntrustedAppsPolicyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for an always-on VPN connection."]
    pub struct AlwaysOnVpnPackage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lockdownEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disallows networking when the VPN is not connected."]
        pub lockdown_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the VPN app."]
        pub package_name: ::std::option::Option<::std::string::String>,
    }
    impl AlwaysOnVpnPackage {
        pub fn builder() -> AlwaysOnVpnPackageBuilder {
            AlwaysOnVpnPackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A compliance rule condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement. There can only be one rule with this type of condition per policy."]
    pub struct ApiLevelCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minApiLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum desired Android Framework API level. If the device doesn't meet the minimum requirement, this condition is satisfied. Must be greater than zero."]
        pub min_api_level: ::std::option::Option<::std::primitive::i64>,
    }
    impl ApiLevelCondition {
        pub fn builder() -> ApiLevelConditionBuilder {
            ApiLevelConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Id to name association of a app track."]
    pub struct AppTrackInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackAlias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The track name associated with the trackId, set in the Play Console. The name is modifiable from Play Console."]
        pub track_alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unmodifiable unique track identifier, taken from the releaseTrackId in the URL of the Play Console page that displays the app’s track information."]
        pub track_id: ::std::option::Option<::std::string::String>,
    }
    impl AppTrackInfo {
        pub fn builder() -> AppTrackInfoBuilder {
            AppTrackInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an app."]
    pub struct Application {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appTracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application tracks visible to the enterprise."]
        pub app_tracks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppTrackInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of managed properties available to be pre-configured for the app."]
        pub managed_properties:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedProperty>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the app in the form enterprises/{enterpriseId}/applications/{package_name}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The permissions required by the app."]
        pub permissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationPermission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the app. Localized."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Application {
        pub fn builder() -> ApplicationBuilder {
            ApplicationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An app-related event."]
    pub struct ApplicationEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the event."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App event type."]
        pub event_type: ::std::option::Option<ApplicationEventEventTypeEnum>,
    }
    impl ApplicationEvent {
        pub fn builder() -> ApplicationEventBuilder {
            ApplicationEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "App event type."]
    pub enum ApplicationEventEventTypeEnum {
        #[serde(rename = "APPLICATION_EVENT_TYPE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        ApplicationEventTypeUnspecified,
        #[serde(rename = "INSTALLED")]
        #[doc = "The app was installed."]
        Installed,
        #[serde(rename = "CHANGED")]
        #[doc = "The app was changed, for example, a component was enabled or disabled."]
        Changed,
        #[serde(rename = "DATA_CLEARED")]
        #[doc = "The app data was cleared."]
        DataCleared,
        #[serde(rename = "REMOVED")]
        #[doc = "The app was removed."]
        Removed,
        #[serde(rename = "REPLACED")]
        #[doc = "A new version of the app has been installed, replacing the old version."]
        Replaced,
        #[serde(rename = "RESTARTED")]
        #[doc = "The app was restarted."]
        Restarted,
        #[serde(rename = "PINNED")]
        #[doc = "The app was pinned to the foreground."]
        Pinned,
        #[serde(rename = "UNPINNED")]
        #[doc = "The app was unpinned."]
        Unpinned,
    }
    impl ::std::default::Default for ApplicationEventEventTypeEnum {
        fn default() -> Self {
            Self::ApplicationEventTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A permission required by the app."]
    pub struct ApplicationPermission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A longer description of the permission, providing more detail on what it affects. Localized."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the permission. Localized."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque string uniquely identifying the permission. Not localized."]
        pub permission_id: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationPermission {
        pub fn builder() -> ApplicationPermissionBuilder {
            ApplicationPermissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policy for an individual app."]
    pub struct ApplicationPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessibleTrackIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the app’s track IDs that a device belonging to the enterprise can access. If the list contains multiple track IDs, devices receive the latest version among all accessible tracks. If the list contains no track IDs, devices only have access to the app’s production track. More details about each track are available in AppTrackInfo."]
        pub accessible_track_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoUpdateMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This feature is not generally available yet."]
        pub auto_update_mode: ::std::option::Option<ApplicationPolicyAutoUpdateModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectedWorkAndPersonalApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether the app can communicate with itself across a device’s work and personal profiles, subject to user consent."]
        pub connected_work_and_personal_app:
            ::std::option::Option<ApplicationPolicyConnectedWorkAndPersonalAppEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultPermissionPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps."]
        pub default_permission_policy:
            ::std::option::Option<ApplicationPolicyDefaultPermissionPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedScopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scopes delegated to the app from Android Device Policy."]
        pub delegated_scopes:
            ::std::option::Option<::std::vec::Vec<ApplicationPolicyDelegatedScopesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app is disabled. When disabled, the app data is still preserved."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of installation to perform."]
        pub install_type: ::std::option::Option<ApplicationPolicyInstallTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lockTaskAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the app is allowed to lock itself in full-screen mode. DEPRECATED. Use InstallType KIOSK or kioskCustomLauncherEnabled to to configure a dedicated device."]
        pub lock_task_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Managed configuration applied to the app. The format for the configuration is dictated by the ManagedProperty values supported by the app. Each field name in the managed configuration must match the key field of the ManagedProperty. The field value must be compatible with the type of the ManagedProperty: *type* *JSON value* BOOL true or false STRING string INTEGER number CHOICE string MULTISELECT array of strings HIDDEN string BUNDLE_ARRAY array of objects "]
        pub managed_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedConfigurationTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The managed configurations template for the app, saved from the managed configurations iframe. This field is ignored if managed_configuration is set."]
        pub managed_configuration_template:
            ::std::option::Option<::std::boxed::Box<ManagedConfigurationTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumVersionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum version of the app that runs on the device. If set, the device attempts to update the app to at least this version code. If the app is not up-to-date, the device will contain a NonComplianceDetail with non_compliance_reason set to APP_NOT_UPDATED. The app must already be published to Google Play with a version code greater than or equal to this value. At most 20 apps may specify a minimum version code per policy."]
        pub minimum_version_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the app. For example, com.google.android.youtube for the YouTube app."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionGrants")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit permission grants or denials for the app. These values override the default_permission_policy and permission_grants which apply to all apps."]
        pub permission_grants:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PermissionGrant>>>,
    }
    impl ApplicationPolicy {
        pub fn builder() -> ApplicationPolicyBuilder {
            ApplicationPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This feature is not generally available yet."]
    pub enum ApplicationPolicyAutoUpdateModeEnum {
        #[serde(rename = "AUTO_UPDATE_MODE_UNSPECIFIED")]
        #[doc = "This feature is not generally available yet."]
        AutoUpdateModeUnspecified,
        #[serde(rename = "AUTO_UPDATE_DEFAULT")]
        #[doc = "This feature is not generally available yet."]
        AutoUpdateDefault,
        #[serde(rename = "AUTO_UPDATE_POSTPONED")]
        #[doc = "This feature is not generally available yet."]
        AutoUpdatePostponed,
        #[serde(rename = "AUTO_UPDATE_HIGH_PRIORITY")]
        #[doc = "This feature is not generally available yet."]
        AutoUpdateHighPriority,
    }
    impl ::std::default::Default for ApplicationPolicyAutoUpdateModeEnum {
        fn default() -> Self {
            Self::AutoUpdateModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls whether the app can communicate with itself across a device’s work and personal profiles, subject to user consent."]
    pub enum ApplicationPolicyConnectedWorkAndPersonalAppEnum {
        #[serde(rename = "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to CONNECTED_WORK_AND_PERSONAL_APPS_DISALLOWED."]
        ConnectedWorkAndPersonalAppUnspecified,
        #[serde(rename = "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED")]
        #[doc = "Default. Prevents the app from communicating cross-profile."]
        ConnectedWorkAndPersonalAppDisallowed,
        #[serde(rename = "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED")]
        #[doc = "Allows the app to communicate across profiles after receiving user consent."]
        ConnectedWorkAndPersonalAppAllowed,
    }
    impl ::std::default::Default for ApplicationPolicyConnectedWorkAndPersonalAppEnum {
        fn default() -> Self {
            Self::ConnectedWorkAndPersonalAppUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps."]
    pub enum ApplicationPolicyDefaultPermissionPolicyEnum {
        #[serde(rename = "PERMISSION_POLICY_UNSPECIFIED")]
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[serde(rename = "PROMPT")]
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[serde(rename = "GRANT")]
        #[doc = "Automatically grant a permission."]
        Grant,
        #[serde(rename = "DENY")]
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl ::std::default::Default for ApplicationPolicyDefaultPermissionPolicyEnum {
        fn default() -> Self {
            Self::PermissionPolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ApplicationPolicyDelegatedScopesEnum {
        #[serde(rename = "DELEGATED_SCOPE_UNSPECIFIED")]
        #[doc = "No delegation scope specified."]
        DelegatedScopeUnspecified,
        #[serde(rename = "CERT_INSTALL")]
        #[doc = "Grants access to certificate installation and management."]
        CertInstall,
        #[serde(rename = "MANAGED_CONFIGURATIONS")]
        #[doc = "Grants access to managed configurations management."]
        ManagedConfigurations,
        #[serde(rename = "BLOCK_UNINSTALL")]
        #[doc = "Grants access to blocking uninstallation."]
        BlockUninstall,
        #[serde(rename = "PERMISSION_GRANT")]
        #[doc = "Grants access to permission policy and permission grant state."]
        PermissionGrant,
        #[serde(rename = "PACKAGE_ACCESS")]
        #[doc = "Grants access to package access state."]
        PackageAccess,
        #[serde(rename = "ENABLE_SYSTEM_APP")]
        #[doc = "Grants access for enabling system apps."]
        EnableSystemApp,
    }
    impl ::std::default::Default for ApplicationPolicyDelegatedScopesEnum {
        fn default() -> Self {
            Self::DelegatedScopeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of installation to perform."]
    pub enum ApplicationPolicyInstallTypeEnum {
        #[serde(rename = "INSTALL_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to AVAILABLE."]
        InstallTypeUnspecified,
        #[serde(rename = "PREINSTALLED")]
        #[doc = "The app is automatically installed and can be removed by the user."]
        Preinstalled,
        #[serde(rename = "FORCE_INSTALLED")]
        #[doc = "The app is automatically installed and can't be removed by the user."]
        ForceInstalled,
        #[serde(rename = "BLOCKED")]
        #[doc = "The app is blocked and can't be installed. If the app was installed under a previous policy, it will be uninstalled."]
        Blocked,
        #[serde(rename = "AVAILABLE")]
        #[doc = "The app is available to install."]
        Available,
        #[serde(rename = "REQUIRED_FOR_SETUP")]
        #[doc = "The app is automatically installed and can't be removed by the user and will prevent setup from completion until installation is complete."]
        RequiredForSetup,
        #[serde(rename = "KIOSK")]
        #[doc = "The app is automatically installed in kiosk mode: it's set as the preferred home intent and whitelisted for lock task mode. Device setup won't complete until the app is installed. After installation, users won't be able to remove the app. You can only set this installType for one app per policy. When this is present in the policy, status bar will be automatically disabled."]
        Kiosk,
    }
    impl ::std::default::Default for ApplicationPolicyInstallTypeEnum {
        fn default() -> Self {
            Self::InstallTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information reported about an installed app."]
    pub struct ApplicationReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of the package."]
        pub application_source: ::std::option::Option<ApplicationReportApplicationSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the app."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of app events. The most recent 20 events are stored in the list."]
        pub events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installerPackageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the app that installed this app."]
        pub installer_package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyedAppStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of keyed app states reported by the app."]
        pub keyed_app_states:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyedAppState>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Package name of the app."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageSha256Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SHA-256 hash of the app's APK file, which can be used to verify the app hasn't been modified. Each byte of the hash value is represented as a two-digit hexadecimal number."]
        pub package_sha256_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signingKeyCertFingerprints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SHA-1 hash of each android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the app package. Each byte of each hash value is represented as a two-digit hexadecimal number."]
        pub signing_key_cert_fingerprints:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application state."]
        pub state: ::std::option::Option<ApplicationReportStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app version code, which can be used to determine whether one version is more recent than another."]
        pub version_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app version as displayed to the user."]
        pub version_name: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationReport {
        pub fn builder() -> ApplicationReportBuilder {
            ApplicationReportBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source of the package."]
    pub enum ApplicationReportApplicationSourceEnum {
        #[serde(rename = "APPLICATION_SOURCE_UNSPECIFIED")]
        #[doc = "The app was sideloaded from an unspecified source."]
        ApplicationSourceUnspecified,
        #[serde(rename = "SYSTEM_APP_FACTORY_VERSION")]
        #[doc = "This is a system app from the device's factory image."]
        SystemAppFactoryVersion,
        #[serde(rename = "SYSTEM_APP_UPDATED_VERSION")]
        #[doc = "This is an updated system app."]
        SystemAppUpdatedVersion,
        #[serde(rename = "INSTALLED_FROM_PLAY_STORE")]
        #[doc = "The app was installed from the Google Play Store."]
        InstalledFromPlayStore,
    }
    impl ::std::default::Default for ApplicationReportApplicationSourceEnum {
        fn default() -> Self {
            Self::ApplicationSourceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Application state."]
    pub enum ApplicationReportStateEnum {
        #[serde(rename = "APPLICATION_STATE_UNSPECIFIED")]
        #[doc = "App state is unspecified"]
        ApplicationStateUnspecified,
        #[serde(rename = "REMOVED")]
        #[doc = "App was removed from the device"]
        Removed,
        #[serde(rename = "INSTALLED")]
        #[doc = "App is installed on the device"]
        Installed,
    }
    impl ::std::default::Default for ApplicationReportStateEnum {
        fn default() -> Self {
            Self::ApplicationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings controlling the behavior of application reports."]
    pub struct ApplicationReportingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeRemovedApps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether removed apps are included in application reports."]
        pub include_removed_apps: ::std::option::Option<::std::primitive::bool>,
    }
    impl ApplicationReportingSettings {
        pub fn builder() -> ApplicationReportingSettingsBuilder {
            ApplicationReportingSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An action to block access to apps and data on a fully managed device or in a work profile. This action also triggers a device or work profile to displays a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified."]
    pub struct BlockAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockAfterDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days the policy is non-compliant before the device or work profile is blocked. To block access immediately, set to 0. blockAfterDays must be less than wipeAfterDays."]
        pub block_after_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the scope of this BlockAction. Only applicable to devices that are company-owned."]
        pub block_scope: ::std::option::Option<BlockActionBlockScopeEnum>,
    }
    impl BlockAction {
        pub fn builder() -> BlockActionBuilder {
            BlockActionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the scope of this BlockAction. Only applicable to devices that are company-owned."]
    pub enum BlockActionBlockScopeEnum {
        #[serde(rename = "BLOCK_SCOPE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to BLOCK_SCOPE_WORK_PROFILE."]
        BlockScopeUnspecified,
        #[serde(rename = "BLOCK_SCOPE_WORK_PROFILE")]
        #[doc = "Block action is only applied to apps in the work profile. Apps in the personal profile are unaffected."]
        BlockScopeWorkProfile,
        #[serde(rename = "BLOCK_SCOPE_DEVICE")]
        #[doc = "Block action is applied to the entire device, including apps in the personal profile."]
        BlockScopeDevice,
    }
    impl ::std::default::Default for BlockActionBlockScopeEnum {
        fn default() -> Self {
            Self::BlockScopeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule for automatically choosing a private key and certificate to authenticate the device to a server."]
    pub struct ChoosePrivateKeyRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package names for which outgoing requests are subject to this rule. If no package names are specified, then the rule applies to all packages. For each package name listed, the rule applies to that package and all other packages that shared the same Android UID. The SHA256 hash of the signing key signatures of each package_name will be verified against those provided by Play"]
        pub package_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateKeyAlias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alias of the private key to be used."]
        pub private_key_alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlPattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL pattern to match against the URL of the outgoing request. The pattern may contain asterisk (*) wildcards. Any URL is matched if unspecified."]
        pub url_pattern: ::std::option::Option<::std::string::String>,
    }
    impl ChoosePrivateKeyRule {
        pub fn builder() -> ChoosePrivateKeyRuleBuilder {
            ChoosePrivateKeyRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A command."]
    pub struct Command {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp at which the command was created. The timestamp is automatically generated by the server."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration for which the command is valid. The command will expire if not executed by the device during this time. The default duration if unspecified is ten minutes. There is no maximum duration."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller."]
        pub error_code: ::std::option::Option<CommandErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies the new password."]
        pub new_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetPasswordFlags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies flags."]
        pub reset_password_flags:
            ::std::option::Option<::std::vec::Vec<CommandResetPasswordFlagsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the command."]
        pub _type: ::std::option::Option<CommandTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the user that owns the device in the form enterprises/{enterpriseId}/users/{userId}. This is automatically generated by the server based on the device the command is sent to."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl Command {
        pub fn builder() -> CommandBuilder {
            CommandBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller."]
    pub enum CommandErrorCodeEnum {
        #[serde(rename = "COMMAND_ERROR_CODE_UNSPECIFIED")]
        #[doc = "There was no error."]
        CommandErrorCodeUnspecified,
        #[serde(rename = "UNKNOWN")]
        #[doc = "An unknown error occurred."]
        Unknown,
        #[serde(rename = "API_LEVEL")]
        #[doc = "The API level of the device does not support this command."]
        ApiLevel,
        #[serde(rename = "MANAGEMENT_MODE")]
        #[doc = "The management mode (profile owner, device owner, etc.) does not support the command."]
        ManagementMode,
        #[serde(rename = "INVALID_VALUE")]
        #[doc = "The command has an invalid parameter value."]
        InvalidValue,
        #[serde(rename = "UNSUPPORTED")]
        #[doc = "The device doesn't support the command. Updating Android Device Policy to the latest version may resolve the issue."]
        Unsupported,
    }
    impl ::std::default::Default for CommandErrorCodeEnum {
        fn default() -> Self {
            Self::CommandErrorCodeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CommandResetPasswordFlagsEnum {
        #[serde(rename = "RESET_PASSWORD_FLAG_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        ResetPasswordFlagUnspecified,
        #[serde(rename = "REQUIRE_ENTRY")]
        #[doc = "Don't allow other admins to change the password again until the user has entered it."]
        RequireEntry,
        #[serde(rename = "DO_NOT_ASK_CREDENTIALS_ON_BOOT")]
        #[doc = "Don't ask for user credentials on device boot."]
        DoNotAskCredentialsOnBoot,
        #[serde(rename = "LOCK_NOW")]
        #[doc = "Lock the device after password reset."]
        LockNow,
    }
    impl ::std::default::Default for CommandResetPasswordFlagsEnum {
        fn default() -> Self {
            Self::ResetPasswordFlagUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the command."]
    pub enum CommandTypeEnum {
        #[serde(rename = "COMMAND_TYPE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        CommandTypeUnspecified,
        #[serde(rename = "LOCK")]
        #[doc = "Lock the device, as if the lock screen timeout had expired."]
        Lock,
        #[serde(rename = "RESET_PASSWORD")]
        #[doc = "Reset the user's password."]
        ResetPassword,
        #[serde(rename = "REBOOT")]
        #[doc = "Reboot the device. Only supported on fully managed devices running Android 7.0 (API level 24) or higher."]
        Reboot,
        #[serde(rename = "RELINQUISH_OWNERSHIP")]
        #[doc = "Removes the work profile and all policies from a company-owned Android 8.0+ device, relinquishing the device for personal use. Apps and data associated with the personal profile(s) are preserved. The device will be deleted from the server after it acknowledges the command."]
        RelinquishOwnership,
    }
    impl ::std::default::Default for CommandTypeEnum {
        fn default() -> Self {
            Self::CommandTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC).This information is only available if statusReportingSettings.commonCriteriaModeEnabled is true in the device's policy."]
    pub struct CommonCriteriaModeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonCriteriaModeStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Common Criteria Mode is enabled."]
        pub common_criteria_mode_status:
            ::std::option::Option<CommonCriteriaModeInfoCommonCriteriaModeStatusEnum>,
    }
    impl CommonCriteriaModeInfo {
        pub fn builder() -> CommonCriteriaModeInfoBuilder {
            CommonCriteriaModeInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether Common Criteria Mode is enabled."]
    pub enum CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
        #[serde(rename = "COMMON_CRITERIA_MODE_STATUS_UNKNOWN")]
        #[doc = "Unknown status."]
        CommonCriteriaModeStatusUnknown,
        #[serde(rename = "COMMON_CRITERIA_MODE_DISABLED")]
        #[doc = "Common Criteria Mode is currently disabled."]
        CommonCriteriaModeDisabled,
        #[serde(rename = "COMMON_CRITERIA_MODE_ENABLED")]
        #[doc = "Common Criteria Mode is currently enabled."]
        CommonCriteriaModeEnabled,
    }
    impl ::std::default::Default for CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
        fn default() -> Self {
            Self::CommonCriteriaModeStatusUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule declaring which mitigating actions to take when a device is not compliant with its policy. For every rule, there is always an implicit mitigating action to set policy_compliant to false for the Device resource, and display a message on the device indicating that the device is not compliant with its policy. Other mitigating actions may optionally be taken as well, depending on the field values in the rule."]
    pub struct ComplianceRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiLevelCondition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement."]
        pub api_level_condition: ::std::option::Option<::std::boxed::Box<ApiLevelCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableApps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, the rule includes a mitigating action to disable apps so that the device is effectively disabled, but app data is preserved. If the device is running an app in locked task mode, the app will be closed and a UI showing the reason for non-compliance will be displayed."]
        pub disable_apps: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonComplianceDetailCondition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition which is satisfied if there exists any matching NonComplianceDetail for the device."]
        pub non_compliance_detail_condition:
            ::std::option::Option<::std::boxed::Box<NonComplianceDetailCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageNamesToDisable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the rule includes a mitigating action to disable apps specified in the list, but app data is preserved."]
        pub package_names_to_disable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ComplianceRule {
        pub fn builder() -> ComplianceRuleBuilder {
            ComplianceRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contact details for LaForge enterprises."]
    pub struct ContactInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address for a point of contact, which will be used to send important announcements related to managed Google Play."]
        pub contact_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataProtectionOfficerEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the data protection officer. The email is validated but not verified."]
        pub data_protection_officer_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataProtectionOfficerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the data protection officer."]
        pub data_protection_officer_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataProtectionOfficerPhone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number of the data protection officer The phone number is validated but not verified."]
        pub data_protection_officer_phone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "euRepresentativeEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the EU representative. The email is validated but not verified."]
        pub eu_representative_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "euRepresentativeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the EU representative."]
        pub eu_representative_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "euRepresentativePhone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number of the EU representative. The phone number is validated but not verified."]
        pub eu_representative_phone: ::std::option::Option<::std::string::String>,
    }
    impl ContactInfo {
        pub fn builder() -> ContactInfoBuilder {
            ContactInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: A full date, with non-zero year, month, and day values A month and day value, with a zero year, such as an anniversary A year on its own, with zero month and day values A year and month value, with a zero day, such as a credit card expiration dateRelated types are google.type.TimeOfDay and google.protobuf.Timestamp."]
    pub struct Date {
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
    impl Date {
        pub fn builder() -> DateBuilder {
            DateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A device owned by an enterprise. Unless otherwise noted, all fields are read-only and can't be modified by enterprises.devices.patch."]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API level of the Android platform version running on the device."]
        pub api_level: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reports for apps installed on the device. This information is only available when application_reports_enabled is true in the device's policy."]
        pub application_reports:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationReport>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appliedPolicyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy currently applied to the device."]
        pub applied_policy_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appliedPolicyVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the policy currently applied to the device."]
        pub applied_policy_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appliedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state currently applied to the device."]
        pub applied_state: ::std::option::Option<DeviceAppliedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonCriteriaModeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC).This information is only available if statusReportingSettings.commonCriteriaModeEnabled is true in the device's policy."]
        pub common_criteria_mode_info:
            ::std::option::Option<::std::boxed::Box<CommonCriteriaModeInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device settings information. This information is only available if deviceSettingsEnabled is true in the device's policy."]
        pub device_settings: ::std::option::Option<::std::boxed::Box<DeviceSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabledReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the device state is DISABLED, an optional message that is displayed on the device indicating the reason the device is disabled. This field can be modified by a patch request."]
        pub disabled_reason: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed information about displays on the device. This information is only available if displayInfoEnabled is true in the device's policy."]
        pub displays: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Display>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enrollmentTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time of device enrollment."]
        pub enrollment_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enrollmentTokenData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the device was enrolled with an enrollment token with additional data provided, this field contains that data."]
        pub enrollment_token_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enrollmentTokenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the device was enrolled with an enrollment token, this field contains the name of the token."]
        pub enrollment_token_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardwareInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed information about the device hardware."]
        pub hardware_info: ::std::option::Option<::std::boxed::Box<HardwareInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardwareStatusSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hardware status samples in chronological order. This information is only available if hardwareStatusEnabled is true in the device's policy."]
        pub hardware_status_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HardwareStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastPolicyComplianceReportTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub last_policy_compliance_report_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastPolicySyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the device fetched its policy."]
        pub last_policy_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastStatusReportTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the device sent a status report."]
        pub last_status_report_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managementMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported."]
        pub management_mode: ::std::option::Option<DeviceManagementModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Events related to memory and storage measurements in chronological order. This information is only available if memoryInfoEnabled is true in the device's policy."]
        pub memory_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MemoryEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memory information. This information is only available if memoryInfoEnabled is true in the device's policy."]
        pub memory_info: ::std::option::Option<::std::boxed::Box<MemoryInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device network information. This information is only available if networkInfoEnabled is true in the device's policy."]
        pub network_info: ::std::option::Option<::std::boxed::Box<NetworkInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonComplianceDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about policy settings that the device is not compliant with."]
        pub non_compliance_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NonComplianceDetail>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ownership of the managed device."]
        pub ownership: ::std::option::Option<DeviceOwnershipEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyCompliant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the device is compliant with its policy."]
        pub policy_compliant: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy applied to the device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device's user is applied. This field can be modified by a patch request. You can specify only the policyId when calling enterprises.devices.patch, as long as the policyId doesn’t contain any slashes. The rest of the policy name is inferred."]
        pub policy_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "powerManagementEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Power management events on the device in chronological order. This information is only available if powerManagementEventsEnabled is true in the device's policy."]
        pub power_management_events:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PowerManagementEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousDeviceNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the same physical device has been enrolled multiple times, this field contains its previous device names. The serial number is used as the unique identifier to determine if the same physical device has enrolled previously. The names are in chronological order."]
        pub previous_device_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityPosture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device's security posture value that reflects how secure the device is."]
        pub security_posture: ::std::option::Option<::std::boxed::Box<SecurityPosture>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "softwareInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed information about the device software. This information is only available if softwareInfoEnabled is true in the device's policy."]
        pub software_info: ::std::option::Option<::std::boxed::Box<SoftwareInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete."]
        pub state: ::std::option::Option<DeviceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of selected system properties name and value related to the device. This information is only available if systemPropertiesEnabled is true in the device's policy."]
        pub system_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who owns the device."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the user that owns this device in the form enterprises/{enterpriseId}/users/{userId}."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state currently applied to the device."]
    pub enum DeviceAppliedStateEnum {
        #[serde(rename = "DEVICE_STATE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The device is active."]
        Active,
        #[serde(rename = "DISABLED")]
        #[doc = "The device is disabled."]
        Disabled,
        #[serde(rename = "DELETED")]
        #[doc = "The device was deleted. This state will never be returned by an API call, but is used in the final status report published to Cloud Pub/Sub when the device acknowledges the deletion."]
        Deleted,
        #[serde(rename = "PROVISIONING")]
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl ::std::default::Default for DeviceAppliedStateEnum {
        fn default() -> Self {
            Self::DeviceStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported."]
    pub enum DeviceManagementModeEnum {
        #[serde(rename = "MANAGEMENT_MODE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        ManagementModeUnspecified,
        #[serde(rename = "DEVICE_OWNER")]
        #[doc = "Device owner. Android Device Policy has full control over the device."]
        DeviceOwner,
        #[serde(rename = "PROFILE_OWNER")]
        #[doc = "Profile owner. Android Device Policy has control over a managed profile on the device."]
        ProfileOwner,
    }
    impl ::std::default::Default for DeviceManagementModeEnum {
        fn default() -> Self {
            Self::ManagementModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Ownership of the managed device."]
    pub enum DeviceOwnershipEnum {
        #[serde(rename = "OWNERSHIP_UNSPECIFIED")]
        #[doc = "Ownership is unspecified."]
        OwnershipUnspecified,
        #[serde(rename = "COMPANY_OWNED")]
        #[doc = "Device is company-owned."]
        CompanyOwned,
        #[serde(rename = "PERSONALLY_OWNED")]
        #[doc = "Device is personally-owned."]
        PersonallyOwned,
    }
    impl ::std::default::Default for DeviceOwnershipEnum {
        fn default() -> Self {
            Self::OwnershipUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete."]
    pub enum DeviceStateEnum {
        #[serde(rename = "DEVICE_STATE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The device is active."]
        Active,
        #[serde(rename = "DISABLED")]
        #[doc = "The device is disabled."]
        Disabled,
        #[serde(rename = "DELETED")]
        #[doc = "The device was deleted. This state will never be returned by an API call, but is used in the final status report published to Cloud Pub/Sub when the device acknowledges the deletion."]
        Deleted,
        #[serde(rename = "PROVISIONING")]
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl ::std::default::Default for DeviceStateEnum {
        fn default() -> Self {
            Self::DeviceStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about security related device settings on device."]
    pub struct DeviceSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adbEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether ADB (https://developer.android.com/studio/command-line/adb.html) is enabled on the device."]
        pub adb_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "developmentSettingsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether developer mode is enabled on the device."]
        pub development_settings_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encryption status from DevicePolicyManager."]
        pub encryption_status: ::std::option::Option<DeviceSettingsEncryptionStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDeviceSecure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the device is secured with PIN/password."]
        pub is_device_secure: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEncrypted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the storage encryption is enabled."]
        pub is_encrypted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unknownSourcesEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether installing apps from unknown sources is enabled."]
        pub unknown_sources_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifyAppsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Verify Apps (Google Play Protect (https://support.google.com/googleplay/answer/2812853)) is enabled on the device."]
        pub verify_apps_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl DeviceSettings {
        pub fn builder() -> DeviceSettingsBuilder {
            DeviceSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Encryption status from DevicePolicyManager."]
    pub enum DeviceSettingsEncryptionStatusEnum {
        #[serde(rename = "ENCRYPTION_STATUS_UNSPECIFIED")]
        #[doc = "Unspecified. No device should have this type."]
        EncryptionStatusUnspecified,
        #[serde(rename = "UNSUPPORTED")]
        #[doc = "Encryption is not supported by the device."]
        Unsupported,
        #[serde(rename = "INACTIVE")]
        #[doc = "Encryption is supported by the device, but is not currently active."]
        Inactive,
        #[serde(rename = "ACTIVATING")]
        #[doc = "Encryption is not currently active, but is currently being activated."]
        Activating,
        #[serde(rename = "ACTIVE")]
        #[doc = "Encryption is active."]
        Active,
        #[serde(rename = "ACTIVE_DEFAULT_KEY")]
        #[doc = "Encryption is active, but an encryption key is not set by the user."]
        ActiveDefaultKey,
        #[serde(rename = "ACTIVE_PER_USER")]
        #[doc = "Encryption is active, and the encryption key is tied to the user profile."]
        ActivePerUser,
    }
    impl ::std::default::Default for DeviceSettingsEncryptionStatusEnum {
        fn default() -> Self {
            Self::EncryptionStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Device display information."]
    pub struct Display {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "density")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display density expressed as dots-per-inch."]
        pub density: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique display id."]
        pub display_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display height in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the display."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refresh rate of the display in frames per second."]
        pub refresh_rate: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the display."]
        pub state: ::std::option::Option<DisplayStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display width in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Display {
        pub fn builder() -> DisplayBuilder {
            DisplayBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the display."]
    pub enum DisplayStateEnum {
        #[serde(rename = "DISPLAY_STATE_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        DisplayStateUnspecified,
        #[serde(rename = "OFF")]
        #[doc = "Display is off."]
        Off,
        #[serde(rename = "ON")]
        #[doc = "Display is on."]
        On,
        #[serde(rename = "DOZE")]
        #[doc = "Display is dozing in a low power state"]
        Doze,
        #[serde(rename = "SUSPENDED")]
        #[doc = "Display is dozing in a suspended low power state."]
        Suspended,
    }
    impl ::std::default::Default for DisplayStateEnum {
        fn default() -> Self {
            Self::DisplayStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An enrollment token."]
    pub struct EnrollmentToken {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional, arbitrary data associated with the enrollment token. This could contain, for example, the ID of an org unit the device is assigned to after enrollment. After a device enrolls with the token, this data will be exposed in the enrollment_token_data field of the Device resource. The data must be 1024 characters or less; otherwise, the creation request will fail."]
        pub additional_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPersonalUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
        pub allow_personal_usage: ::std::option::Option<EnrollmentTokenAllowPersonalUsageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of time the enrollment token is valid, ranging from 1 minute to 90 days. If not specified, the default duration is 1 hour."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expiration time of the token. This is a read-only field generated by the server."]
        pub expiration_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the enrollment token, which is generated by the server during creation, in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneTimeOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the enrollment token is for one time use only. If the flag is set to true, only one device can use it for registration."]
        pub one_time_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy initially applied to the enrolled device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device’s user is applied. If user_name is also not specified, enterprises/{enterpriseId}/policies/default is applied by default. When updating this field, you can specify only the policyId as long as the policyId doesn’t contain any slashes. The rest of the policy name will be inferred."]
        pub policy_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qrCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON."]
        pub qr_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user associated with this enrollment token. If it's specified when the enrollment token is created and the user does not exist, the user will be created. This field must not contain personally identifiable information. Only the account_identifier field needs to be set."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl EnrollmentToken {
        pub fn builder() -> EnrollmentTokenBuilder {
            EnrollmentTokenBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
    pub enum EnrollmentTokenAllowPersonalUsageEnum {
        #[serde(rename = "ALLOW_PERSONAL_USAGE_UNSPECIFIED")]
        #[doc = "Personal usage restriction is not specified"]
        AllowPersonalUsageUnspecified,
        #[serde(rename = "PERSONAL_USAGE_ALLOWED")]
        #[doc = "Personal usage is allowed"]
        PersonalUsageAllowed,
        #[serde(rename = "PERSONAL_USAGE_DISALLOWED")]
        #[doc = "Personal usage is disallowed"]
        PersonalUsageDisallowed,
    }
    impl ::std::default::Default for EnrollmentTokenAllowPersonalUsageEnum {
        fn default() -> Self {
            Self::AllowPersonalUsageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration applied to an enterprise."]
    pub struct Enterprise {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appAutoApprovalEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated and unused."]
        pub app_auto_approval_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This feature is not generally available yet. The enterprise contact info of an EMM owned enterprise"]
        pub contact_info: ::std::option::Option<::std::boxed::Box<ContactInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledNotificationTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The types of Google Pub/Sub notifications enabled for the enterprise."]
        pub enabled_notification_types:
            ::std::option::Option<::std::vec::Vec<EnterpriseEnabledNotificationTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enterpriseDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the enterprise displayed to users."]
        pub enterprise_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image displayed as a logo during device provisioning. Supported types are: image/bmp, image/gif, image/x-ico, image/jpeg, image/png, image/webp, image/vnd.wap.wbmp, image/x-adobe-dng."]
        pub logo: ::std::option::Option<::std::boxed::Box<ExternalData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the enterprise which is generated by the server during creation, in the form enterprises/{enterpriseId}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A color in RGB format that indicates the predominant color to display in the device management app UI. The color components are stored as follows: (red << 16) | (green << 8) | blue, where the value of each component is between 0 and 255, inclusive."]
        pub primary_color: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The topic that Cloud Pub/Sub notifications are published to, in the form projects/{project}/topics/{topic}. This field is only required if Pub/Sub notifications are enabled."]
        pub pubsub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signinDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sign-in details of the enterprise."]
        pub signin_details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SigninDetail>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "termsAndConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Terms and conditions that must be accepted when provisioning a device for this enterprise. A page of terms is generated for each value in this list."]
        pub terms_and_conditions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TermsAndConditions>>>,
    }
    impl Enterprise {
        pub fn builder() -> EnterpriseBuilder {
            EnterpriseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum EnterpriseEnabledNotificationTypesEnum {
        #[serde(rename = "NOTIFICATION_TYPE_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        NotificationTypeUnspecified,
        #[serde(rename = "ENROLLMENT")]
        #[doc = "A notification sent when a device enrolls."]
        Enrollment,
        #[serde(rename = "COMPLIANCE_REPORT")]
        #[doc = "Deprecated."]
        ComplianceReport,
        #[serde(rename = "STATUS_REPORT")]
        #[doc = "A notification sent when a device issues a status report."]
        StatusReport,
        #[serde(rename = "COMMAND")]
        #[doc = "A notification sent when a device command has completed."]
        Command,
    }
    impl ::std::default::Default for EnterpriseEnabledNotificationTypesEnum {
        fn default() -> Self {
            Self::NotificationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data hosted at an external location. The data is to be downloaded by Android Device Policy and verified against the hash."]
    pub struct ExternalData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base-64 encoded SHA-256 hash of the content hosted at url. If the content doesn't match this hash, Android Device Policy won't use the data."]
        pub sha256_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The absolute URL to the data, which must use either the http or https scheme. Android Device Policy doesn't provide any credentials in the GET request, so the URL must be publicly accessible. Including a long, random component in the URL may be used to prevent attackers from discovering the URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ExternalData {
        pub fn builder() -> ExternalDataBuilder {
            ExternalDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A system freeze period. When a device’s clock is within the freeze period, all incoming system updates (including security patches) are blocked and won’t be installed. When a device is outside the freeze period, normal update behavior applies. Leap years are ignored in freeze period calculations, in particular: * If Feb. 29th is set as the start or end date of a freeze period, the freeze period will start or end on Feb. 28th instead. * When a device’s system clock reads Feb. 29th, it’s treated as Feb. 28th. * When calculating the number of days in a freeze period or the time between two freeze periods, Feb. 29th is ignored and not counted as a day."]
    pub struct FreezePeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end date (inclusive) of the freeze period. Must be no later than 90 days from the start date. If the end date is earlier than the start date, the freeze period is considered wrapping year-end. Note: year must not be set. For example, {\"month\": 1,\"date\": 30}."]
        pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start date (inclusive) of the freeze period. Note: year must not be set. For example, {\"month\": 1,\"date\": 30}."]
        pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
    }
    impl FreezePeriod {
        pub fn builder() -> FreezePeriodBuilder {
            FreezePeriodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about device hardware. The fields related to temperature thresholds are only available if hardwareStatusEnabled is true in the device's policy."]
    pub struct HardwareInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batteryShutdownTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Battery shutdown temperature thresholds in Celsius for each battery on the device."]
        pub battery_shutdown_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batteryThrottlingTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Battery throttling temperature thresholds in Celsius for each battery on the device."]
        pub battery_throttling_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Brand of the device. For example, Google."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuShutdownTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU shutdown temperature thresholds in Celsius for each CPU on the device."]
        pub cpu_shutdown_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuThrottlingTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU throttling temperature thresholds in Celsius for each CPU on the device."]
        pub cpu_throttling_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceBasebandVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Baseband version. For example, MDM9625_104662.22.05.34p."]
        pub device_baseband_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gpuShutdownTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GPU shutdown temperature thresholds in Celsius for each GPU on the device."]
        pub gpu_shutdown_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gpuThrottlingTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GPU throttling temperature thresholds in Celsius for each GPU on the device."]
        pub gpu_throttling_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardware")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the hardware. For example, Angler."]
        pub hardware: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manufacturer. For example, Motorola."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The model of the device. For example, Asus Nexus 7."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device serial number."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skinShutdownTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device skin shutdown temperature thresholds in Celsius."]
        pub skin_shutdown_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skinThrottlingTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device skin throttling temperature thresholds in Celsius."]
        pub skin_throttling_temperatures:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl HardwareInfo {
        pub fn builder() -> HardwareInfoBuilder {
            HardwareInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Hardware status. Temperatures may be compared to the temperature thresholds available in hardwareInfo to determine hardware health."]
    pub struct HardwareStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batteryTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current battery temperatures in Celsius for each battery on the device."]
        pub battery_temperatures: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current CPU temperatures in Celsius for each CPU on the device."]
        pub cpu_temperatures: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuUsages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU usages in percentage for each core available on the device. Usage is 0 for each unplugged core. Empty array implies that CPU usage is not supported in the system."]
        pub cpu_usages: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the measurements were taken."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fanSpeeds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fan speeds in RPM for each fan on the device. Empty array means that there are no fans or fan speed is not supported on the system."]
        pub fan_speeds: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gpuTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current GPU temperatures in Celsius for each GPU on the device."]
        pub gpu_temperatures: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skinTemperatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current device skin temperatures in Celsius."]
        pub skin_temperatures: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl HardwareStatus {
        pub fn builder() -> HardwareStatusBuilder {
            HardwareStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Keyed app state reported by the app."]
    pub struct KeyedAppState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the app state on the device."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally, a machine-readable value to be read by the EMM. For example, setting values that the admin can choose to query against in the EMM console (e.g. “notify me if the battery_warning data < 10”)."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key for the app state. Acts as a point of reference for what the app is providing state for. For example, when providing managed configuration feedback, this key could be the managed configuration key."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the app state was most recently updated."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally, a free-form message string to explain the app state. If the state was triggered by a particular value (e.g. a managed configuration value), it should be included in the message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity of the app state."]
        pub severity: ::std::option::Option<KeyedAppStateSeverityEnum>,
    }
    impl KeyedAppState {
        pub fn builder() -> KeyedAppStateBuilder {
            KeyedAppStateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The severity of the app state."]
    pub enum KeyedAppStateSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unspecified severity level."]
        SeverityUnspecified,
        #[serde(rename = "INFO")]
        #[doc = "Information severity level."]
        Info,
        #[serde(rename = "ERROR")]
        #[doc = "Error severity level. This should only be set for genuine error conditions that a management organization needs to take action to fix."]
        Error,
    }
    impl ::std::default::Default for KeyedAppStateSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings controlling the behavior of a device in kiosk mode. To enable kiosk mode, set kioskCustomLauncherEnabled to true or specify an app in the policy with installType KIOSK."]
    pub struct KioskCustomization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether the Settings app is allowed in kiosk mode."]
        pub device_settings: ::std::option::Option<KioskCustomizationDeviceSettingsEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "powerButtonActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets the behavior of a device in kiosk mode when a user presses and holds (long-presses) the Power button."]
        pub power_button_actions: ::std::option::Option<KioskCustomizationPowerButtonActionsEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusBar")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether system info and notifications are disabled in kiosk mode."]
        pub status_bar: ::std::option::Option<KioskCustomizationStatusBarEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemErrorWarnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether system error dialogs for crashed or unresponsive apps are blocked in kiosk mode. When blocked, the system will force-stop the app as if the user chooses the \"close app\" option on the UI."]
        pub system_error_warnings: ::std::option::Option<KioskCustomizationSystemErrorWarningsEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemNavigation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which navigation features are enabled (e.g. Home, Overview buttons) in kiosk mode."]
        pub system_navigation: ::std::option::Option<KioskCustomizationSystemNavigationEnum>,
    }
    impl KioskCustomization {
        pub fn builder() -> KioskCustomizationBuilder {
            KioskCustomizationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether the Settings app is allowed in kiosk mode."]
    pub enum KioskCustomizationDeviceSettingsEnum {
        #[serde(rename = "DEVICE_SETTINGS_UNSPECIFIED")]
        #[doc = "Unspecified, defaults to SETTINGS_ACCESS_ALLOWED."]
        DeviceSettingsUnspecified,
        #[serde(rename = "SETTINGS_ACCESS_ALLOWED")]
        #[doc = "Access to the Settings app is allowed in kiosk mode."]
        SettingsAccessAllowed,
        #[serde(rename = "SETTINGS_ACCESS_BLOCKED")]
        #[doc = "Access to the Settings app is not allowed in kiosk mode."]
        SettingsAccessBlocked,
    }
    impl ::std::default::Default for KioskCustomizationDeviceSettingsEnum {
        fn default() -> Self {
            Self::DeviceSettingsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sets the behavior of a device in kiosk mode when a user presses and holds (long-presses) the Power button."]
    pub enum KioskCustomizationPowerButtonActionsEnum {
        #[serde(rename = "POWER_BUTTON_ACTIONS_UNSPECIFIED")]
        #[doc = "Unspecified, defaults to POWER_BUTTON_AVAILABLE."]
        PowerButtonActionsUnspecified,
        #[serde(rename = "POWER_BUTTON_AVAILABLE")]
        #[doc = "The power menu (e.g. Power off, Restart) is shown when a user long-presses the Power button of a device in kiosk mode."]
        PowerButtonAvailable,
        #[serde(rename = "POWER_BUTTON_BLOCKED")]
        #[doc = "The power menu (e.g. Power off, Restart) is not shown when a user long-presses the Power button of a device in kiosk mode. Note: this may prevent users from turning off the device."]
        PowerButtonBlocked,
    }
    impl ::std::default::Default for KioskCustomizationPowerButtonActionsEnum {
        fn default() -> Self {
            Self::PowerButtonActionsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether system info and notifications are disabled in kiosk mode."]
    pub enum KioskCustomizationStatusBarEnum {
        #[serde(rename = "STATUS_BAR_UNSPECIFIED")]
        #[doc = "Unspecified, defaults to INFO_AND_NOTIFICATIONS_DISABLED."]
        StatusBarUnspecified,
        #[serde(rename = "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED")]
        #[doc = "System info and notifications are shown on the status bar in kiosk mode.Note: For this policy to take effect, the device's home button must be enabled using kioskCustomization.systemNavigation."]
        NotificationsAndSystemInfoEnabled,
        #[serde(rename = "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED")]
        #[doc = "System info and notifications are disabled in kiosk mode."]
        NotificationsAndSystemInfoDisabled,
        #[serde(rename = "SYSTEM_INFO_ONLY")]
        #[doc = "Only system info is shown on the status bar."]
        SystemInfoOnly,
    }
    impl ::std::default::Default for KioskCustomizationStatusBarEnum {
        fn default() -> Self {
            Self::StatusBarUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether system error dialogs for crashed or unresponsive apps are blocked in kiosk mode. When blocked, the system will force-stop the app as if the user chooses the \"close app\" option on the UI."]
    pub enum KioskCustomizationSystemErrorWarningsEnum {
        #[serde(rename = "SYSTEM_ERROR_WARNINGS_UNSPECIFIED")]
        #[doc = "Unspecified, defaults to ERROR_AND_WARNINGS_MUTED."]
        SystemErrorWarningsUnspecified,
        #[serde(rename = "ERROR_AND_WARNINGS_ENABLED")]
        #[doc = "All system error dialogs such as crash and app not responding (ANR) are displayed."]
        ErrorAndWarningsEnabled,
        #[serde(rename = "ERROR_AND_WARNINGS_MUTED")]
        #[doc = "All system error dialogs, such as crash and app not responding (ANR) are blocked. When blocked, the system force-stops the app as if the user closes the app from the UI."]
        ErrorAndWarningsMuted,
    }
    impl ::std::default::Default for KioskCustomizationSystemErrorWarningsEnum {
        fn default() -> Self {
            Self::SystemErrorWarningsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which navigation features are enabled (e.g. Home, Overview buttons) in kiosk mode."]
    pub enum KioskCustomizationSystemNavigationEnum {
        #[serde(rename = "SYSTEM_NAVIGATION_UNSPECIFIED")]
        #[doc = "Unspecified, defaults to NAVIGATION_DISABLED."]
        SystemNavigationUnspecified,
        #[serde(rename = "NAVIGATION_ENABLED")]
        #[doc = "Home and overview buttons are enabled."]
        NavigationEnabled,
        #[serde(rename = "NAVIGATION_DISABLED")]
        #[doc = "The home and Overview buttons are not accessible."]
        NavigationDisabled,
        #[serde(rename = "HOME_BUTTON_ONLY")]
        #[doc = "Only the home button is enabled."]
        HomeButtonOnly,
    }
    impl ::std::default::Default for KioskCustomizationSystemNavigationEnum {
        fn default() -> Self {
            Self::SystemNavigationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An action to launch an app."]
    pub struct LaunchAppAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Package name of app to be launched"]
        pub package_name: ::std::option::Option<::std::string::String>,
    }
    impl LaunchAppAction {
        pub fn builder() -> LaunchAppActionBuilder {
            LaunchAppActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to list devices for a given enterprise."]
    pub struct ListDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of devices."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results, a token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDevicesResponse {
        pub fn builder() -> ListDevicesResponseBuilder {
            ListDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This feature is not generally available yet. Response to a request to list enterprises."]
    pub struct ListEnterprisesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enterprises")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This feature is not generally available yet. The list of enterprises."]
        pub enterprises: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Enterprise>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This feature is not generally available yet. If there are more results, a token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListEnterprisesResponse {
        pub fn builder() -> ListEnterprisesResponseBuilder {
            ListEnterprisesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to list policies for a given enterprise."]
    pub struct ListPoliciesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results, a token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of policies."]
        pub policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Policy>>>,
    }
    impl ListPoliciesResponse {
        pub fn builder() -> ListPoliciesResponseBuilder {
            ListPoliciesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to list web apps for a given enterprise."]
    pub struct ListWebAppsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results, a token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webApps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of web apps."]
        pub web_apps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebApp>>>,
    }
    impl ListWebAppsResponse {
        pub fn builder() -> ListWebAppsResponseBuilder {
            ListWebAppsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The managed configurations template for the app, saved from the managed configurations iframe."]
    pub struct ManagedConfigurationTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional, a map containing configuration variables defined for the configuration."]
        pub configuration_variables:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managed configurations template."]
        pub template_id: ::std::option::Option<::std::string::String>,
    }
    impl ManagedConfigurationTemplate {
        pub fn builder() -> ManagedConfigurationTemplateBuilder {
            ManagedConfigurationTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Managed property."]
    pub struct ManagedProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default value of the property. BUNDLE_ARRAY properties don't have a default value."]
        pub default_value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A longer description of the property, providing more detail of what it affects. Localized."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For CHOICE or MULTISELECT properties, the list of possible entries."]
        pub entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedPropertyEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique key that the app uses to identify the property, e.g. \"com.google.android.gm.fieldname\"."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nestedProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For BUNDLE_ARRAY properties, the list of nested properties. A BUNDLE_ARRAY property is at most two levels deep."]
        pub nested_properties:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedProperty>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the property. Localized."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the property."]
        pub _type: ::std::option::Option<ManagedPropertyTypeEnum>,
    }
    impl ManagedProperty {
        pub fn builder() -> ManagedPropertyBuilder {
            ManagedPropertyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the property."]
    pub enum ManagedPropertyTypeEnum {
        #[serde(rename = "MANAGED_PROPERTY_TYPE_UNSPECIFIED")]
        #[doc = "Not used."]
        ManagedPropertyTypeUnspecified,
        #[serde(rename = "BOOL")]
        #[doc = "A property of boolean type."]
        Bool,
        #[serde(rename = "STRING")]
        #[doc = "A property of string type."]
        String,
        #[serde(rename = "INTEGER")]
        #[doc = "A property of integer type."]
        Integer,
        #[serde(rename = "CHOICE")]
        #[doc = "A choice of one item from a set."]
        Choice,
        #[serde(rename = "MULTISELECT")]
        #[doc = "A choice of multiple items from a set."]
        Multiselect,
        #[serde(rename = "HIDDEN")]
        #[doc = "A hidden restriction of string type (the default value can be used to pass along information that can't be modified, such as a version code)."]
        Hidden,
        #[serde(rename = "BUNDLE")]
        #[doc = "A bundle of properties"]
        Bundle,
        #[serde(rename = "BUNDLE_ARRAY")]
        #[doc = "An array of property bundles."]
        BundleArray,
    }
    impl ::std::default::Default for ManagedPropertyTypeEnum {
        fn default() -> Self {
            Self::ManagedPropertyTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entry of a managed property."]
    pub struct ManagedPropertyEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the value. Localized."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The machine-readable value of the entry, which should be used in the configuration. Not localized."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ManagedPropertyEntry {
        pub fn builder() -> ManagedPropertyEntryBuilder {
            ManagedPropertyEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event related to memory and storage measurements."]
    pub struct MemoryEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of free bytes in the medium, or for EXTERNAL_STORAGE_DETECTED, the total capacity in bytes of the storage medium."]
        pub byte_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the event."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event type."]
        pub event_type: ::std::option::Option<MemoryEventEventTypeEnum>,
    }
    impl MemoryEvent {
        pub fn builder() -> MemoryEventBuilder {
            MemoryEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Event type."]
    pub enum MemoryEventEventTypeEnum {
        #[serde(rename = "MEMORY_EVENT_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified. No events have this type."]
        MemoryEventTypeUnspecified,
        #[serde(rename = "RAM_MEASURED")]
        #[doc = "Free space in RAM was measured."]
        RamMeasured,
        #[serde(rename = "INTERNAL_STORAGE_MEASURED")]
        #[doc = "Free space in internal storage was measured."]
        InternalStorageMeasured,
        #[serde(rename = "EXTERNAL_STORAGE_DETECTED")]
        #[doc = "A new external storage medium was detected. The reported byte count is the total capacity of the storage medium."]
        ExternalStorageDetected,
        #[serde(rename = "EXTERNAL_STORAGE_REMOVED")]
        #[doc = "An external storage medium was removed. The reported byte count is zero."]
        ExternalStorageRemoved,
        #[serde(rename = "EXTERNAL_STORAGE_MEASURED")]
        #[doc = "Free space in an external storage medium was measured."]
        ExternalStorageMeasured,
    }
    impl ::std::default::Default for MemoryEventEventTypeEnum {
        fn default() -> Self {
            Self::MemoryEventTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about device memory and storage."]
    pub struct MemoryInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalInternalStorage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total internal storage on device in bytes."]
        pub total_internal_storage: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRam")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total RAM on device in bytes."]
        pub total_ram: ::std::option::Option<::std::string::String>,
    }
    impl MemoryInfo {
        pub fn builder() -> MemoryInfoBuilder {
            MemoryInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Device network info."]
    pub struct NetworkInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imei")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IMEI number of the GSM device. For example, A1000031212."]
        pub imei: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MEID number of the CDMA device. For example, A00000292788E1."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkOperatorName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alphabetic name of current registered operator. For example, Vodafone."]
        pub network_operator_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telephonyInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides telephony information associated with each SIM card on the device. Only supported on fully managed devices starting from Android API level 23."]
        pub telephony_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TelephonyInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiMacAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Wi-Fi MAC address of the device. For example, 7c:11:11:11:11:11."]
        pub wifi_mac_address: ::std::option::Option<::std::string::String>,
    }
    impl NetworkInfo {
        pub fn builder() -> NetworkInfoBuilder {
            NetworkInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides detail about non-compliance with a policy setting."]
    pub struct NonComplianceDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the policy setting could not be applied, the current value of the setting on the device."]
        pub current_value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For settings with nested fields, if a particular nested field is out of compliance, this specifies the full path to the offending field. The path is formatted in the same way the policy JSON field would be referenced in JavaScript, that is: 1) For object-typed fields, the field name is followed by a dot then by a subfield name. 2) For array-typed fields, the field name is followed by the array index enclosed in brackets. For example, to indicate a problem with the url field in the externalData field in the 3rd application, the path would be applications[2].externalData.url"]
        pub field_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installationFailureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated."]
        pub installation_failure_reason:
            ::std::option::Option<NonComplianceDetailInstallationFailureReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonComplianceReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason the device is not in compliance with the setting."]
        pub non_compliance_reason:
            ::std::option::Option<NonComplianceDetailNonComplianceReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name indicating which app is out of compliance, if applicable."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy field."]
        pub setting_name: ::std::option::Option<::std::string::String>,
    }
    impl NonComplianceDetail {
        pub fn builder() -> NonComplianceDetailBuilder {
            NonComplianceDetailBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated."]
    pub enum NonComplianceDetailInstallationFailureReasonEnum {
        #[serde(rename = "INSTALLATION_FAILURE_REASON_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        InstallationFailureReasonUnspecified,
        #[serde(rename = "INSTALLATION_FAILURE_REASON_UNKNOWN")]
        #[doc = "An unknown condition is preventing the app from being installed. Some potential reasons are that the device doesn't have enough storage, the device network connection is unreliable, or the installation is taking longer than expected. The installation will be retried automatically."]
        InstallationFailureReasonUnknown,
        #[serde(rename = "IN_PROGRESS")]
        #[doc = "The installation is still in progress."]
        InProgress,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "The app was not found in Play."]
        NotFound,
        #[serde(rename = "NOT_COMPATIBLE_WITH_DEVICE")]
        #[doc = "The app is incompatible with the device."]
        NotCompatibleWithDevice,
        #[serde(rename = "NOT_APPROVED")]
        #[doc = "The app has not been approved by the admin."]
        NotApproved,
        #[serde(rename = "PERMISSIONS_NOT_ACCEPTED")]
        #[doc = "The app has new permissions that have not been accepted by the admin."]
        PermissionsNotAccepted,
        #[serde(rename = "NOT_AVAILABLE_IN_COUNTRY")]
        #[doc = "The app is not available in the user's country."]
        NotAvailableInCountry,
        #[serde(rename = "NO_LICENSES_REMAINING")]
        #[doc = "There are no licenses available to assign to the user."]
        NoLicensesRemaining,
        #[serde(rename = "NOT_ENROLLED")]
        #[doc = "The enterprise is no longer enrolled with Managed Google Play or the admin has not accepted the latest Managed Google Play Terms of Service."]
        NotEnrolled,
        #[serde(rename = "USER_INVALID")]
        #[doc = "The user is no longer valid. The user may have been deleted or disabled."]
        UserInvalid,
    }
    impl ::std::default::Default for NonComplianceDetailInstallationFailureReasonEnum {
        fn default() -> Self {
            Self::InstallationFailureReasonUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reason the device is not in compliance with the setting."]
    pub enum NonComplianceDetailNonComplianceReasonEnum {
        #[serde(rename = "NON_COMPLIANCE_REASON_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[serde(rename = "API_LEVEL")]
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[serde(rename = "MANAGEMENT_MODE")]
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[serde(rename = "USER_ACTION")]
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
        #[serde(rename = "INVALID_VALUE")]
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[serde(rename = "APP_NOT_INSTALLED")]
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[serde(rename = "UNSUPPORTED")]
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[serde(rename = "APP_INSTALLED")]
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[serde(rename = "PENDING")]
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[serde(rename = "APP_INCOMPATIBLE")]
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[serde(rename = "APP_NOT_UPDATED")]
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
    }
    impl ::std::default::Default for NonComplianceDetailNonComplianceReasonEnum {
        fn default() -> Self {
            Self::NonComplianceReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A compliance rule condition which is satisfied if there exists any matching NonComplianceDetail for the device. A NonComplianceDetail matches a NonComplianceDetailCondition if all the fields which are set within the NonComplianceDetailCondition match the corresponding NonComplianceDetail fields."]
    pub struct NonComplianceDetailCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonComplianceReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason the device is not in compliance with the setting. If not set, then this condition matches any reason."]
        pub non_compliance_reason:
            ::std::option::Option<NonComplianceDetailConditionNonComplianceReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the app that's out of compliance. If not set, then this condition matches any package name."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy field. If not set, then this condition matches any setting name."]
        pub setting_name: ::std::option::Option<::std::string::String>,
    }
    impl NonComplianceDetailCondition {
        pub fn builder() -> NonComplianceDetailConditionBuilder {
            NonComplianceDetailConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reason the device is not in compliance with the setting. If not set, then this condition matches any reason."]
    pub enum NonComplianceDetailConditionNonComplianceReasonEnum {
        #[serde(rename = "NON_COMPLIANCE_REASON_UNSPECIFIED")]
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[serde(rename = "API_LEVEL")]
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[serde(rename = "MANAGEMENT_MODE")]
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[serde(rename = "USER_ACTION")]
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
        #[serde(rename = "INVALID_VALUE")]
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[serde(rename = "APP_NOT_INSTALLED")]
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[serde(rename = "UNSUPPORTED")]
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[serde(rename = "APP_INSTALLED")]
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[serde(rename = "PENDING")]
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[serde(rename = "APP_INCOMPATIBLE")]
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[serde(rename = "APP_NOT_UPDATED")]
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
    }
    impl ::std::default::Default for NonComplianceDetailConditionNonComplianceReasonEnum {
        fn default() -> Self {
            Self::NonComplianceReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of package names."]
    pub struct PackageNameList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of package names."]
        pub package_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PackageNameList {
        pub fn builder() -> PackageNameListBuilder {
            PackageNameListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Requirements for the password used to unlock a device."]
    pub struct PasswordRequirements {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumFailedPasswordsForWipe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of incorrect device-unlock passwords that can be entered before a device is wiped. A value of 0 means there is no restriction."]
        pub maximum_failed_passwords_for_wipe: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordExpirationTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password expiration timeout."]
        pub password_expiration_timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordHistoryLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of the password history. After setting this field, the user won't be able to enter a new password that is the same as any password in the history. A value of 0 means there is no restriction."]
        pub password_history_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum allowed password length. A value of 0 means there is no restriction. Only enforced when password_quality is NUMERIC, NUMERIC_COMPLEX, ALPHABETIC, ALPHANUMERIC, or COMPLEX."]
        pub password_minimum_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumLetters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of letters required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_letters: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumLowerCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of lower case letters required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_lower_case: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumNonLetter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of non-letter characters (numerical digits or symbols) required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_non_letter: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumNumeric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of numerical digits required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_numeric: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumSymbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of symbols required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_symbols: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordMinimumUpperCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of upper case letters required in the password. Only enforced when password_quality is COMPLEX."]
        pub password_minimum_upper_case: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordQuality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The required password quality."]
        pub password_quality: ::std::option::Option<PasswordRequirementsPasswordQualityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scope that the password requirement applies to."]
        pub password_scope: ::std::option::Option<PasswordRequirementsPasswordScopeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirePasswordUnlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of time after a device or work profile is unlocked using a strong form of authentication (password, PIN, pattern) that it can be unlocked using any other authentication method (e.g. fingerprint, trust agents, face). After the specified time period elapses, only strong forms of authentication can be used to unlock the device or work profile."]
        pub require_password_unlock:
            ::std::option::Option<PasswordRequirementsRequirePasswordUnlockEnum>,
    }
    impl PasswordRequirements {
        pub fn builder() -> PasswordRequirementsBuilder {
            PasswordRequirementsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The required password quality."]
    pub enum PasswordRequirementsPasswordQualityEnum {
        #[serde(rename = "PASSWORD_QUALITY_UNSPECIFIED")]
        #[doc = "There are no password requirements."]
        PasswordQualityUnspecified,
        #[serde(rename = "BIOMETRIC_WEAK")]
        #[doc = "The device must be secured with a low-security biometric recognition technology, at minimum. This includes technologies that can recognize the identity of an individual that are roughly equivalent to a 3-digit PIN (false detection is less than 1 in 1,000)."]
        BiometricWeak,
        #[serde(rename = "SOMETHING")]
        #[doc = "A password is required, but there are no restrictions on what the password must contain."]
        Something,
        #[serde(rename = "NUMERIC")]
        #[doc = "The password must contain numeric characters."]
        Numeric,
        #[serde(rename = "NUMERIC_COMPLEX")]
        #[doc = "The password must contain numeric characters with no repeating (4444) or ordered (1234, 4321, 2468) sequences."]
        NumericComplex,
        #[serde(rename = "ALPHABETIC")]
        #[doc = "The password must contain alphabetic (or symbol) characters."]
        Alphabetic,
        #[serde(rename = "ALPHANUMERIC")]
        #[doc = "The password must contain both numeric and alphabetic (or symbol) characters."]
        Alphanumeric,
        #[serde(rename = "COMPLEX")]
        #[doc = "The password must meet the minimum requirements specified in passwordMinimumLength, passwordMinimumLetters, passwordMinimumSymbols, etc. For example, if passwordMinimumSymbols is 2, the password must contain at least two symbols."]
        Complex,
    }
    impl ::std::default::Default for PasswordRequirementsPasswordQualityEnum {
        fn default() -> Self {
            Self::PasswordQualityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The scope that the password requirement applies to."]
    pub enum PasswordRequirementsPasswordScopeEnum {
        #[serde(rename = "SCOPE_UNSPECIFIED")]
        #[doc = "The scope is unspecified. The password requirements are applied to the work profile for work profile devices and the whole device for fully managed or dedicated devices."]
        ScopeUnspecified,
        #[serde(rename = "SCOPE_DEVICE")]
        #[doc = "The password requirements are only applied to the device."]
        ScopeDevice,
        #[serde(rename = "SCOPE_PROFILE")]
        #[doc = "The password requirements are only applied to the work profile."]
        ScopeProfile,
    }
    impl ::std::default::Default for PasswordRequirementsPasswordScopeEnum {
        fn default() -> Self {
            Self::ScopeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The length of time after a device or work profile is unlocked using a strong form of authentication (password, PIN, pattern) that it can be unlocked using any other authentication method (e.g. fingerprint, trust agents, face). After the specified time period elapses, only strong forms of authentication can be used to unlock the device or work profile."]
    pub enum PasswordRequirementsRequirePasswordUnlockEnum {
        #[serde(rename = "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to USE_DEFAULT_DEVICE_TIMEOUT."]
        RequirePasswordUnlockUnspecified,
        #[serde(rename = "USE_DEFAULT_DEVICE_TIMEOUT")]
        #[doc = "The timeout period is set to the device’s default."]
        UseDefaultDeviceTimeout,
        #[serde(rename = "REQUIRE_EVERY_DAY")]
        #[doc = "The timeout period is set to 24 hours."]
        RequireEveryDay,
    }
    impl ::std::default::Default for PasswordRequirementsRequirePasswordUnlockEnum {
        fn default() -> Self {
            Self::RequirePasswordUnlockUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for an Android permission and its grant state."]
    pub struct PermissionGrant {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Android permission or group, e.g. android.permission.READ_CALENDAR or android.permission_group.CALENDAR."]
        pub permission: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy for granting the permission."]
        pub policy: ::std::option::Option<PermissionGrantPolicyEnum>,
    }
    impl PermissionGrant {
        pub fn builder() -> PermissionGrantBuilder {
            PermissionGrantBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The policy for granting the permission."]
    pub enum PermissionGrantPolicyEnum {
        #[serde(rename = "PERMISSION_POLICY_UNSPECIFIED")]
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[serde(rename = "PROMPT")]
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[serde(rename = "GRANT")]
        #[doc = "Automatically grant a permission."]
        Grant,
        #[serde(rename = "DENY")]
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl ::std::default::Default for PermissionGrantPolicyEnum {
        fn default() -> Self {
            Self::PermissionPolicyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A default activity for handling intents that match a particular intent filter. Note: To set up a kiosk, use InstallType to KIOSK rather than use persistent preferred activities."]
    pub struct PersistentPreferredActivity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent actions to match in the filter. If any actions are included in the filter, then an intent's action must be one of those values for it to match. If no actions are included, the intent action is ignored."]
        pub actions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent categories to match in the filter. An intent includes the categories that it requires, all of which must be included in the filter in order to match. In other words, adding a category to the filter has no impact on matching unless that category is specified in the intent."]
        pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "receiverActivity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The activity that should be the default intent handler. This should be an Android component name, e.g. com.android.enterprise.app/.MainActivity. Alternatively, the value may be the package name of an app, which causes Android Device Policy to choose an appropriate activity from the app to handle the intent."]
        pub receiver_activity: ::std::option::Option<::std::string::String>,
    }
    impl PersistentPreferredActivity {
        pub fn builder() -> PersistentPreferredActivityBuilder {
            PersistentPreferredActivityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policies for apps in the personal profile of a company-owned device with a work profile."]
    pub struct PersonalApplicationPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of installation to perform."]
        pub install_type: ::std::option::Option<PersonalApplicationPolicyInstallTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the application."]
        pub package_name: ::std::option::Option<::std::string::String>,
    }
    impl PersonalApplicationPolicy {
        pub fn builder() -> PersonalApplicationPolicyBuilder {
            PersonalApplicationPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of installation to perform."]
    pub enum PersonalApplicationPolicyInstallTypeEnum {
        #[serde(rename = "INSTALL_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to AVAILABLE."]
        InstallTypeUnspecified,
        #[serde(rename = "BLOCKED")]
        #[doc = "The app is blocked and can't be installed in the personal profile."]
        Blocked,
        #[serde(rename = "AVAILABLE")]
        #[doc = "The app is available to install in the personal profile."]
        Available,
    }
    impl ::std::default::Default for PersonalApplicationPolicyInstallTypeEnum {
        fn default() -> Self {
            Self::InstallTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policies controlling personal usage on a company-owned device with a work profile."]
    pub struct PersonalUsagePolicies {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountTypesWithManagementDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account types that can't be managed by the user."]
        pub account_types_with_management_disabled:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cameraDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether camera is disabled."]
        pub camera_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxDaysWithWorkOff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls how long the work profile can stay off. The duration must be at least 3 days."]
        pub max_days_with_work_off: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personalApplications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy applied to applications in the personal profile."]
        pub personal_applications:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PersonalApplicationPolicy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personalPlayStoreMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used together with personalApplications to control how apps in the personal profile are allowed or blocked."]
        pub personal_play_store_mode:
            ::std::option::Option<PersonalUsagePoliciesPersonalPlayStoreModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenCaptureDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether screen capture is disabled."]
        pub screen_capture_disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl PersonalUsagePolicies {
        pub fn builder() -> PersonalUsagePoliciesBuilder {
            PersonalUsagePoliciesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Used together with personalApplications to control how apps in the personal profile are allowed or blocked."]
    pub enum PersonalUsagePoliciesPersonalPlayStoreModeEnum {
        #[serde(rename = "PLAY_STORE_MODE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to BLOCKLIST."]
        PlayStoreModeUnspecified,
        #[serde(rename = "BLACKLIST")]
        #[doc = "All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications."]
        Blacklist,
        #[serde(rename = "BLOCKLIST")]
        #[doc = "All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications."]
        Blocklist,
        #[serde(rename = "ALLOWLIST")]
        #[doc = "Only apps explicitly specified in personalApplications with installType set to AVAILABLE are allowed to be installed in the personal profile."]
        Allowlist,
    }
    impl ::std::default::Default for PersonalUsagePoliciesPersonalPlayStoreModeEnum {
        fn default() -> Self {
            Self::PlayStoreModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A policy resource represents a group of settings that govern the behavior of a managed device and the apps installed on it."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountTypesWithManagementDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account types that can't be managed by the user."]
        pub account_types_with_management_disabled:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addUserDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether adding new users and profiles is disabled."]
        pub add_user_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adjustVolumeDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether adjusting the master volume is disabled. Also mutes the device."]
        pub adjust_volume_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advancedSecurityOverrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Security policies set to the most secure values by default. To maintain the security posture of a device, we don't recommend overriding any of the default values."]
        pub advanced_security_overrides:
            ::std::option::Option<::std::boxed::Box<AdvancedSecurityOverrides>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alwaysOnVpnPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for an always-on VPN connection. Use with vpn_config_disabled to prevent modification of this setting."]
        pub always_on_vpn_package: ::std::option::Option<::std::boxed::Box<AlwaysOnVpnPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidDevicePolicyTracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app tracks for Android Device Policy the device can access. The device receives the latest version among all accessible tracks. If no tracks are specified, then the device only uses the production track."]
        pub android_device_policy_tracks:
            ::std::option::Option<::std::vec::Vec<PolicyAndroidDevicePolicyTracksEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appAutoUpdatePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app auto update policy, which controls when automatic app updates can be applied."]
        pub app_auto_update_policy: ::std::option::Option<PolicyAppAutoUpdatePolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy applied to apps."]
        pub applications:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationPolicy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoDateAndTimeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether auto date, time, and time zone are enabled on a company-owned device. If this is set, then autoTimeRequired is ignored."]
        pub auto_date_and_time_zone: ::std::option::Option<PolicyAutoDateAndTimeZoneEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoTimeRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether auto time is required, which prevents the user from manually setting the date and time. If autoDateAndTimeZone is set, this field is ignored."]
        pub auto_time_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockApplicationsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether applications other than the ones configured in applications are blocked from being installed. When set, applications that were installed under a previous policy but no longer appear in the policy are automatically uninstalled."]
        pub block_applications_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bluetoothConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring bluetooth is disabled."]
        pub bluetooth_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bluetoothContactSharingDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether bluetooth contact sharing is disabled."]
        pub bluetooth_contact_sharing_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bluetoothDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether bluetooth is disabled. Prefer this setting over bluetooth_config_disabled because bluetooth_config_disabled can be bypassed by the user."]
        pub bluetooth_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cameraDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether all cameras on the device are disabled."]
        pub camera_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellBroadcastsConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring cell broadcast is disabled."]
        pub cell_broadcasts_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "choosePrivateKeyRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rules for automatically choosing a private key and certificate to authenticate the device to a server. The rules are ordered by increasing precedence, so if an outgoing request matches more than one rule, the last rule defines which private key to use."]
        pub choose_private_key_rules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChoosePrivateKeyRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rules declaring which mitigating actions to take when a device is not compliant with its policy. When the conditions for multiple rules are satisfied, all of the mitigating actions for the rules are taken. There is a maximum limit of 100 rules. Use policy enforcement rules instead."]
        pub compliance_rules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ComplianceRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createWindowsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether creating windows besides app windows is disabled."]
        pub create_windows_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentialsConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring user credentials is disabled."]
        pub credentials_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRoamingDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether roaming data services are disabled."]
        pub data_roaming_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debuggingFeaturesAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is allowed to enable debugging features."]
        pub debugging_features_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultPermissionPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default permission policy for runtime permission requests."]
        pub default_permission_policy: ::std::option::Option<PolicyDefaultPermissionPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceOwnerLockScreenInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device owner information to be shown on the lock screen."]
        pub device_owner_lock_screen_info:
            ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether encryption is enabled"]
        pub encryption_policy: ::std::option::Option<PolicyEncryptionPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ensureVerifyAppsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether app verification is force-enabled."]
        pub ensure_verify_apps_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "factoryResetDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether factory resetting from settings is disabled."]
        pub factory_reset_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frpAdminEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email addresses of device administrators for factory reset protection. When the device is factory reset, it will require one of these admins to log in with the Google account email and password to unlock the device. If no admins are specified, the device won't provide factory reset protection."]
        pub frp_admin_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "funDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is allowed to have fun. Controls whether the Easter egg game in Settings is disabled."]
        pub fun_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installAppsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user installation of apps is disabled."]
        pub install_apps_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installUnknownSourcesAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is allowed to enable the \"Unknown Sources\" setting, which allows installation of apps from unknown sources."]
        pub install_unknown_sources_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyguardDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the keyguard is disabled."]
        pub keyguard_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyguardDisabledFeatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disabled keyguard customizations, such as widgets."]
        pub keyguard_disabled_features:
            ::std::option::Option<::std::vec::Vec<PolicyKeyguardDisabledFeaturesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kioskCustomLauncherEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the kiosk custom launcher is enabled. This replaces the home screen with a launcher that locks down the device to the apps installed via the applications setting. Apps appear on a single page in alphabetical order. Use kioskCustomization to further configure the kiosk device behavior."]
        pub kiosk_custom_launcher_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kioskCustomization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings controlling the behavior of a device in kiosk mode. To enable kiosk mode, set kioskCustomLauncherEnabled to true or specify an app in the policy with installType KIOSK."]
        pub kiosk_customization: ::std::option::Option<::std::boxed::Box<KioskCustomization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The degree of location detection enabled."]
        pub location_mode: ::std::option::Option<PolicyLocationModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longSupportMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message displayed to the user in the device administators settings screen."]
        pub long_support_message: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumTimeToLock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum time in milliseconds for user activity until the device locks. A value of 0 means there is no restriction."]
        pub maximum_time_to_lock: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumApiLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum allowed Android API level."]
        pub minimum_api_level: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileNetworksConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring mobile networks is disabled."]
        pub mobile_networks_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifyAccountsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether adding or removing accounts is disabled."]
        pub modify_accounts_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mountPhysicalMediaDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user mounting physical external media is disabled."]
        pub mount_physical_media_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkEscapeHatchEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the network escape hatch is enabled. If a network connection can't be made at boot time, the escape hatch prompts the user to temporarily connect to a network in order to refresh the device policy. After applying policy, the temporary network will be forgotten and the device will continue booting. This prevents being unable to connect to a network if there is no suitable network in the last policy and the device boots into an app in lock task mode, or the user is otherwise unable to reach device settings."]
        pub network_escape_hatch_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkResetDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether resetting network settings is disabled."]
        pub network_reset_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openNetworkConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network configuration for the device. See configure networks for more information."]
        pub open_network_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outgoingBeamDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether using NFC to beam data from apps is disabled."]
        pub outgoing_beam_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outgoingCallsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether outgoing calls are disabled."]
        pub outgoing_calls_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordPolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password requirement policies. Different policies can be set for work profile or fully managed devices by setting the password_scope field in the policy."]
        pub password_policies:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PasswordRequirements>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordRequirements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password requirements. The field password_requirements.require_password_unlock must not be set. DEPRECATED - Use password_policies."]
        pub password_requirements: ::std::option::Option<::std::boxed::Box<PasswordRequirements>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissionGrants")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit permission or group grants or denials for all apps. These values override the default_permission_policy."]
        pub permission_grants:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PermissionGrant>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permittedAccessibilityServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies permitted accessibility services. If the field is not set, any accessibility service can be used. If the field is set, only the accessibility services in this list and the system's built-in accessibility service can be used. In particular, if the field is set to empty, only the system's built-in accessibility servicess can be used."]
        pub permitted_accessibility_services:
            ::std::option::Option<::std::boxed::Box<PackageNameList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permittedInputMethods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, only the input methods provided by packages in this list are permitted. If this field is present, but the list is empty, then only system input methods are permitted."]
        pub permitted_input_methods: ::std::option::Option<::std::boxed::Box<PackageNameList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "persistentPreferredActivities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default intent handler activities."]
        pub persistent_preferred_activities:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PersistentPreferredActivity>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personalUsagePolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policies managing personal usage on a company-owned device."]
        pub personal_usage_policies:
            ::std::option::Option<::std::boxed::Box<PersonalUsagePolicies>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playStoreMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy."]
        pub play_store_mode: ::std::option::Option<PolicyPlayStoreModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyEnforcementRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rules that define the behavior when a particular policy can not be applied on device"]
        pub policy_enforcement_rules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyEnforcementRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateKeySelectionEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows showing UI on a device for a user to choose a private key alias if there are no matching rules in ChoosePrivateKeyRules. For devices below Android P, setting this may leave enterprise keys vulnerable."]
        pub private_key_selection_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendedGlobalProxy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network-independent global HTTP proxy. Typically proxies should be configured per-network in open_network_configuration. However for unusual configurations like general internal filtering a global HTTP proxy may be useful. If the proxy is not accessible, network access may break. The global proxy is only a recommendation and some apps may ignore it."]
        pub recommended_global_proxy: ::std::option::Option<::std::boxed::Box<ProxyInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeUserDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether removing other users is disabled."]
        pub remove_user_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeBootDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether rebooting the device into safe boot is disabled."]
        pub safe_boot_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenCaptureDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether screen capture is disabled."]
        pub screen_capture_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setUserIconDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether changing the user icon is disabled."]
        pub set_user_icon_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setWallpaperDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether changing the wallpaper is disabled."]
        pub set_wallpaper_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setupActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actions to take during the setup process."]
        pub setup_actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SetupAction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shareLocationDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether location sharing is disabled."]
        pub share_location_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortSupportMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message displayed to the user in the settings screen wherever functionality has been disabled by the admin. If the message is longer than 200 characters it may be truncated."]
        pub short_support_message: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipFirstUseHintsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to skip hints on the first use. Enterprise admin can enable the system recommendation for apps to skip their user tutorial and other introductory hints on first start-up."]
        pub skip_first_use_hints_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether sending and receiving SMS messages is disabled."]
        pub sms_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusBarDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the status bar is disabled. This disables notifications, quick settings, and other screen overlays that allow escape from full-screen mode. DEPRECATED. To disable the status bar on a kiosk device, use InstallType KIOSK or kioskCustomLauncherEnabled."]
        pub status_bar_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusReportingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status reporting settings"]
        pub status_reporting_settings:
            ::std::option::Option<::std::boxed::Box<StatusReportingSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stayOnPluggedModes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn't lock itself while it stays on."]
        pub stay_on_plugged_modes:
            ::std::option::Option<::std::vec::Vec<PolicyStayOnPluggedModesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system update policy, which controls how OS updates are applied. If the update type is WINDOWED, the update window will automatically apply to Play app updates as well."]
        pub system_update: ::std::option::Option<::std::boxed::Box<SystemUpdate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tetheringConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring tethering and portable hotspots is disabled."]
        pub tethering_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uninstallAppsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user uninstallation of applications is disabled."]
        pub uninstall_apps_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unmuteMicrophoneDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the microphone is muted and adjusting microphone volume is disabled."]
        pub unmute_microphone_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usbFileTransferDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether transferring files over USB is disabled."]
        pub usb_file_transfer_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usbMassStorageEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether USB storage is enabled. Deprecated."]
        pub usb_mass_storage_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the policy. This is a read-only field. The version is incremented each time the policy is updated."]
        pub version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vpnConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring VPN is disabled."]
        pub vpn_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiConfigDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether configuring Wi-Fi access points is disabled."]
        pub wifi_config_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiConfigsLockdownEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED - Use wifi_config_disabled."]
        pub wifi_configs_lockdown_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PolicyAndroidDevicePolicyTracksEnum {
        #[serde(rename = "APP_TRACK_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        AppTrackUnspecified,
        #[serde(rename = "PRODUCTION")]
        #[doc = "The production track, which provides the latest stable release."]
        Production,
        #[serde(rename = "BETA")]
        #[doc = "The beta track, which provides the latest beta release."]
        Beta,
    }
    impl ::std::default::Default for PolicyAndroidDevicePolicyTracksEnum {
        fn default() -> Self {
            Self::AppTrackUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The app auto update policy, which controls when automatic app updates can be applied."]
    pub enum PolicyAppAutoUpdatePolicyEnum {
        #[serde(rename = "APP_AUTO_UPDATE_POLICY_UNSPECIFIED")]
        #[doc = "The auto-update policy is not set. Equivalent to CHOICE_TO_THE_USER."]
        AppAutoUpdatePolicyUnspecified,
        #[serde(rename = "CHOICE_TO_THE_USER")]
        #[doc = "The user can control auto-updates."]
        ChoiceToTheUser,
        #[serde(rename = "NEVER")]
        #[doc = "Apps are never auto-updated."]
        Never,
        #[serde(rename = "WIFI_ONLY")]
        #[doc = "Apps are auto-updated over Wi-Fi only."]
        WifiOnly,
        #[serde(rename = "ALWAYS")]
        #[doc = "Apps are auto-updated at any time. Data charges may apply."]
        Always,
    }
    impl ::std::default::Default for PolicyAppAutoUpdatePolicyEnum {
        fn default() -> Self {
            Self::AppAutoUpdatePolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether auto date, time, and time zone are enabled on a company-owned device. If this is set, then autoTimeRequired is ignored."]
    pub enum PolicyAutoDateAndTimeZoneEnum {
        #[serde(rename = "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to AUTO_DATE_AND_TIME_ZONE_USER_CHOICE."]
        AutoDateAndTimeZoneUnspecified,
        #[serde(rename = "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE")]
        #[doc = "Auto date, time, and time zone are left to user's choice."]
        AutoDateAndTimeZoneUserChoice,
        #[serde(rename = "AUTO_DATE_AND_TIME_ZONE_ENFORCED")]
        #[doc = "Enforce auto date, time, and time zone on the device."]
        AutoDateAndTimeZoneEnforced,
    }
    impl ::std::default::Default for PolicyAutoDateAndTimeZoneEnum {
        fn default() -> Self {
            Self::AutoDateAndTimeZoneUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The default permission policy for runtime permission requests."]
    pub enum PolicyDefaultPermissionPolicyEnum {
        #[serde(rename = "PERMISSION_POLICY_UNSPECIFIED")]
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[serde(rename = "PROMPT")]
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[serde(rename = "GRANT")]
        #[doc = "Automatically grant a permission."]
        Grant,
        #[serde(rename = "DENY")]
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl ::std::default::Default for PolicyDefaultPermissionPolicyEnum {
        fn default() -> Self {
            Self::PermissionPolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether encryption is enabled"]
    pub enum PolicyEncryptionPolicyEnum {
        #[serde(rename = "ENCRYPTION_POLICY_UNSPECIFIED")]
        #[doc = "This value is ignored, i.e. no encryption required"]
        EncryptionPolicyUnspecified,
        #[serde(rename = "ENABLED_WITHOUT_PASSWORD")]
        #[doc = "Encryption required but no password required to boot"]
        EnabledWithoutPassword,
        #[serde(rename = "ENABLED_WITH_PASSWORD")]
        #[doc = "Encryption required with password required to boot"]
        EnabledWithPassword,
    }
    impl ::std::default::Default for PolicyEncryptionPolicyEnum {
        fn default() -> Self {
            Self::EncryptionPolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PolicyKeyguardDisabledFeaturesEnum {
        #[serde(rename = "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        KeyguardDisabledFeatureUnspecified,
        #[serde(rename = "CAMERA")]
        #[doc = "Disable the camera on secure keyguard screens (e.g. PIN)."]
        Camera,
        #[serde(rename = "NOTIFICATIONS")]
        #[doc = "Disable showing all notifications on secure keyguard screens."]
        Notifications,
        #[serde(rename = "UNREDACTED_NOTIFICATIONS")]
        #[doc = "Disable unredacted notifications on secure keyguard screens."]
        UnredactedNotifications,
        #[serde(rename = "TRUST_AGENTS")]
        #[doc = "Ignore trust agent state on secure keyguard screens."]
        TrustAgents,
        #[serde(rename = "DISABLE_FINGERPRINT")]
        #[doc = "Disable fingerprint sensor on secure keyguard screens."]
        DisableFingerprint,
        #[serde(rename = "DISABLE_REMOTE_INPUT")]
        #[doc = "Disable text entry into notifications on secure keyguard screens."]
        DisableRemoteInput,
        #[serde(rename = "FACE")]
        #[doc = "Disable face authentication on secure keyguard screens."]
        Face,
        #[serde(rename = "IRIS")]
        #[doc = "Disable iris authentication on secure keyguard screens."]
        Iris,
        #[serde(rename = "BIOMETRICS")]
        #[doc = "Disable all biometric authentication on secure keyguard screens."]
        Biometrics,
        #[serde(rename = "ALL_FEATURES")]
        #[doc = "Disable all current and future keyguard customizations."]
        AllFeatures,
    }
    impl ::std::default::Default for PolicyKeyguardDisabledFeaturesEnum {
        fn default() -> Self {
            Self::KeyguardDisabledFeatureUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The degree of location detection enabled."]
    pub enum PolicyLocationModeEnum {
        #[serde(rename = "LOCATION_MODE_UNSPECIFIED")]
        #[doc = "Defaults to LOCATION_USER_CHOICE."]
        LocationModeUnspecified,
        #[serde(rename = "HIGH_ACCURACY")]
        #[doc = "On Android 8 and below, all location detection methods are enabled, including GPS, networks, and other sensors. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        HighAccuracy,
        #[serde(rename = "SENSORS_ONLY")]
        #[doc = "On Android 8 and below, only GPS and other sensors are enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        SensorsOnly,
        #[serde(rename = "BATTERY_SAVING")]
        #[doc = "On Android 8 and below, only the network location provider is enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED."]
        BatterySaving,
        #[serde(rename = "OFF")]
        #[doc = "On Android 8 and below, location setting and accuracy are disabled. On Android 9 and above, this is equivalent to LOCATION_DISABLED."]
        Off,
        #[serde(rename = "LOCATION_USER_CHOICE")]
        #[doc = "Location setting is not restricted on the device. No specific behavior is set or enforced."]
        LocationUserChoice,
        #[serde(rename = "LOCATION_ENFORCED")]
        #[doc = "Enable location setting on the device."]
        LocationEnforced,
        #[serde(rename = "LOCATION_DISABLED")]
        #[doc = "Disable location setting on the device."]
        LocationDisabled,
    }
    impl ::std::default::Default for PolicyLocationModeEnum {
        fn default() -> Self {
            Self::LocationModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy."]
    pub enum PolicyPlayStoreModeEnum {
        #[serde(rename = "PLAY_STORE_MODE_UNSPECIFIED")]
        #[doc = "Unspecified. Defaults to WHITELIST."]
        PlayStoreModeUnspecified,
        #[serde(rename = "WHITELIST")]
        #[doc = "Only apps that are in the policy are available and any app not in the policy will be automatically uninstalled from the device."]
        Whitelist,
        #[serde(rename = "BLACKLIST")]
        #[doc = "All apps are available and any app that should not be on the device should be explicitly marked as 'BLOCKED' in the applications policy."]
        Blacklist,
    }
    impl ::std::default::Default for PolicyPlayStoreModeEnum {
        fn default() -> Self {
            Self::PlayStoreModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PolicyStayOnPluggedModesEnum {
        #[serde(rename = "BATTERY_PLUGGED_MODE_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        BatteryPluggedModeUnspecified,
        #[serde(rename = "AC")]
        #[doc = "Power source is an AC charger."]
        Ac,
        #[serde(rename = "USB")]
        #[doc = "Power source is a USB port."]
        Usb,
        #[serde(rename = "WIRELESS")]
        #[doc = "Power source is wireless."]
        Wireless,
    }
    impl ::std::default::Default for PolicyStayOnPluggedModesEnum {
        fn default() -> Self {
            Self::BatteryPluggedModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule that defines the actions to take if a device or work profile is not compliant with the policy specified in settingName."]
    pub struct PolicyEnforcementRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An action to block access to apps and data on a fully managed device or in a work profile. This action also triggers a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified."]
        pub block_action: ::std::option::Option<::std::boxed::Box<BlockAction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "settingName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top-level policy to enforce. For example, applications or passwordPolicies."]
        pub setting_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wipeAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An action to reset a fully managed device or delete a work profile. Note: blockAction must also be specified."]
        pub wipe_action: ::std::option::Option<::std::boxed::Box<WipeAction>>,
    }
    impl PolicyEnforcementRule {
        pub fn builder() -> PolicyEnforcementRuleBuilder {
            PolicyEnforcementRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details regarding the security posture of the device."]
    pub struct PostureDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponding admin-facing advice to mitigate this security risk and improve the security posture of the device."]
        pub advice: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserFacingMessage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityRisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A specific security risk that negatively affects the security posture of the device."]
        pub security_risk: ::std::option::Option<PostureDetailSecurityRiskEnum>,
    }
    impl PostureDetail {
        pub fn builder() -> PostureDetailBuilder {
            PostureDetailBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A specific security risk that negatively affects the security posture of the device."]
    pub enum PostureDetailSecurityRiskEnum {
        #[serde(rename = "SECURITY_RISK_UNSPECIFIED")]
        #[doc = "Unspecified."]
        SecurityRiskUnspecified,
        #[serde(rename = "UNKNOWN_OS")]
        #[doc = "SafetyNet detects that the device is running an unknown OS (basicIntegrity check succeeds but ctsProfileMatch fails)."]
        UnknownOs,
        #[serde(rename = "COMPROMISED_OS")]
        #[doc = "SafetyNet detects that the device is running a compromised OS (basicIntegrity check fails)."]
        CompromisedOs,
    }
    impl ::std::default::Default for PostureDetailSecurityRiskEnum {
        fn default() -> Self {
            Self::SecurityRiskUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A power management event."]
    pub struct PowerManagementEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batteryLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For BATTERY_LEVEL_COLLECTED events, the battery level as a percentage."]
        pub battery_level: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the event."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event type."]
        pub event_type: ::std::option::Option<PowerManagementEventEventTypeEnum>,
    }
    impl PowerManagementEvent {
        pub fn builder() -> PowerManagementEventBuilder {
            PowerManagementEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Event type."]
    pub enum PowerManagementEventEventTypeEnum {
        #[serde(rename = "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified. No events have this type."]
        PowerManagementEventTypeUnspecified,
        #[serde(rename = "BATTERY_LEVEL_COLLECTED")]
        #[doc = "Battery level was measured."]
        BatteryLevelCollected,
        #[serde(rename = "POWER_CONNECTED")]
        #[doc = "The device started charging."]
        PowerConnected,
        #[serde(rename = "POWER_DISCONNECTED")]
        #[doc = "The device stopped charging."]
        PowerDisconnected,
        #[serde(rename = "BATTERY_LOW")]
        #[doc = "The device entered low-power mode."]
        BatteryLow,
        #[serde(rename = "BATTERY_OKAY")]
        #[doc = "The device exited low-power mode."]
        BatteryOkay,
        #[serde(rename = "BOOT_COMPLETED")]
        #[doc = "The device booted."]
        BootCompleted,
        #[serde(rename = "SHUTDOWN")]
        #[doc = "The device shut down."]
        Shutdown,
    }
    impl ::std::default::Default for PowerManagementEventEventTypeEnum {
        fn default() -> Self {
            Self::PowerManagementEventTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration info for an HTTP proxy. For a direct proxy, set the host, port, and excluded_hosts fields. For a PAC script proxy, set the pac_uri field."]
    pub struct ProxyInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedHosts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For a direct proxy, the hosts for which the proxy is bypassed. The host names may contain wildcards such as *.example.com."]
        pub excluded_hosts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The host of the direct proxy."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pacUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the PAC script used to configure the proxy."]
        pub pac_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The port of the direct proxy."]
        pub port: ::std::option::Option<::std::primitive::i64>,
    }
    impl ProxyInfo {
        pub fn builder() -> ProxyInfoBuilder {
            ProxyInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The security posture of the device, as determined by the current device state and the policies applied."]
    pub struct SecurityPosture {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devicePosture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device's security posture value."]
        pub device_posture: ::std::option::Option<SecurityPostureDevicePostureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postureDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional details regarding the security posture of the device."]
        pub posture_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostureDetail>>>,
    }
    impl SecurityPosture {
        pub fn builder() -> SecurityPostureBuilder {
            SecurityPostureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Device's security posture value."]
    pub enum SecurityPostureDevicePostureEnum {
        #[serde(rename = "POSTURE_UNSPECIFIED")]
        #[doc = "Unspecified. There is no posture detail for this posture value."]
        PostureUnspecified,
        #[serde(rename = "SECURE")]
        #[doc = "This device is secure."]
        Secure,
        #[serde(rename = "AT_RISK")]
        #[doc = "This device may be more vulnerable to malicious actors than is recommended for use with corporate data."]
        AtRisk,
        #[serde(rename = "POTENTIALLY_COMPROMISED")]
        #[doc = "This device may be compromised and corporate data may be accessible to unauthorized actors."]
        PotentiallyCompromised,
    }
    impl ::std::default::Default for SecurityPostureDevicePostureEnum {
        fn default() -> Self {
            Self::PostureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An action executed during setup."]
    pub struct SetupAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of this action."]
        pub description: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An action to launch an app."]
        pub launch_app: ::std::option::Option<::std::boxed::Box<LaunchAppAction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of this action."]
        pub title: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
    }
    impl SetupAction {
        pub fn builder() -> SetupActionBuilder {
            SetupActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource containing sign in details for an enterprise."]
    pub struct SigninDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPersonalUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
        pub allow_personal_usage: ::std::option::Option<SigninDetailAllowPersonalUsageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qrCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON. This is a read-only field generated by the server."]
        pub qr_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signinEnrollmentToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An enterprise wide enrollment token used to trigger custom sign-in flow. This is a read-only field generated by the server."]
        pub signin_enrollment_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signinUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sign-in URL for authentication when device is provisioned with a sign-in enrollment token. The sign-in endpoint should finish authentication flow with a URL in the form of https://enterprise.google.com/android/enroll?et= for a successful login, or https://enterprise.google.com/android/enroll/invalid for a failed login."]
        pub signin_url: ::std::option::Option<::std::string::String>,
    }
    impl SigninDetail {
        pub fn builder() -> SigninDetailBuilder {
            SigninDetailBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device."]
    pub enum SigninDetailAllowPersonalUsageEnum {
        #[serde(rename = "ALLOW_PERSONAL_USAGE_UNSPECIFIED")]
        #[doc = "Personal usage restriction is not specified"]
        AllowPersonalUsageUnspecified,
        #[serde(rename = "PERSONAL_USAGE_ALLOWED")]
        #[doc = "Personal usage is allowed"]
        PersonalUsageAllowed,
        #[serde(rename = "PERSONAL_USAGE_DISALLOWED")]
        #[doc = "Personal usage is disallowed"]
        PersonalUsageDisallowed,
    }
    impl ::std::default::Default for SigninDetailAllowPersonalUsageEnum {
        fn default() -> Self {
            Self::AllowPersonalUsageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An enterprise signup URL."]
    pub struct SignupUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the resource. Use this value in the signupUrl field when calling enterprises.create to complete the enterprise signup flow."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL where an enterprise admin can register their enterprise. The page can't be rendered in an iframe."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl SignupUrl {
        pub fn builder() -> SignupUrlBuilder {
            SignupUrlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about device software."]
    pub struct SoftwareInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidBuildNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Android build ID string meant for displaying to the user. For example, shamu-userdebug 6.0.1 MOB30I 2756745 dev-keys."]
        pub android_build_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidBuildTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Build time."]
        pub android_build_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidDevicePolicyVersionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Android Device Policy app version code."]
        pub android_device_policy_version_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidDevicePolicyVersionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Android Device Policy app version as displayed to the user."]
        pub android_device_policy_version_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-visible Android version string. For example, 6.0.1."]
        pub android_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootloaderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system bootloader version number, e.g. 0.6.7."]
        pub bootloader_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceBuildSignature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SHA-256 hash of android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the system package, which can be used to verify that the system build hasn't been modified."]
        pub device_build_signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceKernelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Kernel version, for example, 2.6.32.9-g103d848."]
        pub device_kernel_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryLanguageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An IETF BCP 47 language code for the primary locale on the device."]
        pub primary_language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityPatchLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Security patch level, e.g. 2016-05-01."]
        pub security_patch_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemUpdateInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a potential pending system update."]
        pub system_update_info: ::std::option::Option<::std::boxed::Box<SystemUpdateInfo>>,
    }
    impl SoftwareInfo {
        pub fn builder() -> SoftwareInfoBuilder {
            SoftwareInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings controlling the behavior of status reports."]
    pub struct StatusReportingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationReportingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application reporting settings. Only applicable if application_reports_enabled is true."]
        pub application_reporting_settings:
            ::std::option::Option<::std::boxed::Box<ApplicationReportingSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationReportsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether app reports are enabled."]
        pub application_reports_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonCriteriaModeEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Common Criteria Mode reporting is enabled."]
        pub common_criteria_mode_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceSettingsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether device settings reporting is enabled."]
        pub device_settings_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayInfoEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether displays reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        pub display_info_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hardwareStatusEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether hardware status reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        pub hardware_status_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryInfoEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether memory reporting is enabled."]
        pub memory_info_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkInfoEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether network info reporting is enabled."]
        pub network_info_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "powerManagementEventsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether power management event reporting is enabled. Report data is not available for personally owned devices with work profiles."]
        pub power_management_events_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "softwareInfoEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether software info reporting is enabled."]
        pub software_info_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemPropertiesEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether system properties reporting is enabled."]
        pub system_properties_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl StatusReportingSettings {
        pub fn builder() -> StatusReportingSettingsBuilder {
            StatusReportingSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for managing system updates"]
    pub struct SystemUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endMinutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the type is WINDOWED, the end of the maintenance window, measured as the number of minutes after midnight in device's local time. This value must be between 0 and 1439, inclusive. If this value is less than start_minutes, then the maintenance window spans midnight. If the maintenance window specified is smaller than 30 minutes, the actual window is extended to 30 minutes beyond the start time."]
        pub end_minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "freezePeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An annually repeating time period in which over-the-air (OTA) system updates are postponed to freeze the OS version running on a device. To prevent freezing the device indefinitely, each freeze period must be separated by at least 60 days."]
        pub freeze_periods: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FreezePeriod>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startMinutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the type is WINDOWED, the start of the maintenance window, measured as the number of minutes after midnight in the device's local time. This value must be between 0 and 1439, inclusive."]
        pub start_minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of system update to configure."]
        pub _type: ::std::option::Option<SystemUpdateTypeEnum>,
    }
    impl SystemUpdate {
        pub fn builder() -> SystemUpdateBuilder {
            SystemUpdateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of system update to configure."]
    pub enum SystemUpdateTypeEnum {
        #[serde(rename = "SYSTEM_UPDATE_TYPE_UNSPECIFIED")]
        #[doc = "Follow the default update behavior for the device, which typically requires the user to accept system updates."]
        SystemUpdateTypeUnspecified,
        #[serde(rename = "AUTOMATIC")]
        #[doc = "Install automatically as soon as an update is available."]
        Automatic,
        #[serde(rename = "WINDOWED")]
        #[doc = "Install automatically within a daily maintenance window. This also configures Play apps to be updated within the window. This is strongly recommended for kiosk devices because this is the only way apps persistently pinned to the foreground can be updated by Play."]
        Windowed,
        #[serde(rename = "POSTPONE")]
        #[doc = "Postpone automatic install up to a maximum of 30 days."]
        Postpone,
    }
    impl ::std::default::Default for SystemUpdateTypeEnum {
        fn default() -> Self {
            Self::SystemUpdateTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a potential pending system update."]
    pub struct SystemUpdateInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateReceivedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the update was first available. A zero value indicates that this field is not set. This field is set only if an update is available (that is, updateStatus is neither UPDATE_STATUS_UNKNOWN nor UP_TO_DATE)."]
        pub update_received_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of an update: whether an update exists and what type it is."]
        pub update_status: ::std::option::Option<SystemUpdateInfoUpdateStatusEnum>,
    }
    impl SystemUpdateInfo {
        pub fn builder() -> SystemUpdateInfoBuilder {
            SystemUpdateInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of an update: whether an update exists and what type it is."]
    pub enum SystemUpdateInfoUpdateStatusEnum {
        #[serde(rename = "UPDATE_STATUS_UNKNOWN")]
        #[doc = "It is unknown whether there is a pending system update. This happens when, for example, the device API level is less than 26, or if the version of Android Device Policy is outdated."]
        UpdateStatusUnknown,
        #[serde(rename = "UP_TO_DATE")]
        #[doc = "There is no pending system update available on the device."]
        UpToDate,
        #[serde(rename = "UNKNOWN_UPDATE_AVAILABLE")]
        #[doc = "There is a pending system update available, but its type is not known."]
        UnknownUpdateAvailable,
        #[serde(rename = "SECURITY_UPDATE_AVAILABLE")]
        #[doc = "There is a pending security update available."]
        SecurityUpdateAvailable,
        #[serde(rename = "OS_UPDATE_AVAILABLE")]
        #[doc = "There is a pending OS update available."]
        OsUpdateAvailable,
    }
    impl ::std::default::Default for SystemUpdateInfoUpdateStatusEnum {
        fn default() -> Self {
            Self::UpdateStatusUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Telephony information associated with a given SIM card on the device. Only supported on fully managed devices starting from Android API level 23."]
    pub struct TelephonyInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier name associated with this SIM card."]
        pub carrier_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number associated with this SIM card."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl TelephonyInfo {
        pub fn builder() -> TelephonyInfoBuilder {
            TelephonyInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A terms and conditions page to be accepted during provisioning."]
    pub struct TermsAndConditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A well-formatted HTML string. It will be parsed on the client with android.text.Html#fromHtml."]
        pub content: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short header which appears above the HTML content."]
        pub header: ::std::option::Option<::std::boxed::Box<UserFacingMessage>>,
    }
    impl TermsAndConditions {
        pub fn builder() -> TermsAndConditionsBuilder {
            TermsAndConditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user belonging to an enterprise."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier you create for this user, such as user342 or asset#44418. This field must be set when the user is created and can't be updated. This field must not contain personally identifiable information (PII). This identifier must be 1024 characters or less; otherwise, the update policy request will fail."]
        pub account_identifier: ::std::option::Option<::std::string::String>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides a user-facing message with locale info. The maximum message length is 4096 characters."]
    pub struct UserFacingMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default message displayed if no localized message is specified or the user's locale doesn't match with any of the localized messages. A default message must be provided if any localized messages are provided."]
        pub default_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map containing pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr."]
        pub localized_messages:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl UserFacingMessage {
        pub fn builder() -> UserFacingMessageBuilder {
            UserFacingMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A web app."]
    pub struct WebApp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display mode of the web app."]
        pub display_mode: ::std::option::Option<WebAppDisplayModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of icons for the web app. Must have at least one element."]
        pub icons: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebAppIcon>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the web app, which is generated by the server during creation in the form enterprises/{enterpriseId}/webApps/{packageName}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start URL, i.e. the URL that should load when the user opens the application."]
        pub start_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon)."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current version of the app.Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date."]
        pub version_code: ::std::option::Option<::std::string::String>,
    }
    impl WebApp {
        pub fn builder() -> WebAppBuilder {
            WebAppBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The display mode of the web app."]
    pub enum WebAppDisplayModeEnum {
        #[serde(rename = "DISPLAY_MODE_UNSPECIFIED")]
        #[doc = "Not used."]
        DisplayModeUnspecified,
        #[serde(rename = "MINIMAL_UI")]
        #[doc = "Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL."]
        MinimalUi,
        #[serde(rename = "STANDALONE")]
        #[doc = "Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible."]
        Standalone,
        #[serde(rename = "FULL_SCREEN")]
        #[doc = "Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area."]
        FullScreen,
    }
    impl ::std::default::Default for WebAppDisplayModeEnum {
        fn default() -> Self {
            Self::DisplayModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An icon for a web app. Supported formats are: png, jpg and webp."]
    pub struct WebAppIcon {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual bytes of the image in a base64url encoded string (c.f. RFC4648, section 5 \"Base 64 Encoding with URL and Filename Safe Alphabet\"). - The image type can be png or jpg. - The image should ideally be square. - The image should ideally have a size of 512x512. "]
        pub image_data: ::std::option::Option<::std::string::String>,
    }
    impl WebAppIcon {
        pub fn builder() -> WebAppIconBuilder {
            WebAppIconBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A web token used to access the managed Google Play iframe."]
    pub struct WebToken {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledFeatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The features to enable. Use this if you want to control exactly which feature(s) will be activated; leave empty to allow all features.Restrictions / things to note: - If no features are listed here, all features are enabled — this is the default behavior where you give access to all features to your admins. - This must not contain any FEATURE_UNSPECIFIED values. - Repeated values are ignored "]
        pub enabled_features: ::std::option::Option<::std::vec::Vec<WebTokenEnabledFeaturesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the web token, which is generated by the server during creation in the form enterprises/{enterpriseId}/webTokens/{webTokenId}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFrameUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the parent frame hosting the iframe with the embedded UI. To prevent XSS, the iframe may not be hosted at other URLs. The URL must use the https scheme."]
        pub parent_frame_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions available to an admin in the embedded UI. An admin must have all of these permissions in order to view the UI. This field is deprecated."]
        pub permissions: ::std::option::Option<::std::vec::Vec<WebTokenPermissionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token value which is used in the hosting page to generate the iframe with the embedded UI. This is a read-only field generated by the server."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl WebToken {
        pub fn builder() -> WebTokenBuilder {
            WebTokenBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum WebTokenEnabledFeaturesEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified feature."]
        FeatureUnspecified,
        #[serde(rename = "PLAY_SEARCH")]
        #[doc = "The Managed Play search apps page (https://developers.google.com/android/management/apps#search-apps)."]
        PlaySearch,
        #[serde(rename = "PRIVATE_APPS")]
        #[doc = "The private apps page (https://developers.google.com/android/management/apps#private-apps)."]
        PrivateApps,
        #[serde(rename = "WEB_APPS")]
        #[doc = "The Web Apps page (https://developers.google.com/android/management/apps#web-apps)."]
        WebApps,
        #[serde(rename = "STORE_BUILDER")]
        #[doc = "The organize apps page (https://developers.google.com/android/management/apps#organize-apps)."]
        StoreBuilder,
        #[serde(rename = "MANAGED_CONFIGURATIONS")]
        #[doc = "The managed configurations page (https://developers.google.com/android/management/managed-configurations-iframe)."]
        ManagedConfigurations,
    }
    impl ::std::default::Default for WebTokenEnabledFeaturesEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum WebTokenPermissionsEnum {
        #[serde(rename = "WEB_TOKEN_PERMISSION_UNSPECIFIED")]
        #[doc = "This value is ignored."]
        WebTokenPermissionUnspecified,
        #[serde(rename = "APPROVE_APPS")]
        #[doc = "The permission to approve apps for the enterprise."]
        ApproveApps,
    }
    impl ::std::default::Default for WebTokenPermissionsEnum {
        fn default() -> Self {
            Self::WebTokenPermissionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An action to reset a fully managed device or delete a work profile. Note: blockAction must also be specified."]
    pub struct WipeAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preserveFrp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the factory-reset protection data is preserved on the device. This setting doesn’t apply to work profiles."]
        pub preserve_frp: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wipeAfterDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days the policy is non-compliant before the device or work profile is wiped. wipeAfterDays must be greater than blockAfterDays."]
        pub wipe_after_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl WipeAction {
        pub fn builder() -> WipeActionBuilder {
            WipeActionBuilder::default()
        }
    }
}
