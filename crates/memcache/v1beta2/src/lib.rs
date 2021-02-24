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
    pub mod projects {
        pub mod resources {
            pub mod locations {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub mod resources {
                    pub mod instances {
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
                                    #[serde(rename = "instanceId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The logical name of the Memcached instance in the user project with the following restrictions: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-40 characters. * Must end with a number or a letter. * Must be unique within the user project / location If any of the above are not met, will raise an invalid argument error."]
                                    pub instance_id: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "List filter. For example, exclude all Memcached instances with name as my-instance by specifying \"name != my-instance\"."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Sort results. Supported values are \"name\", \"name desc\" or \"\" (unsorted)."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of items to return. If not specified, a default value of 1000 will be used by the service. Regardless of the page_size value, the response may include a partial list and a caller should only rely on response's next_page_token to determine if there are more instances left to be queried."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The next_page_token value returned from a previous List request, if any."]
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Mask of fields to update. * `displayName`"]
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
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ApplyParameters."]
    pub struct ApplyParametersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyAll")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to apply instance-level parameter group to all nodes. If set to true, will explicitly restrict users from specifying any nodes, and apply parameter group updates to all nodes within the instance."]
        pub apply_all: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Nodes to which we should apply the instance-level parameter group."]
        pub node_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ApplyParametersRequest {
        pub fn builder() -> ApplyParametersRequestBuilder {
            ApplyParametersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ApplySoftwareUpdate."]
    pub struct ApplySoftwareUpdateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyAll")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to apply the update to all nodes. If set to true, will explicitly restrict users from specifying any nodes, and apply software update to all nodes (where applicable) within the instance."]
        pub apply_all: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Nodes to which we should apply the update to. Note all the selected nodes are updated in parallel."]
        pub node_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ApplySoftwareUpdateRequest {
        pub fn builder() -> ApplySoftwareUpdateRequestBuilder {
            ApplySoftwareUpdateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct CancelOperationRequest {}
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Time window specified for daily operations."]
    pub struct DailyCycle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Duration of the time window, set by service producer."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time within the day to start the operations."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DailyCycle {
        pub fn builder() -> DailyCycleBuilder {
            DailyCycleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
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
    #[doc = "DenyMaintenancePeriod definition. Maintenance is forbidden within the deny period. The start_date must be less than the end_date."]
    pub struct DenyMaintenancePeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deny period end date. This can be: * A full date, with non-zero year, month and day values. * A month and day value, with a zero year. Allows recurring deny periods each year. Date matching this period will have to be before the end."]
        pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deny period start date. This can be: * A full date, with non-zero year, month and day values. * A month and day value, with a zero year. Allows recurring deny periods each year. Date matching this period will have to be the same or after the start."]
        pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time in UTC when the Blackout period starts on start_date and ends on end_date. This can be: * Full time. * All zeros for 00:00:00 UTC"]
        pub time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DenyMaintenancePeriod {
        pub fn builder() -> DenyMaintenancePeriodBuilder {
            DenyMaintenancePeriodBuilder::default()
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
    #[doc = "Metadata for the given google.cloud.location.Location."]
    pub struct GoogleCloudMemcacheV1beta2LocationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by GCE. These keys can be specified in the `zones` field when creating a Memcached instance."]
        pub available_zones: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ZoneMetadata>>,
        >,
    }
    impl GoogleCloudMemcacheV1beta2LocationMetadata {
        pub fn builder() -> GoogleCloudMemcacheV1beta2LocationMetadataBuilder {
            GoogleCloudMemcacheV1beta2LocationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the metadata of a long-running operation."]
    pub struct GoogleCloudMemcacheV1beta2OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. API version used to start the operation."]
        pub api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancelRequested")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
        pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when the operation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when the operation finished running."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Human-readable status of the operation, if any."]
        pub status_detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Server-defined resource path for the target of the operation."]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the verb executed by the operation."]
        pub verb: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudMemcacheV1beta2OperationMetadata {
        pub fn builder() -> GoogleCloudMemcacheV1beta2OperationMetadataBuilder {
            GoogleCloudMemcacheV1beta2OperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerDefinedName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "consumer_defined_name is the name that is set by the consumer. On the other hand Name field represents system-assigned id of an instance so consumers are not necessarily aware of it. consumer_defined_name is used for notification/UI purposes for consumer to recognize their instances."]
        pub consumer_defined_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when the resource was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Resource labels to represent user provided metadata. Each label is a key-value pair, where both the key and the value are arbitrary strings provided by the user."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenancePolicyNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The MaintenancePolicies that have been attached to the instance. The key must be of the type name of the oneof policy name defined in MaintenancePolicy, and the referenced policy must define the same policy type. For complete details of MaintenancePolicy, please refer to go/cloud-saas-mw-ug."]
        pub maintenance_policy_names:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenanceSchedules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MaintenanceSchedule contains the scheduling information of published maintenance schedule with same key as software_versions."]
        pub maintenance_schedules: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<
                    GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSchedule,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenanceSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The MaintenanceSettings associated with instance."]
        pub maintenance_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique name of the resource. It uses the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Custom string attributes used primarily to expose producer-specific information in monitoring dashboards. See go/get-instance-metadata."]
        pub producer_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisionedResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The list of data plane resources provisioned for this instance, e.g. compute VMs. See go/get-instance-metadata."]
        pub provisioned_resources: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResource,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slmInstanceTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the SLM instance template. Only populated when updating SLM instances via SSA's Actuation service adaptor. Service producers with custom control plane (e.g. Cloud SQL) doesn't need to populate this field. Instead they should use software_versions."]
        pub slm_instance_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sloMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. SLO metadata for instance classification in the Standardized dataplane SLO platform. See go/cloud-ssa-standard-slo for feature description."]
        pub slo_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "softwareVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Software versions that are used to deploy this instance. This can be mutated by rollout services."]
        pub software_versions:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Current lifecycle state of the resource (e.g. if it's being created or ready to use)."]
        pub state:
            ::std::option::Option<GoogleCloudSaasacceleratorManagementProvidersV1InstanceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the associated GCP tenant project. See go/get-instance-metadata."]
        pub tenant_project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when the resource was last modified."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1Instance {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1InstanceBuilder {
            GoogleCloudSaasacceleratorManagementProvidersV1InstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Current lifecycle state of the resource (e.g. if it's being created or ready to use)."]
    pub enum GoogleCloudSaasacceleratorManagementProvidersV1InstanceStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Instance is being created."]
        Creating,
        #[serde(rename = "READY")]
        #[doc = "Instance has been created and is ready to use."]
        Ready,
        #[serde(rename = "UPDATING")]
        #[doc = "Instance is being updated."]
        Updating,
        #[serde(rename = "REPAIRING")]
        #[doc = "Instance is unheathy and under repair."]
        Repairing,
        #[serde(rename = "DELETING")]
        #[doc = "Instance is being deleted."]
        Deleting,
        #[serde(rename = "ERROR")]
        #[doc = "Instance encountered an error and is in indeterministic state."]
        Error,
    }
    impl ::std::default::Default for GoogleCloudSaasacceleratorManagementProvidersV1InstanceStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Maintenance schedule which is exposed to customer and potentially end user, indicating published upcoming future maintenance schedule"]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReschedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can this scheduled update be rescheduled? By default, it's true and API needs to do explicitly check whether it's set, if it's set as false explicitly, it's false"]
        pub can_reschedule: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scheduled end time for the maintenance."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rolloutManagementPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rollout management policy this maintenance schedule is associated with. When doing reschedule update request, the reschedule should be against this given policy."]
        pub rollout_management_policy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleDeadlineTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "schedule_deadline_time is the time deadline any schedule start time cannot go beyond, including reschedule. It's normally the initial schedule start time plus a week. If the reschedule type is next window, simply take this value as start time. If reschedule type is IMMEDIATELY or BY_TIME, current or selected time cannot go beyond this deadline."]
        pub schedule_deadline_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scheduled start time for the maintenance."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSchedule {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceScheduleBuilder
        {
            GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Maintenance settings associated with instance. Allows service producers and end users to assign settings that controls maintenance on this instance."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Exclude instance from maintenance. When true, rollout service will not attempt maintenance on the instance. Rollout service will include the instance in reported rollout progress as not attempted."]
        pub exclude: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRollback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If the update call is triggered from rollback, set the value as true."]
        pub is_rollback: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenancePolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The MaintenancePolicies that have been attached to the instance. The key must be of the type name of the oneof policy name defined in MaintenancePolicy, and the embedded policy must define the same policy type. For complete details of MaintenancePolicy, please refer to go/cloud-saas-mw-ug. If only the name is needed (like in the deprecated Instance.maintenance_policy_names field) then only populate MaintenancePolicy.name."]
        pub maintenance_policies: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<MaintenancePolicy>>,
        >,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettings {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettingsBuilder
        {
            GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Node information for custom per-node SLO implementations. SSA does not support per-node SLO, but producers can populate per-node information in SloMetadata for custom precomputations. SSA Eligibility Exporter will emit per-node metric based on this information."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default node is eligible if instance is eligible. But individual node might be excluded from SLO by adding entry here. For semantic see SloMetadata.exclusions. If both instance and node level exclusions are present for time period, the node level's reason will be reported by Eligibility Exporter."]
        pub exclusions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the node, if different from instance location."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the node. This should be equal to SaasInstanceNode.node_id."]
        pub node_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadata {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadataBuilder {
            GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes provisioned dataplane resources."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the resource. This can be either a GCP resource or a custom one (e.g. another cloud provider's VM). For GCP compute resources use singular form of the names listed in GCP compute API documentation (https://cloud.google.com/compute/docs/reference/rest/v1/), prefixed with 'compute-', for example: 'compute-instance', 'compute-disk', 'compute-autoscaler'."]
        pub resource_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL identifying the resource, e.g. \"https://www.googleapis.com/compute/v1/projects/...)\"."]
        pub resource_url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResource {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResourceBuilder
        {
            GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SloEligibility is a tuple containing eligibility value: true if an instance is eligible for SLO calculation or false if it should be excluded from all SLO-related calculations along with a user-defined reason."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eligible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether an instance is eligible or ineligible."]
        pub eligible: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-defined reason for the current value of instance eligibility. Usually, this can be directly mapped to the internal state. An empty reason is allowed."]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1SloEligibilityBuilder {
            GoogleCloudSaasacceleratorManagementProvidersV1SloEligibilityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SloExclusion represents an exclusion in SLI calculation applies to all SLOs."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exclusion duration. No restrictions on the possible values. When an ongoing operation is taking longer than initially expected, an existing entry in the exclusion list can be updated by extending the duration. This is supported by the subsystem exporting eligibility data as long as such extension is committed at least 10 minutes before the original exclusion expiration - otherwise it is possible that there will be \"gaps\" in the exclusion application in the exported timeseries."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable reason for the exclusion. This should be a static string (e.g. \"Disruptive update in progress\") and should not contain dynamically generated data (e.g. instance name). Can be left empty."]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sliName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of an SLI that this exclusion applies to. Can be left empty, signaling that the instance should be excluded from all SLIs defined in the service SLO configuration."]
        pub sli_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of the exclusion. No alignment (e.g. to a full minute) needed."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1SloExclusionBuilder {
            GoogleCloudSaasacceleratorManagementProvidersV1SloExclusionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SloMetadata contains resources required for proper SLO classification of the instance."]
    pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eligibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-defined instance eligibility."]
        pub eligibility: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of SLO exclusion windows. When multiple entries in the list match (matching the exclusion time-window against current time point) the exclusion reason used in the first matching entry will be published. It is not needed to include expired exclusion in this list, as only the currently applicable exclusions are taken into account by the eligibility exporting subsystem (the historical state of exclusions will be reflected in the historically produced timeseries regardless of the current state). This field can be used to mark the instance as temporary ineligible for the purpose of SLO calculation. For permanent instance SLO exclusion, use of custom instance eligibility is recommended. See 'eligibility' field below."]
        pub exclusions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of nodes. Some producers need to use per-node metadata to calculate SLO. This field allows such producers to publish per-node SLO meta data, which will be consumed by SSA Eligibility Exporter and published in the form of per node metric to Monarch."]
        pub nodes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadata>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the SLO tier the Instance belongs to. This name will be expected to match the tiers specified in the service SLO configuration. Field is mandatory and must not be empty."]
        pub tier: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudSaasacceleratorManagementProvidersV1SloMetadata {
        pub fn builder() -> GoogleCloudSaasacceleratorManagementProvidersV1SloMetadataBuilder {
            GoogleCloudSaasacceleratorManagementProvidersV1SloMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used."]
        pub authorized_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the instance was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discoveryEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Endpoint for Discovery API"]
        pub discovery_endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User provided name for the instance only used for display purposes. Cannot be more than 80 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of messages that describe current statuses of memcached instance."]
        pub instance_messages:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InstanceMessage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memcacheFullVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be \"memcached-1.5.16\"."]
        pub memcache_full_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memcacheNodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. List of Memcached nodes. Refer to [Node] message for more details."]
        pub memcache_nodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Node>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memcacheVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the latest supported minor version."]
        pub memcache_version: ::std::option::Option<InstanceMemcacheVersionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which zones Memcached nodes within an instances should be provisioned in. Refer to [zones] field for more details."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Configuration for Memcached nodes."]
        pub node_config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Number of nodes in the Memcached instance."]
        pub node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional: User defined parameters to apply to the memcached process on each node."]
        pub parameters: ::std::option::Option<::std::boxed::Box<MemcacheParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The state of this Memcached instance."]
        pub state: ::std::option::Option<InstanceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Returns true if there is an update waiting to be applied"]
        pub update_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the instance was updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zones where Memcached nodes should be provisioned in. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance."]
        pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Instance {
        pub fn builder() -> InstanceBuilder {
            InstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the latest supported minor version."]
    pub enum InstanceMemcacheVersionEnum {
        #[serde(rename = "MEMCACHE_VERSION_UNSPECIFIED")]
        #[doc = ""]
        MemcacheVersionUnspecified,
        #[serde(rename = "MEMCACHE_1_5")]
        #[doc = "Memcached 1.5 version."]
        Memcache15,
    }
    impl ::std::default::Default for InstanceMemcacheVersionEnum {
        fn default() -> Self {
            Self::MemcacheVersionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The state of this Memcached instance."]
    pub enum InstanceStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State not set."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Memcached instance is being created."]
        Creating,
        #[serde(rename = "READY")]
        #[doc = "Memcached instance has been created and ready to be used."]
        Ready,
        #[serde(rename = "DELETING")]
        #[doc = "Memcached instance is being deleted."]
        Deleting,
        #[serde(rename = "PERFORMING_MAINTENANCE")]
        #[doc = "Memcached instance is going through maintenance, e.g. data plane rollout."]
        PerformingMaintenance,
    }
    impl ::std::default::Default for InstanceStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InstanceMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A code that correspond to one type of user-facing message."]
        pub code: ::std::option::Option<InstanceMessageCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message on memcached instance which will be exposed to users."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl InstanceMessage {
        pub fn builder() -> InstanceMessageBuilder {
            InstanceMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A code that correspond to one type of user-facing message."]
    pub enum InstanceMessageCodeEnum {
        #[serde(rename = "CODE_UNSPECIFIED")]
        #[doc = "Message Code not set."]
        CodeUnspecified,
        #[serde(rename = "ZONE_DISTRIBUTION_UNBALANCED")]
        #[doc = "Memcached nodes are distributed unevenly."]
        ZoneDistributionUnbalanced,
    }
    impl ::std::default::Default for InstanceMessageCodeEnum {
        fn default() -> Self {
            Self::CodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListInstances."]
    pub struct ListInstancesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Memcached instances in the project in the specified location, or across all locations. If the `location_id` in the parent field of the request is \"-\", all regions available to the project are queried, and the results aggregated."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unreachable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locations that could not be reached."]
        pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ListInstancesResponse {
        pub fn builder() -> ListInstancesResponseBuilder {
            ListInstancesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Locations.ListLocations."]
    pub struct ListLocationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of locations that matches the specified filter in the request."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListLocationsResponse {
        pub fn builder() -> ListLocationsResponseBuilder {
            ListLocationsResponseBuilder::default()
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
    #[doc = "A resource that represents Google Cloud Platform location."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines policies to service maintenance events."]
    pub struct MaintenancePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the resource was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of what this policy is for. Create/Update methods return INVALID_ARGUMENT if the length is greater than 512."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Resource labels to represent user provided metadata. Each label is a key-value pair, where both the key and the value are arbitrary strings provided by the user."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. MaintenancePolicy name using the form: `projects/{project_id}/locations/{location_id}/maintenancePolicies/{maintenance_policy_id}` where {project_id} refers to a GCP consumer project ID, {location_id} refers to a GCP region/zone, {maintenance_policy_id} must be 1-63 characters long and match the regular expression `[a-z0-9]([-a-z0-9]*[a-z0-9])?`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The state of the policy."]
        pub state: ::std::option::Option<MaintenancePolicyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maintenance policy applicable to instance update."]
        pub update_policy: ::std::option::Option<::std::boxed::Box<UpdatePolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the resource was updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl MaintenancePolicy {
        pub fn builder() -> MaintenancePolicyBuilder {
            MaintenancePolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The state of the policy."]
    pub enum MaintenancePolicyStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "READY")]
        #[doc = "Resource is ready to be used."]
        Ready,
        #[serde(rename = "DELETING")]
        #[doc = "Resource is being deleted. It can no longer be attached to instances."]
        Deleting,
    }
    impl ::std::default::Default for MaintenancePolicyStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "MaintenanceWindow definition."]
    pub struct MaintenanceWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dailyCycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Daily cycle."]
        pub daily_cycle: ::std::option::Option<::std::boxed::Box<DailyCycle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weeklyCycle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Weekly cycle."]
        pub weekly_cycle: ::std::option::Option<::std::boxed::Box<WeeklyCycle>>,
    }
    impl MaintenanceWindow {
        pub fn builder() -> MaintenanceWindowBuilder {
            MaintenanceWindowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MemcacheParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique ID associated with this set of parameters. Users can use this id to determine if the parameters associated with the instance differ from the parameters associated with the nodes and any action needs to be taken to apply parameters on nodes."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User defined set of parameters to use in the memcached process."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl MemcacheParameters {
        pub fn builder() -> MemcacheParametersBuilder {
            MemcacheParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Node {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Hostname or IP address of the Memcached node used by the clients to connect to the Memcached server on this node."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Identifier of the Memcached node. The node id does not include project or location like the Memcached instance name."]
        pub node_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User defined parameters currently applied to the node."]
        pub parameters: ::std::option::Option<::std::boxed::Box<MemcacheParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The port number of the Memcached server on this node."]
        pub port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Current state of the Memcached node."]
        pub state: ::std::option::Option<NodeStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Returns true if there is an update waiting to be applied"]
        pub update_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Location (GCP Zone) for the Memcached node."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl Node {
        pub fn builder() -> NodeBuilder {
            NodeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Current state of the Memcached node."]
    pub enum NodeStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Node state is not set."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "Node is being created."]
        Creating,
        #[serde(rename = "READY")]
        #[doc = "Node has been created and ready to be used."]
        Ready,
        #[serde(rename = "DELETING")]
        #[doc = "Node is being deleted."]
        Deleting,
        #[serde(rename = "UPDATING")]
        #[doc = "Node is being updated."]
        Updating,
    }
    impl ::std::default::Default for NodeStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a Memcached Node."]
    pub struct NodeConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Number of cpus per Memcached node."]
        pub cpu_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memorySizeMb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Memory size in MiB for each Memcached node."]
        pub memory_size_mb: ::std::option::Option<::std::primitive::i64>,
    }
    impl NodeConfig {
        pub fn builder() -> NodeConfigBuilder {
            NodeConfigBuilder::default()
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
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
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
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
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
    #[doc = "Represents the metadata of a long-running operation."]
    pub struct OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. API version used to start the operation."]
        pub api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancelRequested")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
        pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when the operation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when the operation finished running."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Human-readable status of the operation, if any."]
        pub status_detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Server-defined resource path for the target of the operation."]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the verb executed by the operation."]
        pub verb: ::std::option::Option<::std::string::String>,
    }
    impl OperationMetadata {
        pub fn builder() -> OperationMetadataBuilder {
            OperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configure the schedule."]
    pub struct Schedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows to define schedule that runs specified day of the week."]
        pub day: ::std::option::Option<ScheduleDayEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Duration of the time window, set by service producer."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time within the window to start the operations."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl Schedule {
        pub fn builder() -> ScheduleBuilder {
            ScheduleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Allows to define schedule that runs specified day of the week."]
    pub enum ScheduleDayEnum {
        #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
        #[doc = "The day of the week is unspecified."]
        DayOfWeekUnspecified,
        #[serde(rename = "MONDAY")]
        #[doc = "Monday"]
        Monday,
        #[serde(rename = "TUESDAY")]
        #[doc = "Tuesday"]
        Tuesday,
        #[serde(rename = "WEDNESDAY")]
        #[doc = "Wednesday"]
        Wednesday,
        #[serde(rename = "THURSDAY")]
        #[doc = "Thursday"]
        Thursday,
        #[serde(rename = "FRIDAY")]
        #[doc = "Friday"]
        Friday,
        #[serde(rename = "SATURDAY")]
        #[doc = "Saturday"]
        Saturday,
        #[serde(rename = "SUNDAY")]
        #[doc = "Sunday"]
        Sunday,
    }
    impl ::std::default::Default for ScheduleDayEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
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
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct TimeOfDay {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        pub hours: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        pub seconds: ::std::option::Option<::std::primitive::i64>,
    }
    impl TimeOfDay {
        pub fn builder() -> TimeOfDayBuilder {
            TimeOfDayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for UpdateParameters."]
    pub struct UpdateParametersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters to apply to the instance."]
        pub parameters: ::std::option::Option<::std::boxed::Box<MemcacheParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Mask of fields to update."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateParametersRequest {
        pub fn builder() -> UpdateParametersRequestBuilder {
            UpdateParametersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Maintenance policy applicable to instance updates."]
    pub struct UpdatePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Relative scheduling channel applied to resource."]
        pub channel: ::std::option::Option<UpdatePolicyChannelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "denyMaintenancePeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deny Maintenance Period that is applied to resource to indicate when maintenance is forbidden. User can specify zero or more non-overlapping deny periods. For V1, Maximum number of deny_maintenance_periods is expected to be one."]
        pub deny_maintenance_periods:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DenyMaintenancePeriod>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "window")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maintenance window that is applied to resources covered by this policy."]
        pub window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
    }
    impl UpdatePolicy {
        pub fn builder() -> UpdatePolicyBuilder {
            UpdatePolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Relative scheduling channel applied to resource."]
    pub enum UpdatePolicyChannelEnum {
        #[serde(rename = "UPDATE_CHANNEL_UNSPECIFIED")]
        #[doc = "Unspecified channel."]
        UpdateChannelUnspecified,
        #[serde(rename = "EARLIER")]
        #[doc = "Early channel within a customer project."]
        Earlier,
        #[serde(rename = "LATER")]
        #[doc = "Later channel within a customer project."]
        Later,
    }
    impl ::std::default::Default for UpdatePolicyChannelEnum {
        fn default() -> Self {
            Self::UpdateChannelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Time window specified for weekly operations."]
    pub struct WeeklyCycle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User can specify multiple windows in a week. Minimum of 1 window."]
        pub schedule: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Schedule>>>,
    }
    impl WeeklyCycle {
        pub fn builder() -> WeeklyCycleBuilder {
            WeeklyCycleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ZoneMetadata {}
    impl ZoneMetadata {
        pub fn builder() -> ZoneMetadataBuilder {
            ZoneMetadataBuilder::default()
        }
    }
}
