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
    pub mod organizations {
        pub mod resources {
            pub mod locations {
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
                    pub mod workloads {
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
                                    #[serde(rename = "externalId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. A identifier associated with the workload and underlying projects which allows for the break down of billing costs for a workload. The value provided for the identifier will add a label to the workload and contained projects with the identifier as the value."]
                                    pub external_id: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "etag")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. The etag of the workload. If this is provided, it must match the server's etag."]
                                    pub etag: ::std::option::Option<::std::string::String>,
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
                                    #[doc = "A custom filter for filtering by properties of a workload. At this time, only filtering by labels is supported."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token returned from previous request. Page token contains context from previous request. Page token needs to be passed in the second and following requests."]
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
                                    #[doc = "Required. The list of fields to be updated."]
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
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Operation metadata to give request details of CreateWorkload."]
    pub struct GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceRegime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."]
        pub compliance_regime: ::std::option::Option<
            GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegimeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Time when the operation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The display name of the workload."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The parent of the workload."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadata {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataBuilder {
            GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."]
    pub enum GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegimeEnum {
        #[serde(rename = "COMPLIANCE_REGIME_UNSPECIFIED")]
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[serde(rename = "IL4")]
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[serde(rename = "CJIS")]
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[serde(rename = "FEDRAMP_HIGH")]
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[serde(rename = "FEDRAMP_MODERATE")]
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[serde(rename = "US_REGIONAL_ACCESS")]
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl ::std::default::Default
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegimeEnum
    {
        fn default() -> Self {
            Self::ComplianceRegimeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of ListWorkloads endpoint."]
    pub struct GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next page token. Return empty if reached the last page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workloads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Workloads under a given parent."]
        pub workloads: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssuredworkloadsV1Workload>>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1ListWorkloadsResponseBuilder {
            GoogleCloudAssuredworkloadsV1ListWorkloadsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Workload object for managing highly regulated workloads of cloud customers."]
    pub struct GoogleCloudAssuredworkloadsV1Workload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
        pub billing_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceRegime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
        pub compliance_regime:
            ::std::option::Option<GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Immutable. The Workload creation timestamp."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Settings used to create a CMEK crypto key. When set a project with a KMS CMEK key is provisioned. This field is mandatory for a subset of Compliance Regimes."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1WorkloadKmsSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Labels applied to the workload."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisionedResourcesParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. The parent resource for the resources managed by this Assured Workload. May be either an organization or a folder. Must be the same or a child of the Workload parent. If not specified all resources are created under the Workload parent. Formats: folders/{folder_id} organizations/{organization_id}"]
        pub provisioned_resources_parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
        pub resources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssuredworkloadsV1WorkloadResourceInfo>>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1Workload {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1WorkloadBuilder {
            GoogleCloudAssuredworkloadsV1WorkloadBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
        #[serde(rename = "COMPLIANCE_REGIME_UNSPECIFIED")]
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[serde(rename = "IL4")]
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[serde(rename = "CJIS")]
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[serde(rename = "FEDRAMP_HIGH")]
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[serde(rename = "FEDRAMP_MODERATE")]
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[serde(rename = "US_REGIONAL_ACCESS")]
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl ::std::default::Default for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegimeEnum {
        fn default() -> Self {
            Self::ComplianceRegimeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to the Key Management Service."]
    pub struct GoogleCloudAssuredworkloadsV1WorkloadKmsSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextRotationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
        pub next_rotation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotationPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
        pub rotation_period: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadKmsSettings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1WorkloadKmsSettingsBuilder {
            GoogleCloudAssuredworkloadsV1WorkloadKmsSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represent the resources that are children of this Workload."]
    pub struct GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource identifier. For a project this represents project_number."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the type of resource."]
        pub resource_type: ::std::option::Option<
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1WorkloadResourceInfoBuilder {
            GoogleCloudAssuredworkloadsV1WorkloadResourceInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the type of resource."]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
        #[serde(rename = "CONSUMER_PROJECT")]
        #[doc = "Consumer project."]
        ConsumerProject,
        #[serde(rename = "ENCRYPTION_KEYS_PROJECT")]
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
    }
    impl ::std::default::Default for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceTypeEnum {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Operation metadata to give request details of CreateWorkload."]
    pub struct GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceRegime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."]
        pub compliance_regime: ::std::option::Option<
            GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadataComplianceRegimeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Time when the operation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The display name of the workload."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The parent of the workload."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadata {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadataBuilder
        {
            GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."]
    pub enum GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadataComplianceRegimeEnum {
        #[serde(rename = "COMPLIANCE_REGIME_UNSPECIFIED")]
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[serde(rename = "IL4")]
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[serde(rename = "CJIS")]
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[serde(rename = "FEDRAMP_HIGH")]
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[serde(rename = "FEDRAMP_MODERATE")]
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[serde(rename = "US_REGIONAL_ACCESS")]
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl ::std::default::Default
        for GoogleCloudAssuredworkloadsV1beta1CreateWorkloadOperationMetadataComplianceRegimeEnum
    {
        fn default() -> Self {
            Self::ComplianceRegimeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Workload object for managing highly regulated workloads of cloud customers."]
    pub struct GoogleCloudAssuredworkloadsV1beta1Workload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
        pub billing_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cjisSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings specific to resources needed for CJIS."]
        pub cjis_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadCjisSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceRegime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
        pub compliance_regime:
            ::std::option::Option<GoogleCloudAssuredworkloadsV1beta1WorkloadComplianceRegimeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Immutable. The Workload creation timestamp."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fedrampHighSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings specific to resources needed for FedRAMP High."]
        pub fedramp_high_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampHighSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fedrampModerateSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate."]
        pub fedramp_moderate_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampModerateSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "il4Settings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings specific to resources needed for IL4."]
        pub il4_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadIl4Settings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Settings used to create a CMEK crypto key. When set a project with a KMS CMEK key is provisioned. This field is mandatory for a subset of Compliance Regimes."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Labels applied to the workload."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provisionedResourcesParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. The parent resource for the resources managed by this Assured Workload. May be either an organization or a folder. Must be the same or a child of the Workload parent. If not specified all resources are created under the Workload parent. Formats: folders/{folder_id} organizations/{organization_id}"]
        pub provisioned_resources_parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
        pub resources: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfo>,
            >,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1Workload {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
    pub enum GoogleCloudAssuredworkloadsV1beta1WorkloadComplianceRegimeEnum {
        #[serde(rename = "COMPLIANCE_REGIME_UNSPECIFIED")]
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[serde(rename = "IL4")]
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[serde(rename = "CJIS")]
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[serde(rename = "FEDRAMP_HIGH")]
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[serde(rename = "FEDRAMP_MODERATE")]
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[serde(rename = "US_REGIONAL_ACCESS")]
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl ::std::default::Default for GoogleCloudAssuredworkloadsV1beta1WorkloadComplianceRegimeEnum {
        fn default() -> Self {
            Self::ComplianceRegimeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to resources needed for CJIS."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadCjisSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings used to create a CMEK crypto key."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadCjisSettings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadCjisSettingsBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadCjisSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to resources needed for FedRAMP High."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampHighSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings used to create a CMEK crypto key."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampHighSettings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampHighSettingsBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampHighSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to resources needed for FedRAMP Moderate."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampModerateSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings used to create a CMEK crypto key."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampModerateSettings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampModerateSettingsBuilder
        {
            GoogleCloudAssuredworkloadsV1beta1WorkloadFedrampModerateSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to resources needed for IL4."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadIl4Settings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. Settings used to create a CMEK crypto key."]
        pub kms_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings>,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadIl4Settings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadIl4SettingsBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadIl4SettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to the Key Management Service."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextRotationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
        pub next_rotation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotationPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
        pub rotation_period: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettings {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettingsBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadKmsSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represent the resources that are children of this Workload."]
    pub struct GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource identifier. For a project this represents project_number."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the type of resource."]
        pub resource_type: ::std::option::Option<
            GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfoResourceTypeEnum,
        >,
    }
    impl GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfo {
        pub fn builder() -> GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfoBuilder {
            GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the type of resource."]
    pub enum GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfoResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
        #[serde(rename = "CONSUMER_PROJECT")]
        #[doc = "Consumer project."]
        ConsumerProject,
        #[serde(rename = "ENCRYPTION_KEYS_PROJECT")]
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
    }
    impl ::std::default::Default
        for GoogleCloudAssuredworkloadsV1beta1WorkloadResourceInfoResourceTypeEnum
    {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
}
