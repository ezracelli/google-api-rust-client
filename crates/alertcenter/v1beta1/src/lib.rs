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
    pub mod alerts {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get_metadata {
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
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert metadata is associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A query string for filtering alert results. For more details, see [Query filters](/admin-sdk/alertcenter/guides/query-filters) and [Supported query filter fields](/admin-sdk/alertcenter/reference/filter-fields#alerts.list)."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The sort order of the list results. If not specified results may be returned in arbitrary order. You can sort the results in descending order based on the creation timestamp using `order_by=\"create_time desc\"`. Currently, supported sorting are `create_time asc`, `create_time desc`, `update_time desc`"]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The requested page size. Server may return fewer items than requested. If unspecified, server picks an appropriate default."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A token identifying a page of results the server should return. If empty, a new iteration is started. To continue an iteration, pass in the value from the previous ListAlertsResponse's next_page_token field."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod feedback {
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
                            #[serde(rename = "customerId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
                            pub customer_id: ::std::option::Option<::std::string::String>,
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "customerId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert feedback are associated with. Inferred from the caller identity if not provided."]
                            pub customer_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A query string for filtering alert feedback results. For more details, see [Query filters](/admin-sdk/alertcenter/guides/query-filters) and [Supported query filter fields](/admin-sdk/alertcenter/reference/filter-fields#alerts.feedback.list)."]
                            pub filter: ::std::option::Option<::std::string::String>,
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
    pub mod v1beta1 {
        pub mod methods {
            pub mod get_settings {
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
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert settings are associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update_settings {
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
                    #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert settings are associated with. Inferred from the caller identity if not provided."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
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
    #[doc = "Alerts for user account warning events."]
    pub struct AccountWarning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The email of the user that this event belongs to."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loginDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Details of the login action associated with the warning event. This is only available for: * Suspicious login * Suspicious login (less secure app) * Suspicious programmatic login * User suspended (suspicious activity)"]
        pub login_details: ::std::option::Option<::std::boxed::Box<LoginDetails>>,
    }
    impl AccountWarning {
        pub fn builder() -> AccountWarningBuilder {
            AccountWarningBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alerts from G Suite Security Center rules service configured by admin."]
    pub struct ActivityRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of action names associated with the rule threshold."]
        pub action_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rule create timestamp."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the rule."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alert display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rule name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query that is used to get the data from the associated source."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supersededAlerts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of alert IDs superseded by this alert. It is used to indicate that this alert is essentially extension of superseded alerts and we found the relationship after creating these alerts."]
        pub superseded_alerts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supersedingAlert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alert ID superseding this alert. It is used to indicate that superseding alert is essentially extension of this alert and we found the relationship after creating both alerts."]
        pub superseding_alert: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alert threshold is for example “COUNT > 5”."]
        pub threshold: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The trigger sources for this rule. * GMAIL_EVENTS * DEVICE_EVENTS * USER_EVENTS"]
        pub trigger_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the last update to the rule."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rule window size. Possible values are 1 hour or 24 hours."]
        pub window_size: ::std::option::Option<::std::string::String>,
    }
    impl ActivityRule {
        pub fn builder() -> ActivityRuleBuilder {
            ActivityRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An alert affecting a customer."]
    pub struct Alert {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier for the alert."]
        pub alert_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this alert was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The data associated with this alert, for example google.apps.alertcenter.type.DeviceCompromised."]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. `True` if this alert is marked for deletion."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The time the event that caused this alert ceased being active. If provided, the end time must not be earlier than the start time. If not provided, it indicates an ongoing alert."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform alert updates in order to avoid race conditions: An `etag` is returned in the response which contains alerts, and systems are expected to put that etag in the request to update alert to ensure that their change will be applied to the same version of the alert. If no `etag` is provided in the call to update alert, then the existing alert is overwritten blindly."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The metadata associated with this alert."]
        pub metadata: ::std::option::Option<::std::boxed::Box<AlertMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityInvestigationToolLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An optional [Security Investigation Tool](https://support.google.com/a/answer/7575955) query for this alert."]
        pub security_investigation_tool_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the system that reported the alert. This is output only after alert is created. Supported sources are any of the following: * Google Operations * Mobile device management * Gmail phishing * Domain wide takeout * State sponsored attack * Google identity"]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The time the event that caused this alert was started or detected."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the alert. This is output only after alert is created. For a list of available alert types see [Google Workspace Alert types](/admin-sdk/alertcenter/reference/alert-types)."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this alert was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Alert {
        pub fn builder() -> AlertBuilder {
            AlertBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A customer feedback about an alert."]
    pub struct AlertFeedback {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The alert identifier."]
        pub alert_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this feedback was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The email of the user that provided the feedback."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedbackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier for the feedback."]
        pub feedback_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the feedback."]
        pub _type: ::std::option::Option<AlertFeedbackTypeEnum>,
    }
    impl AlertFeedback {
        pub fn builder() -> AlertFeedbackBuilder {
            AlertFeedbackBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of the feedback."]
    pub enum AlertFeedbackTypeEnum {
        #[serde(rename = "ALERT_FEEDBACK_TYPE_UNSPECIFIED")]
        #[doc = "The feedback type is not specified."]
        AlertFeedbackTypeUnspecified,
        #[serde(rename = "NOT_USEFUL")]
        #[doc = "The alert report is not useful."]
        NotUseful,
        #[serde(rename = "SOMEWHAT_USEFUL")]
        #[doc = "The alert report is somewhat useful."]
        SomewhatUseful,
        #[serde(rename = "VERY_USEFUL")]
        #[doc = "The alert report is very useful."]
        VeryUseful,
    }
    impl ::std::default::Default for AlertFeedbackTypeEnum {
        fn default() -> Self {
            Self::AlertFeedbackTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An alert metadata."]
    pub struct AlertMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The alert identifier."]
        pub alert_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignee")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user assigned to the alert."]
        pub assignee: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert metadata from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform metatdata updates in order to avoid race conditions: An `etag` is returned in the response which contains alert metadata, and systems are expected to put that etag in the request to update alert metadata to ensure that their change will be applied to the same version of the alert metadata. If no `etag` is provided in the call to update alert metadata, then the existing alert metadata is overwritten blindly."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity value of the alert. Alert Center will set this field at alert creation time, default's to an empty string when it could not be determined. The supported values for update actions on this field are the following: * HIGH * MEDIUM * LOW"]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current status of the alert. The supported values are the following: * NOT_STARTED * IN_PROGRESS * CLOSED"]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this metadata was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl AlertMetadata {
        pub fn builder() -> AlertMetadataBuilder {
            AlertMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alerts from App Maker to notify admins to set up default SQL instance."]
    pub struct AppMakerSqlSetupNotification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of applications with requests for default SQL set up."]
        pub request_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RequestInfo>>>,
    }
    impl AppMakerSqlSetupNotification {
        pub fn builder() -> AppMakerSqlSetupNotificationBuilder {
            AppMakerSqlSetupNotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Attachment with application-specific information about an alert."]
    pub struct Attachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A CSV file attachment."]
        pub csv: ::std::option::Option<::std::boxed::Box<Csv>>,
    }
    impl Attachment {
        pub fn builder() -> AttachmentBuilder {
            AttachmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alert for setting the domain or IP that malicious email comes from as whitelisted domain or IP in Gmail advanced settings."]
    pub struct BadWhitelist {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain ID."]
        pub domain_id: ::std::option::Option<::std::boxed::Box<DomainId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maliciousEntity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        pub malicious_entity: ::std::option::Option<::std::boxed::Box<MaliciousEntity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of messages contained by this alert."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmailMessageInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source IP address of the malicious email, for example, `127.0.0.1`."]
        pub source_ip: ::std::option::Option<::std::string::String>,
    }
    impl BadWhitelist {
        pub fn builder() -> BadWhitelistBuilder {
            BadWhitelistBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to perform batch delete on alerts."]
    pub struct BatchDeleteAlertsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. list of alert IDs."]
        pub alert_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with."]
        pub customer_id: ::std::option::Option<::std::string::String>,
    }
    impl BatchDeleteAlertsRequest {
        pub fn builder() -> BatchDeleteAlertsRequestBuilder {
            BatchDeleteAlertsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to batch delete operation on alerts."]
    pub struct BatchDeleteAlertsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedAlertStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status details for each failed alert_id."]
        pub failed_alert_status:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Status>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successAlertIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The successful list of alert IDs."]
        pub success_alert_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchDeleteAlertsResponse {
        pub fn builder() -> BatchDeleteAlertsResponseBuilder {
            BatchDeleteAlertsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to perform batch undelete on alerts."]
    pub struct BatchUndeleteAlertsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. list of alert IDs."]
        pub alert_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with."]
        pub customer_id: ::std::option::Option<::std::string::String>,
    }
    impl BatchUndeleteAlertsRequest {
        pub fn builder() -> BatchUndeleteAlertsRequestBuilder {
            BatchUndeleteAlertsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to batch undelete operation on alerts."]
    pub struct BatchUndeleteAlertsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedAlertStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status details for each failed alert_id."]
        pub failed_alert_status:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Status>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successAlertIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The successful list of alert IDs."]
        pub success_alert_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchUndeleteAlertsResponse {
        pub fn builder() -> BatchUndeleteAlertsResponseBuilder {
            BatchUndeleteAlertsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a Cloud Pubsub topic. To register for notifications, the owner of the topic must grant `alerts-api-push-notifications@system.gserviceaccount.com` the `projects.topics.publish` permission."]
    pub struct CloudPubsubTopic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payloadFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The format of the payload that would be sent. If not specified the format will be JSON."]
        pub payload_format: ::std::option::Option<CloudPubsubTopicPayloadFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `name` field of a Cloud Pubsub [Topic] (https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic)."]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl CloudPubsubTopic {
        pub fn builder() -> CloudPubsubTopicBuilder {
            CloudPubsubTopicBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The format of the payload that would be sent. If not specified the format will be JSON."]
    pub enum CloudPubsubTopicPayloadFormatEnum {
        #[serde(rename = "PAYLOAD_FORMAT_UNSPECIFIED")]
        #[doc = "Payload format is not specified (will use JSON as default)."]
        PayloadFormatUnspecified,
        #[serde(rename = "JSON")]
        #[doc = "Use JSON."]
        Json,
    }
    impl ::std::default::Default for CloudPubsubTopicPayloadFormatEnum {
        fn default() -> Self {
            Self::PayloadFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a CSV file attachment, as a list of column headers and a list of data rows."]
    pub struct Csv {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of data rows in a CSV file, as string arrays rather than as a single comma-separated string."]
        pub data_rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CsvRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of headers for data columns in a CSV file."]
        pub headers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Csv {
        pub fn builder() -> CsvBuilder {
            CsvBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a single data row in a CSV file."]
    pub struct CsvRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data entries in a CSV file row, as a string array rather than a single comma-separated string."]
        pub entries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CsvRow {
        pub fn builder() -> CsvRowBuilder {
            CsvRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A mobile device compromised alert. Derived from audit logs."]
    pub struct DeviceCompromised {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user this alert was created for."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of security events."]
        pub events: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<DeviceCompromisedSecurityDetail>>,
        >,
    }
    impl DeviceCompromised {
        pub fn builder() -> DeviceCompromisedBuilder {
            DeviceCompromisedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed information of a single MDM device compromised event."]
    pub struct DeviceCompromisedSecurityDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceCompromisedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device compromised state. Possible values are \"`Compromised`\" or \"`Not Compromised`\"."]
        pub device_compromised_state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device ID."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The model of the device."]
        pub device_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the device."]
        pub device_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosVendorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for iOS, empty for others."]
        pub ios_vendor_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device resource ID."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The serial number of the device."]
        pub serial_number: ::std::option::Option<::std::string::String>,
    }
    impl DeviceCompromisedSecurityDetail {
        pub fn builder() -> DeviceCompromisedSecurityDetailBuilder {
            DeviceCompromisedSecurityDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alerts that get triggered on violations of Data Loss Prevention (DLP) rules."]
    pub struct DlpRuleViolation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ruleViolationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the violated DLP rule. Admins can use the predefined detectors provided by Google Cloud DLP https://cloud.google.com/dlp/ when setting up a DLP rule. Matched Cloud DLP detectors in this violation if any will be captured in the MatchInfo.predefined_detector."]
        pub rule_violation_info: ::std::option::Option<::std::boxed::Box<RuleViolationInfo>>,
    }
    impl DlpRuleViolation {
        pub fn builder() -> DlpRuleViolationBuilder {
            DlpRuleViolationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Domain ID of Gmail phishing alerts."]
    pub struct DomainId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerPrimaryDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary domain for the customer."]
        pub customer_primary_domain: ::std::option::Option<::std::string::String>,
    }
    impl DomainId {
        pub fn builder() -> DomainIdBuilder {
            DomainIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A takeout operation for the entire domain was initiated by an admin. Derived from audit logs."]
    pub struct DomainWideTakeoutInitiated {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the admin who initiated the takeout."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "takeoutRequestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The takeout request ID."]
        pub takeout_request_id: ::std::option::Option<::std::string::String>,
    }
    impl DomainWideTakeoutInitiated {
        pub fn builder() -> DomainWideTakeoutInitiatedBuilder {
            DomainWideTakeoutInitiatedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a message in phishing spike alert."]
    pub struct GmailMessageInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachmentsSha256Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `SHA256` hash of email's attachment and all MIME parts."]
        pub attachments_sha256_hash: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date the malicious email was sent."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5HashMessageBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash of the message body text."]
        pub md5_hash_message_body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5HashSubject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MD5 Hash of email's subject (only available for reported emails)."]
        pub md5_hash_subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageBodySnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet of the message body text (only available for reported emails)."]
        pub message_body_snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message ID."]
        pub message_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipient")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recipient of this email."]
        pub recipient: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subjectText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email subject text (only available for reported emails)."]
        pub subject_text: ::std::option::Option<::std::string::String>,
    }
    impl GmailMessageInfo {
        pub fn builder() -> GmailMessageInfoBuilder {
            GmailMessageInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An incident reported by Google Operations for a G Suite application."]
    pub struct GoogleOperations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedUserEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of emails which correspond to the users directly affected by the incident."]
        pub affected_user_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachmentData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Application-specific data for an incident, provided when the G Suite application which reported the incident cannot be completely restored to a valid state."]
        pub attachment_data: ::std::option::Option<::std::boxed::Box<Attachment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed, freeform incident description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A header to display above the incident message. Typcially used to attach a localized notice on the timeline for followup comms translations."]
        pub header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A one-line incident description."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleOperations {
        pub fn builder() -> GoogleOperationsBuilder {
            GoogleOperationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for an alert feedback listing request."]
    pub struct ListAlertFeedbackResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of alert feedback. Feedback entries for each alert are ordered by creation time descending."]
        pub feedback: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AlertFeedback>>>,
    }
    impl ListAlertFeedbackResponse {
        pub fn builder() -> ListAlertFeedbackResponseBuilder {
            ListAlertFeedbackResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for an alert listing request."]
    pub struct ListAlertsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alerts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of alerts."]
        pub alerts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Alert>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the next page. If not empty, indicates that there may be more alerts that match the listing request; this value can be used in a subsequent ListAlertsRequest to get alerts continuing from last result of the current list call."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAlertsResponse {
        pub fn builder() -> ListAlertsResponseBuilder {
            ListAlertsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The details of the login action."]
    pub struct LoginDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The human-readable IP address (for example, `11.22.33.44`) that is associated with the warning event."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loginTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The successful login time that is associated with the warning event. This isn't present for blocked login attempts."]
        pub login_time: ::std::option::Option<::std::string::String>,
    }
    impl LoginDetails {
        pub fn builder() -> LoginDetailsBuilder {
            LoginDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Proto for all phishing alerts with common payload. Supported types are any of the following: * User reported phishing * User reported spam spike * Suspicious message reported * Phishing reclassification * Malware reclassification * Gmail potential employee spoofing"]
    pub struct MailPhishing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain ID."]
        pub domain_id: ::std::option::Option<::std::boxed::Box<DomainId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInternal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, the email originated from within the organization."]
        pub is_internal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maliciousEntity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        pub malicious_entity: ::std::option::Option<::std::boxed::Box<MaliciousEntity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of messages contained by this alert."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmailMessageInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemActionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System actions on the messages."]
        pub system_action_type: ::std::option::Option<MailPhishingSystemActionTypeEnum>,
    }
    impl MailPhishing {
        pub fn builder() -> MailPhishingBuilder {
            MailPhishingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "System actions on the messages."]
    pub enum MailPhishingSystemActionTypeEnum {
        #[serde(rename = "SYSTEM_ACTION_TYPE_UNSPECIFIED")]
        #[doc = "System action is unspecified."]
        SystemActionTypeUnspecified,
        #[serde(rename = "NO_OPERATION")]
        #[doc = "No operation."]
        NoOperation,
        #[serde(rename = "REMOVED_FROM_INBOX")]
        #[doc = "Messages were removed from the inbox."]
        RemovedFromInbox,
    }
    impl ::std::default::Default for MailPhishingSystemActionTypeEnum {
        fn default() -> Self {
            Self::SystemActionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity whose actions triggered a Gmail phishing alert."]
    pub struct MaliciousEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The header from display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actor who triggered a gmail phishing alert."]
        pub entity: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sender email address."]
        pub from_header: ::std::option::Option<::std::string::String>,
    }
    impl MaliciousEntity {
        pub fn builder() -> MaliciousEntityBuilder {
            MaliciousEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Proto that contains match information from the condition part of the rule."]
    pub struct MatchInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predefinedDetector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For matched detector predefined by Google."]
        pub predefined_detector: ::std::option::Option<::std::boxed::Box<PredefinedDetectorInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userDefinedDetector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For matched detector defined by administrators."]
        pub user_defined_detector:
            ::std::option::Option<::std::boxed::Box<UserDefinedDetectorInfo>>,
    }
    impl MatchInfo {
        pub fn builder() -> MatchInfoBuilder {
            MatchInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for callback notifications. For more details see [Google Workspace Alert Notification](/admin-sdk/alertcenter/guides/notifications)."]
    pub struct Notification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudPubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Google Cloud Pub/sub topic destination."]
        pub cloud_pubsub_topic: ::std::option::Option<::std::boxed::Box<CloudPubsubTopic>>,
    }
    impl Notification {
        pub fn builder() -> NotificationBuilder {
            NotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alert for a spike in user reported phishing. *Warning*: This type has been deprecated. Use [MailPhishing](/admin-sdk/alertcenter/reference/rest/v1beta1/MailPhishing) instead."]
    pub struct PhishingSpike {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain ID."]
        pub domain_id: ::std::option::Option<::std::boxed::Box<DomainId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInternal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, the email originated from within the organization."]
        pub is_internal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maliciousEntity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        pub malicious_entity: ::std::option::Option<::std::boxed::Box<MaliciousEntity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of messages contained by this alert."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmailMessageInfo>>>,
    }
    impl PhishingSpike {
        pub fn builder() -> PhishingSpikeBuilder {
            PhishingSpikeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detector provided by Google."]
    pub struct PredefinedDetectorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectorName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name that uniquely identifies the detector."]
        pub detector_name: ::std::option::Option<::std::string::String>,
    }
    impl PredefinedDetectorInfo {
        pub fn builder() -> PredefinedDetectorInfoBuilder {
            PredefinedDetectorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Requests for one application that needs default SQL setup."]
    pub struct RequestInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appDeveloperEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of app developers who triggered notifications for above application."]
        pub app_developer_email: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The application that requires the SQL setup."]
        pub app_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberOfRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Number of requests sent for this application to set up default SQL instance."]
        pub number_of_requests: ::std::option::Option<::std::string::String>,
    }
    impl RequestInfo {
        pub fn builder() -> RequestInfoBuilder {
            RequestInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Proto that contains resource information."]
    pub struct ResourceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Drive file ID."]
        pub document_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the resource, for example email subject, or document title."]
        pub resource_title: ::std::option::Option<::std::string::String>,
    }
    impl ResourceInfo {
        pub fn builder() -> ResourceInfoBuilder {
            ResourceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Proto that contains rule information."]
    pub struct RuleInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User provided name of the rule."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name that uniquely identifies the rule."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl RuleInfo {
        pub fn builder() -> RuleInfoBuilder {
            RuleInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Common alert information about violated rules that are configured by G Suite administrators."]
    pub struct RuleViolationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source of the data."]
        pub data_source: ::std::option::Option<RuleViolationInfoDataSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of matches that were found in the resource content."]
        pub match_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatchInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource recipients. For Drive, they are grantees that the Drive file was shared with at the time of rule triggering. Valid values include user emails, group emails, domains, or 'anyone' if the file was publicly accessible. If the file was private the recipients list will be empty. For Gmail, they are emails of the users or groups that the Gmail message was sent to."]
        pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the resource which violated the rule."]
        pub resource_info: ::std::option::Option<::std::boxed::Box<ResourceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ruleInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the violated rule."]
        pub rule_info: ::std::option::Option<::std::boxed::Box<RuleInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suppressedActionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actions suppressed due to other actions with higher priority."]
        pub suppressed_action_types:
            ::std::option::Option<::std::vec::Vec<RuleViolationInfoSuppressedActionTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trigger of the rule."]
        pub trigger: ::std::option::Option<RuleViolationInfoTriggerEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggeredActionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actions applied as a consequence of the rule being triggered."]
        pub triggered_action_types:
            ::std::option::Option<::std::vec::Vec<RuleViolationInfoTriggeredActionTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggeringUserEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who caused the violation. Value could be empty if not applicable, for example, a violation found by drive continuous scan."]
        pub triggering_user_email: ::std::option::Option<::std::string::String>,
    }
    impl RuleViolationInfo {
        pub fn builder() -> RuleViolationInfoBuilder {
            RuleViolationInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Source of the data."]
    pub enum RuleViolationInfoDataSourceEnum {
        #[serde(rename = "DATA_SOURCE_UNSPECIFIED")]
        #[doc = "Data source is unspecified."]
        DataSourceUnspecified,
        #[serde(rename = "DRIVE")]
        #[doc = "Drive data source."]
        Drive,
    }
    impl ::std::default::Default for RuleViolationInfoDataSourceEnum {
        fn default() -> Self {
            Self::DataSourceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum RuleViolationInfoSuppressedActionTypesEnum {
        #[serde(rename = "ACTION_TYPE_UNSPECIFIED")]
        #[doc = "Action type is unspecified."]
        ActionTypeUnspecified,
        #[serde(rename = "DRIVE_BLOCK_EXTERNAL_SHARING")]
        #[doc = "Block sharing a file externally."]
        DriveBlockExternalSharing,
        #[serde(rename = "DRIVE_WARN_ON_EXTERNAL_SHARING")]
        #[doc = "Show a warning message when sharing a file externally."]
        DriveWarnOnExternalSharing,
        #[serde(rename = "ALERT")]
        #[doc = "Send alert."]
        Alert,
        #[serde(rename = "RULE_ACTIVATE")]
        #[doc = "Activate Rule Action"]
        RuleActivate,
        #[serde(rename = "RULE_DEACTIVATE")]
        #[doc = "Deactivate Rule Action"]
        RuleDeactivate,
    }
    impl ::std::default::Default for RuleViolationInfoSuppressedActionTypesEnum {
        fn default() -> Self {
            Self::ActionTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Trigger of the rule."]
    pub enum RuleViolationInfoTriggerEnum {
        #[serde(rename = "TRIGGER_UNSPECIFIED")]
        #[doc = "Trigger is unspecified."]
        TriggerUnspecified,
        #[serde(rename = "DRIVE_SHARE")]
        #[doc = "A Drive file is shared."]
        DriveShare,
    }
    impl ::std::default::Default for RuleViolationInfoTriggerEnum {
        fn default() -> Self {
            Self::TriggerUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum RuleViolationInfoTriggeredActionTypesEnum {
        #[serde(rename = "ACTION_TYPE_UNSPECIFIED")]
        #[doc = "Action type is unspecified."]
        ActionTypeUnspecified,
        #[serde(rename = "DRIVE_BLOCK_EXTERNAL_SHARING")]
        #[doc = "Block sharing a file externally."]
        DriveBlockExternalSharing,
        #[serde(rename = "DRIVE_WARN_ON_EXTERNAL_SHARING")]
        #[doc = "Show a warning message when sharing a file externally."]
        DriveWarnOnExternalSharing,
        #[serde(rename = "ALERT")]
        #[doc = "Send alert."]
        Alert,
        #[serde(rename = "RULE_ACTIVATE")]
        #[doc = "Activate Rule Action"]
        RuleActivate,
        #[serde(rename = "RULE_DEACTIVATE")]
        #[doc = "Deactivate Rule Action"]
        RuleDeactivate,
    }
    impl ::std::default::Default for RuleViolationInfoTriggeredActionTypesEnum {
        fn default() -> Self {
            Self::ActionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Customer-level settings."]
    pub struct Settings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of notifications."]
        pub notifications: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Notification>>>,
    }
    impl Settings {
        pub fn builder() -> SettingsBuilder {
            SettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A state-sponsored attack alert. Derived from audit logs."]
    pub struct StateSponsoredAttack {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user this incident was created for."]
        pub email: ::std::option::Option<::std::string::String>,
    }
    impl StateSponsoredAttack {
        pub fn builder() -> StateSponsoredAttackBuilder {
            StateSponsoredAttackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
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
    #[doc = "A mobile suspicious activity alert. Derived from audit logs."]
    pub struct SuspiciousActivity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user this alert was created for."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of security events."]
        pub events: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<SuspiciousActivitySecurityDetail>>,
        >,
    }
    impl SuspiciousActivity {
        pub fn builder() -> SuspiciousActivityBuilder {
            SuspiciousActivityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed information of a single MDM suspicious activity event."]
    pub struct SuspiciousActivitySecurityDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device ID."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The model of the device."]
        pub device_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceProperty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device property which was changed."]
        pub device_property: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the device."]
        pub device_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosVendorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for iOS, empty for others."]
        pub ios_vendor_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new value of the device property after the change."]
        pub new_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The old value of the device property before the change."]
        pub old_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device resource ID."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The serial number of the device."]
        pub serial_number: ::std::option::Option<::std::string::String>,
    }
    impl SuspiciousActivitySecurityDetail {
        pub fn builder() -> SuspiciousActivitySecurityDetailBuilder {
            SuspiciousActivitySecurityDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to undelete a specific alert that was marked for deletion."]
    pub struct UndeleteAlertRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
        pub customer_id: ::std::option::Option<::std::string::String>,
    }
    impl UndeleteAlertRequest {
        pub fn builder() -> UndeleteAlertRequestBuilder {
            UndeleteAlertRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user."]
        pub email_address: ::std::option::Option<::std::string::String>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detector defined by administrators."]
    pub struct UserDefinedDetectorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the detector."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name that uniquely identifies the detector."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl UserDefinedDetectorInfo {
        pub fn builder() -> UserDefinedDetectorInfoBuilder {
            UserDefinedDetectorInfoBuilder::default()
        }
    }
}
