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
    pub mod customers {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of customers to show in a page of results. A number between 1 and 100 (inclusive)."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A token specifying which result page to return."]
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
            pub mod configurations {
                pub mod methods {
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
                            #[doc = "Required. The field mask applied to the target `Configuration` before updating the fields. To learn more about using field masks, read [FieldMask](/protocol-buffers/docs/reference/google.protobuf#fieldmask) in the Protocol Buffers documentation."]
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
            pub mod devices {
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
                            #[doc = "The maximum number of devices to show in a page of results. Must be between 1 and 100 inclusive."]
                            pub page_size: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token specifying which result page to return."]
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
    pub mod partners {
        pub mod resources {
            pub mod customers {
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
                            #[doc = "The maximum number of results to be returned. If not specified or 0, all the records are returned."]
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
                }
            }
            pub mod vendors {
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
                            #[doc = "The maximum number of results to be returned."]
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
                }
                pub mod resources {
                    pub mod customers {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of results to be returned."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results returned by the server."]
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
    #[doc = "Request message to claim a device on behalf of a customer."]
    pub struct ClaimDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the customer for whom the device is being claimed."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Required. The device identifier of the device to claim."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The metadata to attach to the device."]
        pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The section type of the device's provisioning record."]
        pub section_type: ::std::option::Option<ClaimDeviceRequestSectionTypeEnum>,
    }
    impl ClaimDeviceRequest {
        pub fn builder() -> ClaimDeviceRequestBuilder {
            ClaimDeviceRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The section type of the device's provisioning record."]
    pub enum ClaimDeviceRequestSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for ClaimDeviceRequestSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message containing device id of the claim."]
    pub struct ClaimDeviceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device ID of the claimed device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the device in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`."]
        pub device_name: ::std::option::Option<::std::string::String>,
    }
    impl ClaimDeviceResponse {
        pub fn builder() -> ClaimDeviceResponseBuilder {
            ClaimDeviceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to claim devices asynchronously in batch. Claiming a device adds the device to zero-touch enrollment and shows the device in the customer's view of the portal."]
    pub struct ClaimDevicesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of device claims."]
        pub claims: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartnerClaim>>>,
    }
    impl ClaimDevicesRequest {
        pub fn builder() -> ClaimDevicesRequestBuilder {
            ClaimDevicesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reseller, vendor, or customer in the zero-touch reseller and customer APIs."]
    pub struct Company {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adminEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Email address of customer's users in the admin role. Each email address must be associated with a Google Account."]
        pub admin_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ID of the company. Assigned by the server."]
        pub company_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the company. For example _XYZ Corp_. Displayed to the company's employees in the zero-touch enrollment portal."]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The API resource name of the company. The resource name is one of the following formats: * `partners/[PARTNER_ID]/customers/[CUSTOMER_ID]` * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]` * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]/customers/[CUSTOMER_ID]` Assigned by the server."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Email address of customer's users in the owner role. At least one `owner_email` is required. Each email address must be associated with a Google Account. Owners share the same access as admins but can also add, delete, and edit your organization's portal users."]
        pub owner_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "termsStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether any user from the company has accepted the latest Terms of Service (ToS). See TermsStatus."]
        pub terms_status: ::std::option::Option<CompanyTermsStatusEnum>,
    }
    impl Company {
        pub fn builder() -> CompanyBuilder {
            CompanyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Whether any user from the company has accepted the latest Terms of Service (ToS). See TermsStatus."]
    pub enum CompanyTermsStatusEnum {
        #[serde(rename = "TERMS_STATUS_UNSPECIFIED")]
        #[doc = "Default value. This value should never be set if the enum is present."]
        TermsStatusUnspecified,
        #[serde(rename = "TERMS_STATUS_NOT_ACCEPTED")]
        #[doc = "None of the company's users have accepted the ToS."]
        TermsStatusNotAccepted,
        #[serde(rename = "TERMS_STATUS_ACCEPTED")]
        #[doc = "One (or more) of the company's users has accepted the ToS."]
        TermsStatusAccepted,
        #[serde(rename = "TERMS_STATUS_STALE")]
        #[doc = "None of the company's users has accepted the current ToS but at least one user accepted a previous ToS."]
        TermsStatusStale,
    }
    impl ::std::default::Default for CompanyTermsStatusEnum {
        fn default() -> Self {
            Self::TermsStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A configuration collects the provisioning options for Android devices. Each configuration combines the following: * The EMM device policy controller (DPC) installed on the devices. * EMM policies enforced on the devices. * Metadata displayed on the device to help users during setup. Customers can add as many configurations as they need. However, zero-touch enrollment works best when a customer sets a default configuration that's applied to any new devices the organization purchases."]
    pub struct Configuration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the organization. Zero-touch enrollment shows this organization name to device users during device provisioning."]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ID of the configuration. Assigned by the server."]
        pub configuration_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A short name that describes the configuration's purpose. For example, _Sales team_ or _Temporary employees_. The zero-touch enrollment portal displays this name to IT admins."]
        pub configuration_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The email address that device users can contact to get help. Zero-touch enrollment shows this email address to device users before device provisioning. The value is validated on input."]
        pub contact_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactPhone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The telephone number that device users can call, using another device, to get help. Zero-touch enrollment shows this number to device users before device provisioning. Accepts numerals, spaces, the plus sign, hyphens, and parentheses."]
        pub contact_phone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message, containing one or two sentences, to help device users get help or give them more details about what’s happening to their device. Zero-touch enrollment shows this message before the device is provisioned."]
        pub custom_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dpcExtras")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The JSON-formatted EMM provisioning extras that are passed to the DPC."]
        pub dpc_extras: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dpcResourcePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the selected DPC (device policy controller) in the format `customers/[CUSTOMER_ID]/dpcs/*`. To list the supported DPCs, call `customers.dpcs.list`."]
        pub dpc_resource_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Whether this is the default configuration that zero-touch enrollment applies to any new devices the organization purchases in the future. Only one customer configuration can be the default. Setting this value to `true`, changes the previous default configuration's `isDefault` value to `false`."]
        pub is_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by the server."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Configuration {
        pub fn builder() -> ConfigurationBuilder {
            ConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message to create a customer."]
    pub struct CreateCustomerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The company data to populate the new customer. Must contain a value for `companyName` and at least one `owner_email` that's associated with a Google Account. The values for `companyId` and `name` must be empty."]
        pub customer: ::std::option::Option<::std::boxed::Box<Company>>,
    }
    impl CreateCustomerRequest {
        pub fn builder() -> CreateCustomerRequestBuilder {
            CreateCustomerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for customer to assign a configuration to device."]
    pub struct CustomerApplyConfigurationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The configuration applied to the device in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`."]
        pub configuration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device the configuration is applied to."]
        pub device: ::std::option::Option<::std::boxed::Box<DeviceReference>>,
    }
    impl CustomerApplyConfigurationRequest {
        pub fn builder() -> CustomerApplyConfigurationRequestBuilder {
            CustomerApplyConfigurationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message of customer's listing configuration."]
    pub struct CustomerListConfigurationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configurations."]
        pub configurations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Configuration>>>,
    }
    impl CustomerListConfigurationsResponse {
        pub fn builder() -> CustomerListConfigurationsResponseBuilder {
            CustomerListConfigurationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing my customers."]
    pub struct CustomerListCustomersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer accounts the calling user is a member of."]
        pub customers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Company>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to access the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CustomerListCustomersResponse {
        pub fn builder() -> CustomerListCustomersResponseBuilder {
            CustomerListCustomersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message of customer's liting devices."]
    pub struct CustomerListDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's devices."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to access the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CustomerListDevicesResponse {
        pub fn builder() -> CustomerListDevicesResponseBuilder {
            CustomerListDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message of customer's listing DPCs."]
    pub struct CustomerListDpcsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dpcs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of DPCs available to the customer that support zero-touch enrollment."]
        pub dpcs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dpc>>>,
    }
    impl CustomerListDpcsResponse {
        pub fn builder() -> CustomerListDpcsResponseBuilder {
            CustomerListDpcsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for customer to remove the configuration from device."]
    pub struct CustomerRemoveConfigurationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device to remove the configuration from."]
        pub device: ::std::option::Option<::std::boxed::Box<DeviceReference>>,
    }
    impl CustomerRemoveConfigurationRequest {
        pub fn builder() -> CustomerRemoveConfigurationRequestBuilder {
            CustomerRemoveConfigurationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for customer to unclaim a device."]
    pub struct CustomerUnclaimDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device to unclaim."]
        pub device: ::std::option::Option<::std::boxed::Box<DeviceReference>>,
    }
    impl CustomerUnclaimDeviceRequest {
        pub fn builder() -> CustomerUnclaimDeviceRequestBuilder {
            CustomerUnclaimDeviceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Android device registered for zero-touch enrollment."]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The provisioning claims for a device. Devices claimed for zero-touch enrollment have a claim with the type `SECTION_TYPE_ZERO_TOUCH`. Call `partners.devices.unclaim` or `partners.devices.unclaimAsync` to remove the device from zero-touch enrollment."]
        pub claims: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceClaim>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not available to resellers."]
        pub configuration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ID of the device. Assigned by the server."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hardware IDs that identify a manufactured device. To learn more, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers)."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata attached to the device. Structured as key-value pairs. To learn more, read [Device metadata](https://developers.google.com/zero-touch/guides/metadata)."]
        pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The API resource name in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`. Assigned by the server."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A record of a device claimed by a reseller for a customer. Devices claimed for zero-touch enrollment have a claim with the type `SECTION_TYPE_ZERO_TOUCH`. To learn more, read [Claim devices for customers](/zero-touch/guides/how-it-works#claim)."]
    pub struct DeviceClaim {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerCompanyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Customer that purchased the device."]
        pub owner_company_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resellerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the reseller that claimed the device."]
        pub reseller_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of claim made on the device."]
        pub section_type: ::std::option::Option<DeviceClaimSectionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the device will exit ‘vacation mode’. This value is present iff the device is in 'vacation mode'."]
        pub vacation_mode_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the device was put into ‘vacation mode’. This value is present iff the device is in 'vacation mode'."]
        pub vacation_mode_start_time: ::std::option::Option<::std::string::String>,
    }
    impl DeviceClaim {
        pub fn builder() -> DeviceClaimBuilder {
            DeviceClaimBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The type of claim made on the device."]
    pub enum DeviceClaimSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for DeviceClaimSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates hardware and product IDs to identify a manufactured device. To understand requirements on identifier sets, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers)."]
    pub struct DeviceIdentifier {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imei")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device’s IMEI number. Validated on input."]
        pub imei: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device manufacturer’s name. Matches the device's built-in value returned from `android.os.Build.MANUFACTURER`. Allowed values are listed in [manufacturers](/zero-touch/resources/manufacturer-names#manufacturers-names)."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device’s MEID number."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device model's name. Matches the device's built-in value returned from `android.os.Build.MODEL`. Allowed values are listed in [models](/zero-touch/resources/manufacturer-names#model-names)."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The manufacturer's serial number for the device. This value might not be unique across different device models."]
        pub serial_number: ::std::option::Option<::std::string::String>,
    }
    impl DeviceIdentifier {
        pub fn builder() -> DeviceIdentifierBuilder {
            DeviceIdentifierBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata entries that can be attached to a `Device`. To learn more, read [Device metadata](https://developers.google.com/zero-touch/guides/metadata)."]
    pub struct DeviceMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata entries recorded as key-value pairs."]
        pub entries:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl DeviceMetadata {
        pub fn builder() -> DeviceMetadataBuilder {
            DeviceMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `DeviceReference` is an API abstraction that lets you supply a _device_ argument to a method using one of the following identifier types: * A numeric API resource ID. * Real-world hardware IDs, such as IMEI number, belonging to the manufactured device. Methods that operate on devices take a `DeviceReference` as a parameter type because it's more flexible for the caller. To learn more about device identifiers, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers)."]
    pub struct DeviceReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hardware IDs of the device."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
    }
    impl DeviceReference {
        pub fn builder() -> DeviceReferenceBuilder {
            DeviceReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tracks the status of a long-running operation to asynchronously update a batch of reseller metadata attached to devices. To learn more, read [Long‑running batch operations](/zero-touch/guides/how-it-works#operations)."]
    pub struct DevicesLongRunningOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devicesCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of metadata updates in the operation. This might be different from the number of updates in the request if the API can't parse some of the updates."]
        pub devices_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processing status of the operation."]
        pub processing_status:
            ::std::option::Option<DevicesLongRunningOperationMetadataProcessingStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processing progress of the operation. Measured as a number from 0 to 100. A value of 10O doesnt always mean the operation completed—check for the inclusion of a `done` field."]
        pub progress: ::std::option::Option<::std::primitive::i64>,
    }
    impl DevicesLongRunningOperationMetadata {
        pub fn builder() -> DevicesLongRunningOperationMetadataBuilder {
            DevicesLongRunningOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The processing status of the operation."]
    pub enum DevicesLongRunningOperationMetadataProcessingStatusEnum {
        #[serde(rename = "BATCH_PROCESS_STATUS_UNSPECIFIED")]
        #[doc = "Invalid code. Shouldn't be used."]
        BatchProcessStatusUnspecified,
        #[serde(rename = "BATCH_PROCESS_PENDING")]
        #[doc = "Pending."]
        BatchProcessPending,
        #[serde(rename = "BATCH_PROCESS_IN_PROGRESS")]
        #[doc = "In progress."]
        BatchProcessInProgress,
        #[serde(rename = "BATCH_PROCESS_PROCESSED")]
        #[doc = "Processed. This doesn't mean all items were processed successfully, you should check the `response` field for the result of every item."]
        BatchProcessProcessed,
    }
    impl ::std::default::Default for DevicesLongRunningOperationMetadataProcessingStatusEnum {
        fn default() -> Self {
            Self::BatchProcessStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tracks the status of a long-running operation to claim, unclaim, or attach metadata to devices. To learn more, read [Long‑running batch operations](/zero-touch/guides/how-it-works#operations)."]
    pub struct DevicesLongRunningOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perDeviceStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processing status for each device in the operation. One `PerDeviceStatus` per device. The list order matches the items in the original request."]
        pub per_device_status:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperationPerDevice>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A summary of how many items in the operation the server processed successfully. Updated as the operation progresses."]
        pub success_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl DevicesLongRunningOperationResponse {
        pub fn builder() -> DevicesLongRunningOperationResponseBuilder {
            DevicesLongRunningOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An EMM's DPC ([device policy controller](http://developer.android.com/work/dpc/build-dpc.html)). Zero-touch enrollment installs a DPC (listed in the `Configuration`) on a device to maintain the customer's mobile policies. All the DPCs listed by the API support zero-touch enrollment and are available in Google Play."]
    pub struct Dpc {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dpcName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The title of the DPC app in Google Play. For example, _Google Apps Device Policy_. Useful in an application's user interface."]
        pub dpc_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The API resource name in the format `customers/[CUSTOMER_ID]/dpcs/[DPC_ID]`. Assigned by the server. To maintain a reference to a DPC across customer accounts, persist and match the last path component (`DPC_ID`)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The DPC's Android application ID that looks like a Java package name. Zero-touch enrollment installs the DPC app onto a device using this identifier."]
        pub package_name: ::std::option::Option<::std::string::String>,
    }
    impl Dpc {
        pub fn builder() -> DpcBuilder {
            DpcBuilder::default()
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
    #[doc = "Request to find devices."]
    pub struct FindDevicesByDeviceIdentifierRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Required. The device identifier to search for."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The maximum number of devices to show in a page of results. Must be between 1 and 100 inclusive."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token specifying which result page to return."]
        pub page_token: ::std::option::Option<::std::string::String>,
    }
    impl FindDevicesByDeviceIdentifierRequest {
        pub fn builder() -> FindDevicesByDeviceIdentifierRequestBuilder {
            FindDevicesByDeviceIdentifierRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing found devices."]
    pub struct FindDevicesByDeviceIdentifierResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Found devices."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to access the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of items in the list irrespective of pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl FindDevicesByDeviceIdentifierResponse {
        pub fn builder() -> FindDevicesByDeviceIdentifierResponseBuilder {
            FindDevicesByDeviceIdentifierResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to find devices by customers."]
    pub struct FindDevicesByOwnerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of customer IDs to search for."]
        pub customer_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The maximum number of devices to show in a page of results. Must be between 1 and 100 inclusive."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token specifying which result page to return."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The section type of the device's provisioning record."]
        pub section_type: ::std::option::Option<FindDevicesByOwnerRequestSectionTypeEnum>,
    }
    impl FindDevicesByOwnerRequest {
        pub fn builder() -> FindDevicesByOwnerRequestBuilder {
            FindDevicesByOwnerRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The section type of the device's provisioning record."]
    pub enum FindDevicesByOwnerRequestSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for FindDevicesByOwnerRequestSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing found devices."]
    pub struct FindDevicesByOwnerResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's devices."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to access the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of items in the list irrespective of pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl FindDevicesByOwnerResponse {
        pub fn builder() -> FindDevicesByOwnerResponseBuilder {
            FindDevicesByOwnerResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message of all customers related to this partner."]
    pub struct ListCustomersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of customers related to this reseller partner."]
        pub customers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Company>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of items in the list irrespective of pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListCustomersResponse {
        pub fn builder() -> ListCustomersResponseBuilder {
            ListCustomersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message to list customers of the vendor."]
    pub struct ListVendorCustomersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of customers of the vendor."]
        pub customers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Company>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of items in the list irrespective of pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListVendorCustomersResponse {
        pub fn builder() -> ListVendorCustomersResponseBuilder {
            ListVendorCustomersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message to list vendors of the partner."]
    pub struct ListVendorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Omitted if no further results are available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of items in the list irrespective of pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vendors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of vendors of the reseller partner. Fields `name`, `companyId` and `companyName` are populated to the Company object."]
        pub vendors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Company>>>,
    }
    impl ListVendorsResponse {
        pub fn builder() -> ListVendorsResponseBuilder {
            ListVendorsResponseBuilder::default()
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
        #[doc = "This field will always be not set if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`. In this case, error information for each device is set in `response.perDeviceStatus.result.status`."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field will contain a `DevicesLongRunningOperationMetadata` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`."]
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
        #[doc = "This field will contain a `DevicesLongRunningOperationResponse` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`."]
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
    #[doc = "A task for each device in the operation. Corresponds to each device change in the request."]
    pub struct OperationPerDevice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claim")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A copy of the original device-claim request received by the server."]
        pub claim: ::std::option::Option<::std::boxed::Box<PartnerClaim>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processing result for each device."]
        pub result: ::std::option::Option<::std::boxed::Box<PerDeviceStatusInBatch>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unclaim")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A copy of the original device-unclaim request received by the server."]
        pub unclaim: ::std::option::Option<::std::boxed::Box<PartnerUnclaim>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A copy of the original metadata-update request received by the server."]
        pub update_metadata: ::std::option::Option<::std::boxed::Box<UpdateMetadataArguments>>,
    }
    impl OperationPerDevice {
        pub fn builder() -> OperationPerDeviceBuilder {
            OperationPerDeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies one claim request."]
    pub struct PartnerClaim {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the customer for whom the device is being claimed."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Required. Device identifier of the device."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The metadata to attach to the device at claim."]
        pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The section type of the device's provisioning record."]
        pub section_type: ::std::option::Option<PartnerClaimSectionTypeEnum>,
    }
    impl PartnerClaim {
        pub fn builder() -> PartnerClaimBuilder {
            PartnerClaimBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The section type of the device's provisioning record."]
    pub enum PartnerClaimSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for PartnerClaimSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies one unclaim request."]
    pub struct PartnerUnclaim {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Device ID of the device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Device identifier of the device."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The section type of the device's provisioning record."]
        pub section_type: ::std::option::Option<PartnerUnclaimSectionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The duration of the vacation unlock starting from when the request is processed. (1 day is treated as 24 hours)"]
        pub vacation_mode_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The expiration time of the vacation unlock."]
        pub vacation_mode_expire_time: ::std::option::Option<::std::string::String>,
    }
    impl PartnerUnclaim {
        pub fn builder() -> PartnerUnclaimBuilder {
            PartnerUnclaimBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The section type of the device's provisioning record."]
    pub enum PartnerUnclaimSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for PartnerUnclaimSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Captures the processing status for each device in the operation."]
    pub struct PerDeviceStatusInBatch {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If processing succeeds, the device ID of the device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If processing fails, the error type."]
        pub error_identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If processing fails, a developer message explaining what went wrong."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result status of the device after processing."]
        pub status: ::std::option::Option<PerDeviceStatusInBatchStatusEnum>,
    }
    impl PerDeviceStatusInBatch {
        pub fn builder() -> PerDeviceStatusInBatchBuilder {
            PerDeviceStatusInBatchBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The result status of the device after processing."]
    pub enum PerDeviceStatusInBatchStatusEnum {
        #[serde(rename = "SINGLE_DEVICE_STATUS_UNSPECIFIED")]
        #[doc = "Invalid code. Shouldn't be used."]
        SingleDeviceStatusUnspecified,
        #[serde(rename = "SINGLE_DEVICE_STATUS_UNKNOWN_ERROR")]
        #[doc = "Unknown error. We don't expect this error to occur here."]
        SingleDeviceStatusUnknownError,
        #[serde(rename = "SINGLE_DEVICE_STATUS_OTHER_ERROR")]
        #[doc = "Other error. We know/expect this error, but there's no defined error code for the error."]
        SingleDeviceStatusOtherError,
        #[serde(rename = "SINGLE_DEVICE_STATUS_SUCCESS")]
        #[doc = "Success."]
        SingleDeviceStatusSuccess,
        #[serde(rename = "SINGLE_DEVICE_STATUS_PERMISSION_DENIED")]
        #[doc = "Permission denied."]
        SingleDeviceStatusPermissionDenied,
        #[serde(rename = "SINGLE_DEVICE_STATUS_INVALID_DEVICE_IDENTIFIER")]
        #[doc = "Invalid device identifier."]
        SingleDeviceStatusInvalidDeviceIdentifier,
        #[serde(rename = "SINGLE_DEVICE_STATUS_INVALID_SECTION_TYPE")]
        #[doc = "Invalid section type."]
        SingleDeviceStatusInvalidSectionType,
        #[serde(rename = "SINGLE_DEVICE_STATUS_SECTION_NOT_YOURS")]
        #[doc = "This section is claimed by another company."]
        SingleDeviceStatusSectionNotYours,
    }
    impl ::std::default::Default for PerDeviceStatusInBatchStatusEnum {
        fn default() -> Self {
            Self::SingleDeviceStatusUnspecified
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
    #[doc = "Request message to unclaim a device."]
    pub struct UnclaimDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device ID returned by `ClaimDevice`."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device identifier you used when you claimed this device."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sectionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The section type of the device's provisioning record."]
        pub section_type: ::std::option::Option<UnclaimDeviceRequestSectionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration of the vacation unlock starting from when the request is processed. (1 day is treated as 24 hours)"]
        pub vacation_mode_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vacationModeExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expiration time of the vacation unlock."]
        pub vacation_mode_expire_time: ::std::option::Option<::std::string::String>,
    }
    impl UnclaimDeviceRequest {
        pub fn builder() -> UnclaimDeviceRequestBuilder {
            UnclaimDeviceRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The section type of the device's provisioning record."]
    pub enum UnclaimDeviceRequestSectionTypeEnum {
        #[serde(rename = "SECTION_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified section type."]
        SectionTypeUnspecified,
        #[serde(rename = "SECTION_TYPE_SIM_LOCK")]
        #[doc = "SIM-lock section type."]
        SectionTypeSimLock,
        #[serde(rename = "SECTION_TYPE_ZERO_TOUCH")]
        #[doc = "Zero-touch enrollment section type."]
        SectionTypeZeroTouch,
    }
    impl ::std::default::Default for UnclaimDeviceRequestSectionTypeEnum {
        fn default() -> Self {
            Self::SectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to unclaim devices asynchronously in batch."]
    pub struct UnclaimDevicesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unclaims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of devices to unclaim."]
        pub unclaims: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartnerUnclaim>>>,
    }
    impl UnclaimDevicesRequest {
        pub fn builder() -> UnclaimDevicesRequestBuilder {
            UnclaimDevicesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to update device metadata in batch."]
    pub struct UpdateDeviceMetadataInBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of metadata updates."]
        pub updates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdateMetadataArguments>>>,
    }
    impl UpdateDeviceMetadataInBatchRequest {
        pub fn builder() -> UpdateDeviceMetadataInBatchRequestBuilder {
            UpdateDeviceMetadataInBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to set metadata for a device."]
    pub struct UpdateDeviceMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The metadata to attach to the device."]
        pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
    }
    impl UpdateDeviceMetadataRequest {
        pub fn builder() -> UpdateDeviceMetadataRequestBuilder {
            UpdateDeviceMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies metadata updates to one device."]
    pub struct UpdateMetadataArguments {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Device ID of the device."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Device identifier."]
        pub device_identifier: ::std::option::Option<::std::boxed::Box<DeviceIdentifier>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The metadata to update."]
        pub device_metadata: ::std::option::Option<::std::boxed::Box<DeviceMetadata>>,
    }
    impl UpdateMetadataArguments {
        pub fn builder() -> UpdateMetadataArgumentsBuilder {
            UpdateMetadataArgumentsBuilder::default()
        }
    }
}
